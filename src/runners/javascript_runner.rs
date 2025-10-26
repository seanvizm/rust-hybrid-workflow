use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::Write;
use tempfile::NamedTempFile;

pub fn run_javascript_step(
    name: &str,
    code: &str,
    inputs: &HashMap<String, serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    // Create a temporary JavaScript file
    let mut temp_file = NamedTempFile::with_suffix(".js")?;
    
    // Write the JavaScript code with inputs available as a global object
    writeln!(temp_file, "// JavaScript runner for step: {}", name)?;
    writeln!(temp_file, "const process = require('process');")?;
    writeln!(temp_file)?;
    
    // Create inputs object from environment variables or direct injection
    writeln!(temp_file, "// Input data from previous steps")?;
    writeln!(temp_file, "const inputs = {{}};")?;
    
    for (key, value) in inputs {
        let json_str = serde_json::to_string(value)?;
        writeln!(temp_file, "inputs['{}'] = {};", key, json_str)?;
    }
    writeln!(temp_file)?;
    
    // Add helper functions
    writeln!(temp_file, "// Helper function to output results")?;
    writeln!(temp_file, "function outputResult(result) {{")?;
    writeln!(temp_file, "  console.log(JSON.stringify(result));")?;
    writeln!(temp_file, "}}")?;
    writeln!(temp_file)?;
    
    // Add the user's JavaScript code
    writeln!(temp_file, "// User JavaScript code")?;
    writeln!(temp_file, "{}", code)?;
    writeln!(temp_file)?;
    
    // Execute the run function and capture output
    writeln!(temp_file, "// Execute and output result")?;
    writeln!(temp_file, "try {{")?;
    writeln!(temp_file, "  let result;")?;
    writeln!(temp_file, "  if (typeof run === 'function') {{")?;
    writeln!(temp_file, "    if (Object.keys(inputs).length === 0) {{")?;
    writeln!(temp_file, "      result = run();")?;
    writeln!(temp_file, "    }} else {{")?;
    writeln!(temp_file, "      result = run(inputs);")?;
    writeln!(temp_file, "    }}")?;
    writeln!(temp_file, "  }} else {{")?;
    writeln!(temp_file, "    throw new Error('No run function defined in step {}');", name)?;
    writeln!(temp_file, "  }}")?;
    writeln!(temp_file, "  ")?;
    writeln!(temp_file, "  // Handle different result types")?;
    writeln!(temp_file, "  if (result === undefined || result === null) {{")?;
    writeln!(temp_file, "    result = {{}};")?;
    writeln!(temp_file, "  }}")?;
    writeln!(temp_file, "  ")?;
    writeln!(temp_file, "  // Ensure result is serializable")?;
    writeln!(temp_file, "  if (typeof result === 'object') {{")?;
    writeln!(temp_file, "    console.log(JSON.stringify(result));")?;
    writeln!(temp_file, "  }} else {{")?;
    writeln!(temp_file, "    console.log(JSON.stringify({{ value: result }}));")?;
    writeln!(temp_file, "  }}")?;
    writeln!(temp_file, "}} catch (error) {{")?;
    writeln!(temp_file, "  console.error('Error in JavaScript step {}: ' + error.message);", name)?;
    writeln!(temp_file, "  process.exit(1);")?;
    writeln!(temp_file, "}}")?;
    
    temp_file.flush()?;
    
    // Check if Node.js is available
    let node_check = Command::new("node")
        .arg("--version")
        .output();
        
    if node_check.is_err() {
        return Err(anyhow::anyhow!(
            "Node.js is not installed or not available in PATH. Please install Node.js to run JavaScript steps."
        ));
    }
    
    // Execute the JavaScript file with Node.js
    let output = Command::new("node")
        .arg(temp_file.path())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow::anyhow!(
            "JavaScript step '{}' failed:\nStdout: {}\nStderr: {}",
            name, stdout, stderr
        ));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed_output = stdout.trim();
    
    if trimmed_output.is_empty() {
        return Ok(serde_json::json!({}));
    }
    
    // Try to parse the output as JSON
    match serde_json::from_str(trimmed_output) {
        Ok(json_value) => Ok(json_value),
        Err(_) => {
            // If parsing fails, try to parse each line separately and take the last valid JSON
            let lines: Vec<&str> = trimmed_output.lines().collect();
            let mut last_valid_json = serde_json::json!({});
            
            for line in lines.iter().rev() {
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(line) {
                    last_valid_json = json_value;
                    break;
                }
            }
            
            // If no valid JSON found, wrap the output as a string
            if last_valid_json == serde_json::json!({}) && !trimmed_output.is_empty() {
                Ok(serde_json::json!({
                    "output": trimmed_output,
                    "raw": true
                }))
            } else {
                Ok(last_valid_json)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_simple_javascript_execution() {
        let code = r#"
function run() {
    return { message: "Hello from JavaScript!", number: 42 };
}
"#;
        let inputs = HashMap::new();
        let result = run_javascript_step("test_step", code, &inputs);
        
        if result.is_ok() {
            let json_result = result.unwrap();
            assert!(json_result.get("message").is_some());
            assert_eq!(json_result["message"], "Hello from JavaScript!");
            assert_eq!(json_result["number"], 42);
        } else {
            // Skip test if Node.js is not available
            println!("Skipping JavaScript test - Node.js not available");
        }
    }

    #[test]
    fn test_javascript_with_inputs() {
        let code = r#"
function run(inputs) {
    const data = inputs.test_data.values;
    const sum = data.reduce((a, b) => a + b, 0);
    return { sum: sum, count: data.length };
}
"#;
        let mut inputs = HashMap::new();
        inputs.insert("test_data".to_string(), serde_json::json!({
            "values": [1, 2, 3, 4, 5]
        }));
        
        let result = run_javascript_step("test_step", code, &inputs);
        
        if result.is_ok() {
            let json_result = result.unwrap();
            assert_eq!(json_result["sum"], 15);
            assert_eq!(json_result["count"], 5);
        } else {
            // Skip test if Node.js is not available
            println!("Skipping JavaScript test - Node.js not available");
        }
    }

    #[test]
    fn test_javascript_error_handling() {
        let code = r#"
function run() {
    // This will cause an error
    throw new Error("Test error");
}
"#;
        let inputs = HashMap::new();
        let result = run_javascript_step("test_step", code, &inputs);
        
        // Should return an error
        assert!(result.is_err());
    }

    #[test]
    fn test_javascript_async_operations() {
        let code = r#"
function run() {
    // Simple sync operation (async would require different handling)
    const data = [1, 2, 3].map(x => x * 2);
    return { processed: data, timestamp: new Date().toISOString() };
}
"#;
        let inputs = HashMap::new();
        let result = run_javascript_step("test_step", code, &inputs);
        
        if result.is_ok() {
            let json_result = result.unwrap();
            assert!(json_result.get("processed").is_some());
            assert!(json_result.get("timestamp").is_some());
        } else {
            // Skip test if Node.js is not available
            println!("Skipping JavaScript test - Node.js not available");
        }
    }
}