mod core;
mod runners;
mod config;

use core::run_workflow;
use config::AppConfig;
use std::env;
use std::path::Path;
use std::fs;

fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = AppConfig::load()?;
    
    println!("Loaded configuration:");
    println!("  Workflow directory: {}", config.workflows.directory.display());
    println!("  Server: {}:{}", config.server.host, config.server.port);
    println!("  Log level: {}", config.logging.level);
    println!();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // User provided a workflow file argument
        let workflow_filename = &args[1];
        let full_path = resolve_workflow_path(workflow_filename, &config);
        
        println!("=== Running workflow: {} ===", workflow_filename);
        run_workflow(&full_path)?;
    } else {
        // Default behavior: run all workflows found in the workflows directory
        let workflow_files = discover_workflow_files(&config.workflows.directory.to_string_lossy(), &config)?;
        
        if workflow_files.is_empty() {
            println!("No workflow files found in {} directory", config.workflows.directory.display());
            return Ok(());
        }
        
        println!("Found {} workflow files. Running all workflows...\n", workflow_files.len());
        
        for (index, workflow_path) in workflow_files.iter().enumerate() {
            if index > 0 {
                println!(); // Add spacing between workflows
            }
            
            let workflow_info = get_workflow_info(workflow_path)?;
            println!("=== Running workflow {}/{}: {} ===", 
                index + 1, 
                workflow_files.len(),
                workflow_info.display_name
            );
            
            if let Some(description) = workflow_info.description {
                println!("Description: {}", description);
            }
            
            match run_workflow(workflow_path) {
                Ok(_) => println!("✅ Workflow '{}' completed successfully", workflow_info.name),
                Err(e) => {
                    println!("❌ Workflow '{}' failed: {}", workflow_info.name, e);
                    // Continue with other workflows instead of stopping
                }
            }
        }
    }
    
    Ok(())
}

/// Resolves workflow path to always look in workflows/ folder or subfolders
fn resolve_workflow_path(path: &str, config: &AppConfig) -> String {
    let workflow_dir = config.workflows.directory.to_string_lossy();
    
    // If path already starts with workflows/, use as-is
    if path.starts_with(&workflow_dir.to_string()) {
        return path.to_string();
    }
    
    // If it's just a filename or relative path, prepend workflows/
    if !path.contains('/') || !Path::new(path).exists() {
        let workflow_path = format!("{}/{}", workflow_dir, path);
        
        // Check if the file exists in workflows/
        if Path::new(&workflow_path).exists() {
            return workflow_path;
        }
        
        // Also check common subfolders
        let subfolders = ["examples", "templates", "tests"];
        for subfolder in &subfolders {
            let subfolder_path = format!("{}/{}/{}", workflow_dir, subfolder, path);
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

/// Discovers all workflow files in the specified directory
fn discover_workflow_files(dir: &str, config: &AppConfig) -> anyhow::Result<Vec<String>> {
    let mut workflow_files = Vec::new();
    
    if !Path::new(dir).exists() {
        return Ok(workflow_files);
    }
    
    let entries = fs::read_dir(dir)?;
    let max_workflows = config.workflows.max_workflows;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                // Check if extension is in configured list
                let ext_str = extension.to_string_lossy();
                if config.workflows.extensions.iter().any(|e| e == &ext_str.to_string()) {
                    if let Some(path_str) = path.to_str() {
                        // Skip temporary test files
                        if !path_str.contains("test_temp_") {
                            workflow_files.push(path_str.to_string());
                            
                            // Respect max_workflows limit
                            if workflow_files.len() >= max_workflows {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort for consistent ordering
    workflow_files.sort();
    Ok(workflow_files)
}

/// Workflow information extracted from the file
#[derive(Debug)]
struct WorkflowInfo {
    name: String,
    description: Option<String>,
    display_name: String,
}

/// Extracts workflow name and description from a workflow file
fn get_workflow_info(workflow_path: &str) -> anyhow::Result<WorkflowInfo> {
    use mlua::Lua;
    
    let lua = Lua::new();
    let workflow_content = fs::read_to_string(workflow_path)?;
    
    // Execute the Lua file to get the workflow table
    lua.load(&workflow_content).exec()?;
    
    // Get the workflow table
    let workflow_table: mlua::Table = lua.globals().get("workflow")?;
    
    let name: String = workflow_table.get("name").unwrap_or_else(|_| {
        // Fallback to filename if name not found
        Path::new(workflow_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    });
    
    let description: Option<String> = workflow_table.get("description").ok();
    
    // Create a display name from the filename for better readability
    let display_name = Path::new(workflow_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(workflow_path)
        .to_string();
    
    Ok(WorkflowInfo {
        name,
        description,
        display_name,
    })
}

#[cfg(test)]
mod tests {
    use crate::core::run_workflow;
    use crate::{discover_workflow_files, get_workflow_info};
    use crate::config::AppConfig;
    use std::fs;

    #[test]
    fn test_all_existing_workflows() {
        let config = AppConfig::default();
        let workflow_files = discover_workflow_files("workflows", &config)
            .expect("Should be able to discover workflow files");
        
        assert!(!workflow_files.is_empty(), "Should find at least one workflow file");
        
        for workflow_path in workflow_files {
            let workflow_info = get_workflow_info(&workflow_path)
                .expect(&format!("Should be able to get info for {}", workflow_path));
            
            println!("Testing workflow: {} ({})", workflow_info.name, workflow_info.display_name);
            
            let result = run_workflow(&workflow_path);
            assert!(result.is_ok(), 
                "Workflow '{}' ({}) should execute successfully: {:?}", 
                workflow_info.name, 
                workflow_path,
                result.err()
            );
        }
    }

    #[test]
    fn test_nonexistent_workflow_file() {
        let result = run_workflow("workflows/nonexistent.lua");
        assert!(result.is_err(), "Should fail for nonexistent file");
    }

    #[test]
    fn test_create_and_run_simple_lua_workflow() {
        // Create a temporary simple workflow using new format
        let test_workflow_content = r#"
workflow = {
  name = "test_simple",
  description = "Simple test workflow",
  steps = {
    test_step = {
      language = "lua",
      code = [[
function run()
    return { result = "test_passed", value = 42 }
end
]]
    }
  }
}
"#;

        let test_file = "workflows/test_temp_simple.lua";
        fs::write(test_file, test_workflow_content).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        if let Err(e) = &result {
            eprintln!("Simple Lua workflow failed with error: {:?}", e);
        }
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
        // Create a mixed Lua and Python workflow using new format
        let mixed_workflow_content = r#"
workflow = {
  name = "test_mixed",
  description = "Mixed Lua and Python workflow",
  steps = {
    lua_step = {
      language = "lua",
      code = [[
function run()
    return { message = "Hello from Lua", number = 42 }
end
]]
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
        
        if let Err(e) = &result {
            eprintln!("Mixed Lua-Python workflow failed with error: {:?}", e);
        }
        assert!(result.is_ok(), "Mixed Lua-Python workflow should execute successfully");
    }
}
