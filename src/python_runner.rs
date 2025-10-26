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
                    panic!("'run' is not callable in step {}", name);
                }
            }
            None => {
                panic!("No 'run' function found in step {}", name);
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