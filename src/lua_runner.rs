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