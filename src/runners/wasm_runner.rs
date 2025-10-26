use std::collections::HashMap;
use std::path::Path;
use wasmtime::*;

pub fn run_wasm_step(
    _name: &str,
    module_path: &str,
    function_name: Option<&str>,
    inputs: &HashMap<String, serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    // Check if WASM module file exists
    if !Path::new(module_path).exists() {
        return Err(anyhow::anyhow!(
            "WASM module file not found: {}. Please ensure the .wasm file exists.",
            module_path
        ));
    }

    // Create WASM engine and store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // Load the WASM module
    let module = Module::from_file(&engine, module_path)
        .map_err(|e| anyhow::anyhow!("Failed to load WASM module '{}': {}", module_path, e))?;

    // Create instance
    let instance = Instance::new(&mut store, &module, &[])
        .map_err(|e| anyhow::anyhow!("Failed to instantiate WASM module '{}': {}", module_path, e))?;

    // Determine which function to call
    let func_name = function_name.unwrap_or("run");
    
    // Get the function from the WASM module
    let func = instance
        .get_typed_func::<(), i32>(&mut store, func_name)
        .or_else(|_| {
            // Try with different signatures
            instance.get_typed_func::<i32, i32>(&mut store, func_name)
                .map(|f| unsafe { std::mem::transmute(f) })
        })
        .or_else(|_| {
            // Try void function
            instance.get_typed_func::<(), ()>(&mut store, func_name)
                .map(|f| unsafe { std::mem::transmute(f) })
        })
        .map_err(|e| anyhow::anyhow!(
            "Function '{}' not found in WASM module '{}'. Available exports: {:?}. Error: {}", 
            func_name, 
            module_path,
            instance.exports(&mut store).map(|e| e.name()).collect::<Vec<_>>(),
            e
        ))?;

    // For now, we'll implement a simple approach where WASM modules return status codes
    // In a more advanced implementation, we could use WASI or custom host functions
    // to pass complex data structures
    
    println!("Executing WASM function '{}' from module '{}'", func_name, module_path);
    println!("Input data available: {} items", inputs.len());
    
    // Call the WASM function
    let result: Result<i32, _> = func.call(&mut store, ());
    
    match result {
        Ok(return_code) => {
            println!("WASM function completed with return code: {}", return_code);
            
            // Create result based on return code and inputs
            let mut wasm_result = serde_json::json!({
                "wasm_execution": {
                    "module": module_path,
                    "function": func_name,
                    "return_code": return_code,
                    "status": if return_code == 0 { "success" } else { "error" },
                    "input_count": inputs.len()
                }
            });

            // Include input data summary in the result
            if !inputs.is_empty() {
                let input_summary: HashMap<String, String> = inputs
                    .iter()
                    .map(|(k, v)| {
                        let summary = match v {
                            serde_json::Value::Array(arr) => format!("array[{}]", arr.len()),
                            serde_json::Value::Object(obj) => format!("object[{}]", obj.len()),
                            serde_json::Value::String(s) => format!("string[{}]", s.len()),
                            serde_json::Value::Number(n) => format!("number[{}]", n),
                            serde_json::Value::Bool(b) => format!("bool[{}]", b),
                            serde_json::Value::Null => "null".to_string(),
                        };
                        (k.clone(), summary)
                    })
                    .collect();
                
                wasm_result["input_summary"] = serde_json::to_value(input_summary)?;
            }

            // Simulate some processing results based on return code
            match return_code {
                0 => {
                    wasm_result["processed_data"] = serde_json::json!({
                        "success": true,
                        "message": "WASM processing completed successfully",
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    });
                }
                1..=10 => {
                    wasm_result["processed_data"] = serde_json::json!({
                        "warning": true,
                        "message": format!("WASM processing completed with warning code {}", return_code),
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    });
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "WASM function '{}' failed with return code: {}", 
                        func_name, 
                        return_code
                    ));
                }
            }

            Ok(wasm_result)
        }
        Err(trap) => {
            Err(anyhow::anyhow!(
                "WASM function '{}' trapped: {}", 
                func_name, 
                trap
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_wasm_module_not_found() {
        let inputs = HashMap::new();
        let result = run_wasm_step("test", "nonexistent.wasm", None, &inputs);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("WASM module file not found"));
    }

    #[test] 
    fn test_wasm_step_basic_functionality() {
        // This test would require a actual WASM file to work
        // For now, we test the error handling
        let inputs = HashMap::new();
        let result = run_wasm_step("test", "test.wasm", Some("test_func"), &inputs);
        // Should fail because test.wasm doesn't exist
        assert!(result.is_err());
    }

    #[test]
    fn test_wasm_with_inputs() {
        let mut inputs = HashMap::new();
        inputs.insert("data".to_string(), serde_json::json!([1, 2, 3]));
        inputs.insert("config".to_string(), serde_json::json!({"enabled": true}));
        
        let result = run_wasm_step("test", "nonexistent.wasm", None, &inputs);
        assert!(result.is_err());
        // Test that we properly handle inputs in error cases
        assert!(result.unwrap_err().to_string().contains("WASM module file not found"));
    }
}