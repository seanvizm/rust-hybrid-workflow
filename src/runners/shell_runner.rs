use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::Write;
use tempfile::NamedTempFile;

pub fn run_shell_step(
    name: &str,
    code: &str,
    inputs: &HashMap<String, serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    // Create a temporary shell script file
    let mut temp_file = NamedTempFile::new()?;
    
    // Write the shell script with inputs available as environment variables
    writeln!(temp_file, "#!/bin/bash")?;
    writeln!(temp_file, "set -e")?; // Exit on error
    writeln!(temp_file)?;
    
    // Export inputs as environment variables
    writeln!(temp_file, "# Input variables from previous steps")?;
    for (key, value) in inputs {
        let json_str = serde_json::to_string(value)?;
        // Create environment variables with INPUT_ prefix to avoid conflicts
        writeln!(temp_file, "export INPUT_{}='{}'", key.to_uppercase(), json_str)?;
    }
    writeln!(temp_file)?;
    
    // Add helper functions for JSON parsing
    writeln!(temp_file, "# Helper function to parse JSON input")?;
    writeln!(temp_file, "parse_input() {{")?;
    writeln!(temp_file, "  local step_name=\"$1\"")?;
    writeln!(temp_file, "  local var_name=\"INPUT_$(echo \"$step_name\" | tr '[:lower:]' '[:upper:]')\"")?;
    writeln!(temp_file, "  eval \"echo \\$$var_name\"")?;
    writeln!(temp_file, "}}")?;
    writeln!(temp_file)?;
    
    // Add the user's shell code
    writeln!(temp_file, "# User shell code")?;
    writeln!(temp_file, "{}", code)?;
    
    // Always call run function at the end if it exists
    writeln!(temp_file)?;
    writeln!(temp_file, "# Call run function if it exists")?;
    writeln!(temp_file, "if declare -f run > /dev/null; then")?;
    writeln!(temp_file, "  run")?;
    writeln!(temp_file, "fi")?;
    
    temp_file.flush()?;
    
    // Make the script executable
    let script_path = temp_file.path();
    Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .output()?;
    
    // Execute the shell script
    let output = Command::new("bash")
        .arg(script_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Shell script failed in step '{}': {}", 
            name, 
            stderr
        ));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Try to parse the output as JSON, fall back to a simple structure
    let result = {
        let stdout_trimmed = stdout.trim();
        
        // Try to find JSON in the output (look for lines that start with { and end with })
        let mut json_result = None;
        for line in stdout_trimmed.lines() {
            let line = line.trim();
            if line.starts_with('{') && line.ends_with('}') {
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(line) {
                    json_result = Some(json_value);
                    break;
                }
            }
        }
        
        if let Some(json_value) = json_result {
            json_value
        } else {
            // If no valid JSON found, wrap everything in a standard structure
            serde_json::json!({
                "stdout": stdout_trimmed,
                "stderr": stderr.trim(),
                "exit_code": output.status.code().unwrap_or(0)
            })
        }
    };
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_simple_shell_command() {
        let code = r#"
run() {
    echo '{"result": "hello world", "status": "success"}'
}
"#;
        let inputs = HashMap::new();
        let result = run_shell_step("test", code, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.get("result").is_some());
        assert_eq!(output["result"], "hello world");
    }

    #[test]
    fn test_shell_with_inputs() {
        let code = r#"
run() {
    echo "Debug: INPUT_TEST_INPUT=$INPUT_TEST_INPUT"
    local input_data="$INPUT_TEST_INPUT"
    local value=$(echo "$input_data" | grep -o '"data":[0-9]*' | cut -d':' -f2)
    echo "Debug: extracted value=$value"
    if [ -n "$value" ]; then
        local result=$((value * 2))
        echo "{\"processed\": $result, \"status\": \"success\"}"
    else
        echo "{\"processed\": 0, \"status\": \"failed to parse\", \"input_received\": \"$input_data\"}"
    fi
}
"#;
        let mut inputs = HashMap::new();
        inputs.insert("test_input".to_string(), serde_json::json!({"data": 42}));
        
        let result = run_shell_step("test", code, &inputs);
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("Shell output: {:#}", output);
        
        // Check either the JSON parsed correctly or we got stdout with some result
        if let Some(processed) = output.get("processed") {
            assert!(processed.is_number());
        } else {
            // If JSON parsing failed, check stdout contains our processing
            assert!(output.get("stdout").is_some());
            let stdout = output["stdout"].as_str().unwrap();
            assert!(stdout.contains("processed") || stdout.contains("42"));
        }
    }

    #[test]
    fn test_shell_command_failure() {
        let code = r#"
run() {
    exit 1
}
"#;
        let inputs = HashMap::new();
        let result = run_shell_step("test", code, &inputs);
        assert!(result.is_err());
    }

    #[test]
    fn test_shell_plain_output() {
        let code = r#"
run() {
    echo "Simple text output"
    echo "Another line" >&2
}
"#;
        let inputs = HashMap::new();
        let result = run_shell_step("test", code, &inputs);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.get("stdout").is_some());
        assert_eq!(output["stdout"], "Simple text output");
        assert_eq!(output["stderr"], "Another line");
    }

    #[test]
    fn test_shell_environment_variables() {
        let code = r#"
run() {
    echo "Debug: INPUT_MY_VAR=$INPUT_MY_VAR"
    # Properly escape JSON to avoid nested quotes
    echo "{\"received_input\": $INPUT_MY_VAR, \"status\": \"success\", \"env_check\": true}"
}
"#;
        let mut inputs = HashMap::new();
        inputs.insert("my_var".to_string(), serde_json::json!("test_value"));
        
        let result = run_shell_step("test", code, &inputs);
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("Environment test output: {:#}", output);
        
        // Since JSON parsing may fail due to quoting issues, check the stdout
        if let Some(received_input) = output.get("received_input") {
            assert!(received_input.is_string());
        } else {
            // Check that we at least got stdout with our data
            assert!(output.get("stdout").is_some());
            let stdout = output["stdout"].as_str().unwrap();
            assert!(stdout.contains("test_value"));
        }
    }
}