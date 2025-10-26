mod core;
mod runners;
mod components;
mod pages;
mod utils;

use core::run_workflow;
use std::env;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // User provided a workflow file argument
        let workflow_path = &args[1];
        let full_path = resolve_workflow_path(workflow_path);
        
        println!("=== Running workflow: {} ===", full_path);
        run_workflow(&full_path)?;
    } else {
        // Default behavior: run all demo workflows
        println!("=== Running hybrid workflow (Python + Lua) ===");
        run_workflow("workflows/hybrid_workflow.lua")?;
        
        println!("\n=== Running pure Lua workflow ===");
        run_workflow("workflows/workflow.lua")?;
        
        println!("\n=== Running shell workflow (Shell + Python) ===");
        run_workflow("workflows/shell_workflow.lua")?;
        
        println!("\n=== Running pure shell workflow ===");
        run_workflow("workflows/pure_shell_workflow.lua")?;
        
        println!("\n=== Running comprehensive multi-language workflow ===");
        run_workflow("workflows/comprehensive_workflow.lua")?;
        
        println!("\n=== Running JavaScript workflow ===");
        run_workflow("workflows/javascript_workflow.lua")?;
        
        println!("\n=== Running JavaScript integration workflow (Python + JavaScript + Shell) ===");
        run_workflow("workflows/js_python_shell_workflow.lua")?;
    }
    
    Ok(())
}

/// Resolves workflow path to always look in workflows/ folder or subfolders
fn resolve_workflow_path(path: &str) -> String {
    // If path already starts with workflows/, use as-is
    if path.starts_with("workflows/") {
        return path.to_string();
    }
    
    // If it's just a filename or relative path, prepend workflows/
    if !path.contains('/') || !Path::new(path).exists() {
        let workflow_path = format!("workflows/{}", path);
        
        // Check if the file exists in workflows/
        if Path::new(&workflow_path).exists() {
            return workflow_path;
        }
        
        // Also check common subfolders
        let subfolders = ["examples", "templates", "tests"];
        for subfolder in &subfolders {
            let subfolder_path = format!("workflows/{}/{}", subfolder, path);
            if Path::new(&subfolder_path).exists() {
                return subfolder_path;
            }
        }
        
        // Return the workflows/ path even if it doesn't exist (let run_workflow handle the error)
        return workflow_path;
    }
    
    // If it's an absolute path or relative path that exists, use as-is
    path.to_string()
}

#[cfg(test)]
mod tests {
    use crate::core::run_workflow;
    use std::fs;

    #[test]
    fn test_hybrid_workflow_execution() {
        let result = run_workflow("workflows/hybrid_workflow.lua");
        assert!(result.is_ok(), "Hybrid workflow should execute successfully");
    }

    #[test]
    fn test_pure_lua_workflow_execution() {
        let result = run_workflow("workflows/workflow.lua");
        assert!(result.is_ok(), "Pure Lua workflow should execute successfully");
    }

    #[test]
    fn test_nonexistent_workflow_file() {
        let result = run_workflow("workflows/nonexistent.lua");
        assert!(result.is_err(), "Should fail for nonexistent file");
    }

    #[test]
    fn test_create_and_run_simple_lua_workflow() {
        // Create a temporary simple workflow
        let test_workflow_content = r#"
workflow = {
  name = "test_simple",
  description = "Simple test workflow",
  steps = {
    test_step = {
      run = function()
        return { result = "test_passed", value = 42 }
      end
    }
  }
}
"#;

        let test_file = "workflows/test_temp_simple.lua";
        fs::write(test_file, test_workflow_content).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        assert!(result.is_ok(), "Simple Lua workflow should execute successfully");
    }

    #[test]
    fn test_create_and_run_python_workflow() {
        // Create a temporary Python workflow
        let test_workflow_content = r#"
workflow = {
  name = "test_python",
  description = "Simple Python test workflow",
  steps = {
    python_step = {
      language = "python",
      code = [[
def run():
    return {"result": "python_test_passed", "calculation": 10 * 5}
]]
    }
  }
}
"#;

        let test_file = "workflows/test_temp_python.lua";
        fs::write(test_file, test_workflow_content).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        assert!(result.is_ok(), "Simple Python workflow should execute successfully");
    }

    #[test]
    fn test_workflow_with_dependencies() {
        // Create a workflow with step dependencies
        let test_workflow_content = r#"
workflow = {
  name = "test_dependencies",
  description = "Test workflow with dependencies",
  steps = {
    first_step = {
      language = "python",
      code = [[
def run():
    return {"data": [1, 2, 3]}
]]
    },
    second_step = {
      depends_on = {"first_step"},
      language = "python",
      code = [[
def run(inputs):
    data = inputs["first_step"]["data"]
    return {"processed": [x * 2 for x in data]}
]]
    }
  }
}
"#;

        let test_file = "workflows/test_temp_deps.lua";
        fs::write(test_file, test_workflow_content).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        assert!(result.is_ok(), "Workflow with dependencies should execute successfully");
    }

    #[test]
    fn test_invalid_workflow_structure() {
        // Create an invalid workflow (missing steps)
        let invalid_workflow_content = r#"
workflow = {
  name = "invalid_test",
  description = "Invalid workflow missing steps"
}
"#;

        let test_file = "workflows/test_temp_invalid.lua";
        fs::write(test_file, invalid_workflow_content).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        assert!(result.is_err(), "Invalid workflow should fail");
    }

    #[test]
    fn test_mixed_lua_python_workflow() {
        // Create a mixed Lua and Python workflow
        let mixed_workflow_content = r#"
workflow = {
  name = "test_mixed",
  description = "Mixed Lua and Python workflow",
  steps = {
    lua_step = {
      run = function()
        return { message = "Hello from Lua", number = 42 }
      end
    },
    python_step = {
      depends_on = {"lua_step"},
      language = "python",
      code = [[
def run(inputs):
    lua_msg = inputs["lua_step"]["message"]
    lua_num = inputs["lua_step"]["number"]
    return {"combined": f"{lua_msg} -> Python doubled: {lua_num * 2}"}
]]
    }
  }
}
"#;

        let test_file = "workflows/test_temp_mixed.lua";
        fs::write(test_file, mixed_workflow_content).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        assert!(result.is_ok(), "Mixed Lua-Python workflow should execute successfully");
    }
}
