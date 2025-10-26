use mlua::{Lua, Value, Table};
use std::collections::HashMap;

pub fn run_lua_step(
    name: &str,
    lua: &Lua,
    workflow_table: &Table,
    inputs: &HashMap<String, serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    // Get the steps table
    let steps: Table = workflow_table.get("steps")?;
    let step: Table = steps.get(name)?;
    
    // Get the run function
    let run_func: mlua::Function = step.get("run")?;
    
    // Convert inputs to Lua table
    let inputs_table = lua.create_table()?;
    for (key, value) in inputs {
        let lua_value = json_to_lua(lua, value)?;
        inputs_table.set(key.as_str(), lua_value)?;
    }
    
    // Call the function
    let result = if inputs.is_empty() {
        run_func.call::<_, Value>(())?
    } else {
        run_func.call::<_, Value>(inputs_table)?
    };
    
    // Convert result back to JSON
    lua_to_json(&result)
}

// Helper function to convert serde_json::Value to Lua Value
fn json_to_lua<'lua>(lua: &'lua Lua, value: &serde_json::Value) -> mlua::Result<Value<'lua>> {
    match value {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Ok(Value::Integer(n.as_u64().unwrap_or(0) as i64))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                let lua_value = json_to_lua(lua, item)?;
                table.set(i + 1, lua_value)?; // Lua arrays are 1-based
            }
            Ok(Value::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj {
                let lua_value = json_to_lua(lua, val)?;
                table.set(key.as_str(), lua_value)?;
            }
            Ok(Value::Table(table))
        }
    }
}

