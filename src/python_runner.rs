use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::ffi::CString;

pub fn run_python_step(
    name: &str,
    code: &str,
    inputs: &HashMap<String, serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        
        // Convert inputs HashMap to Python dict using Python's json module
        let inputs_dict = PyDict::new(py);
        
        // Import Python's json module
        let json_module = py.import("json")?;
        
        for (key, value) in inputs {
            // Convert serde_json::Value to JSON string and then parse with Python's json module
            let json_str = serde_json::to_string(value)?;
            // Debug: println!("Converting {} -> {} for step '{}'", key, json_str, name);
            let py_value = json_module.call_method1("loads", (json_str,))?;
            inputs_dict.set_item(key, py_value)?;
        }
        
        locals.set_item("inputs", &inputs_dict)?;
        
        // Convert code string to CString for py.run
        let code_cstring = CString::new(code)?;
        py.run(&code_cstring, None, Some(&locals))?;

        let run_func = locals.get_item("run")?;
        let result = match run_func {
            Some(func) => {
                if func.is_callable() {
                    if inputs.is_empty() {
                        func.call0()?
                    } else {
                        func.call1((&inputs_dict,))?
                    }
                } else {
                    return Err(anyhow::anyhow!("'run' is not callable in step {}", name));
                }
            }
            None => {
                return Err(anyhow::anyhow!("No 'run' function found in step {}", name));
            }
        };

        // Convert Python result back to JSON using Python's json module
        let json_str = json_module.call_method1("dumps", (result,))?;
        let json_string: String = json_str.extract()?;
        let json: serde_json::Value = serde_json::from_str(&json_string)
            .unwrap_or_else(|_| serde_json::Value::String(json_string));
        
        Ok(json)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_run_python_step_no_inputs() {
        let code = r#"
def run():
    return {"result": "success", "value": 42}
"#;
        let inputs = HashMap::new();
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.is_object());
        
        if let Some(result_val) = output.get("result") {
            assert_eq!(result_val.as_str().unwrap(), "success");
        }
        if let Some(value_val) = output.get("value") {
            assert_eq!(value_val.as_i64().unwrap(), 42);
        }
    }

    #[test]
    fn test_run_python_step_with_inputs() {
        let code = r#"
def run(inputs):
    input_data = inputs["test_input"]["data"]
    return {"doubled": [x * 2 for x in input_data]}
"#;
        let mut inputs = HashMap::new();
        let input_data = serde_json::json!({"data": [1, 2, 3]});
        inputs.insert("test_input".to_string(), input_data);
        
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        if let Some(doubled) = output.get("doubled") {
            let expected = serde_json::json!([2, 4, 6]);
            assert_eq!(doubled, &expected);
        }
    }

    #[test]
    fn test_run_python_step_syntax_error() {
        let code = r#"
def run():
    return {"result": "success"  # Missing closing brace
"#;
        let inputs = HashMap::new();
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_run_python_step_no_run_function() {
        let code = r#"
def other_function():
    return {"result": "success"}
"#;
        let inputs = HashMap::new();
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_run_python_step_runtime_error() {
        let code = r#"
def run():
    return 1 / 0  # Division by zero
"#;
        let inputs = HashMap::new();
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_run_python_step_complex_data_types() {
        let code = r#"
def run():
    return {
        "string": "hello",
        "number": 3.14,
        "boolean": True,
        "null": None,
        "array": [1, 2, 3],
        "object": {"nested": "value"}
    }
"#;
        let inputs = HashMap::new();
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        
        assert_eq!(output.get("string").unwrap().as_str().unwrap(), "hello");
        assert_eq!(output.get("number").unwrap().as_f64().unwrap(), 3.14);
        assert_eq!(output.get("boolean").unwrap().as_bool().unwrap(), true);
        assert!(output.get("null").unwrap().is_null());
        assert!(output.get("array").unwrap().is_array());
        assert!(output.get("object").unwrap().is_object());
    }

    #[test]
    fn test_run_python_step_with_complex_inputs() {
        let code = r#"
def run(inputs):
    data = inputs["complex_data"]
    return {
        "string_length": len(data["text"]),
        "array_sum": sum(data["numbers"]),
        "nested_value": data["nested"]["value"]
    }
"#;
        let mut inputs = HashMap::new();
        let complex_data = serde_json::json!({
            "text": "hello world",
            "numbers": [1, 2, 3, 4, 5],
            "nested": {"value": "found"}
        });
        inputs.insert("complex_data".to_string(), complex_data);
        
        let result = run_python_step("test_step", code, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.get("string_length").unwrap().as_i64().unwrap(), 11);
        assert_eq!(output.get("array_sum").unwrap().as_i64().unwrap(), 15);
        assert_eq!(output.get("nested_value").unwrap().as_str().unwrap(), "found");
    }
}