// Helper function to convert Lua Value to serde_json::Value
fn lua_to_json(value: &Value) -> anyhow::Result<serde_json::Value> {
    match value {
        Value::Nil => Ok(serde_json::Value::Null),
        Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Integer(i) => Ok(serde_json::Value::Number((*i).into())),
        Value::Number(f) => {
            if let Some(n) = serde_json::Number::from_f64(*f) {
                Ok(serde_json::Value::Number(n))
            } else {
                Ok(serde_json::Value::Null)
            }
        }
        Value::String(s) => Ok(serde_json::Value::String(s.to_str()?.to_string())),
        Value::Table(table) => {
            // Try to determine if it's an array or object
            let mut is_array = true;
            let mut max_index = 0;
            
            for pair in table.clone().pairs::<Value, Value>() {
                let (key, _) = pair?;
                match key {
                    Value::Integer(i) if i > 0 => {
                        max_index = max_index.max(i as usize);
                    }
                    _ => {
                        is_array = false;
                        break;
                    }
                }
            }
            
            if is_array && max_index > 0 {
                // Convert to JSON array
                let mut arr = vec![serde_json::Value::Null; max_index];
                for pair in table.clone().pairs::<i64, Value>() {
                    let (key, value) = pair?;
                    if key > 0 && key <= max_index as i64 {
                        arr[(key - 1) as usize] = lua_to_json(&value)?;
                    }
                }
                Ok(serde_json::Value::Array(arr))
            } else {
                // Convert to JSON object
                let mut obj = serde_json::Map::new();
                for pair in table.clone().pairs::<String, Value>() {
                    let (key, value) = pair?;
                    obj.insert(key, lua_to_json(&value)?);
                }
                Ok(serde_json::Value::Object(obj))
            }
        }
        _ => Ok(serde_json::Value::String(format!("{:?}", value))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;
    use std::collections::HashMap;

    fn create_test_lua_context() -> Lua {
        let lua = Lua::new();
        let workflow_script = r#"
workflow = {
  name = "test_workflow",
  description = "Test workflow for unit tests",
  steps = {
    simple_step = {
      run = function()
        return { result = "success", value = 42 }
      end
    },
    input_step = {
      run = function(inputs)
        local data = inputs.test_input.data
        local doubled = {}
        for i, v in ipairs(data) do
          doubled[i] = v * 2
        end
        return { doubled = doubled }
      end
    }
  }
}
"#;
        lua.load(workflow_script).exec().unwrap();
        lua
    }

    #[test]
    fn test_run_lua_step_no_inputs() {
        let lua = create_test_lua_context();
        let workflow_table: Table = lua.globals().get("workflow").unwrap();
        let inputs = HashMap::new();
        
        let result = run_lua_step("simple_step", &lua, &workflow_table, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.get("result").unwrap().as_str().unwrap(), "success");
        assert_eq!(output.get("value").unwrap().as_i64().unwrap(), 42);
    }

    #[test]
    fn test_run_lua_step_with_inputs() {
        let lua = create_test_lua_context();
        let workflow_table: Table = lua.globals().get("workflow").unwrap();
        let mut inputs = HashMap::new();
        let input_data = serde_json::json!({"data": [1, 2, 3]});
        inputs.insert("test_input".to_string(), input_data);
        
        let result = run_lua_step("input_step", &lua, &workflow_table, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        if let Some(doubled) = output.get("doubled") {
            let expected = serde_json::json!([2, 4, 6]);
            assert_eq!(doubled, &expected);
        }
    }

    #[test]
    fn test_run_lua_step_nonexistent_step() {
        let lua = create_test_lua_context();
        let workflow_table: Table = lua.globals().get("workflow").unwrap();
        let inputs = HashMap::new();
        
        let result = run_lua_step("nonexistent_step", &lua, &workflow_table, &inputs);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_json_to_lua_conversion() {
        let lua = Lua::new();
        
        // Test null
        let null_val = serde_json::Value::Null;
        let lua_val = json_to_lua(&lua, &null_val).unwrap();
        assert!(matches!(lua_val, Value::Nil));
        
        // Test boolean
        let bool_val = serde_json::Value::Bool(true);
        let lua_val = json_to_lua(&lua, &bool_val).unwrap();
        assert!(matches!(lua_val, Value::Boolean(true)));
        
        // Test number
        let num_val = serde_json::Value::Number(42.into());
        let lua_val = json_to_lua(&lua, &num_val).unwrap();
        assert!(matches!(lua_val, Value::Integer(42)));
        
        // Test string
        let str_val = serde_json::Value::String("hello".to_string());
        let lua_val = json_to_lua(&lua, &str_val).unwrap();
        if let Value::String(s) = lua_val {
            assert_eq!(s.to_str().unwrap(), "hello");
        } else {
            panic!("Expected Lua string");
        }
    }

    #[test]
    fn test_lua_to_json_conversion() {
        let lua = Lua::new();
        
        // Test nil
        let nil_val = Value::Nil;
        let json_val = lua_to_json(&nil_val).unwrap();
        assert!(json_val.is_null());
        
        // Test boolean
        let bool_val = Value::Boolean(true);
        let json_val = lua_to_json(&bool_val).unwrap();
        assert_eq!(json_val, serde_json::Value::Bool(true));
        
        // Test integer
        let int_val = Value::Integer(42);
        let json_val = lua_to_json(&int_val).unwrap();
        assert_eq!(json_val, serde_json::Value::Number(42.into()));
        
        // Test string
        let str_val = Value::String(lua.create_string("hello").unwrap());
        let json_val = lua_to_json(&str_val).unwrap();
        assert_eq!(json_val, serde_json::Value::String("hello".to_string()));
    }

    #[test]
    fn test_lua_array_conversion() {
        let lua = Lua::new();
        
        // Create Lua array (1-based indexing)
        let table = lua.create_table().unwrap();
        table.set(1, "first").unwrap();
        table.set(2, "second").unwrap();
        table.set(3, "third").unwrap();
        
        let lua_val = Value::Table(table);
        let json_val = lua_to_json(&lua_val).unwrap();
        
        let expected = serde_json::json!(["first", "second", "third"]);
        assert_eq!(json_val, expected);
    }

    #[test]
    fn test_lua_object_conversion() {
        let lua = Lua::new();
        
        // Create Lua object (table with string keys)
        let table = lua.create_table().unwrap();
        table.set("name", "test").unwrap();
        table.set("value", 42).unwrap();
        
        let lua_val = Value::Table(table);
        let json_val = lua_to_json(&lua_val).unwrap();
        
        assert!(json_val.is_object());
        assert_eq!(json_val.get("name").unwrap().as_str().unwrap(), "test");
        assert_eq!(json_val.get("value").unwrap().as_i64().unwrap(), 42);
    }
}