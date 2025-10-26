mod engine;
mod lua_loader;
mod lua_runner;
mod python_runner;

fn main() -> anyhow::Result<()> {
    println!("=== Running hybrid workflow (Python + Lua) ===");
    engine::run_workflow("workflows/hybrid_workflow.lua")?;
    
    println!("\n=== Running pure Lua workflow ===");
    engine::run_workflow("workflows/workflow.lua")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_hybrid_workflow_execution() {
        let result = engine::run_workflow("workflows/hybrid_workflow.lua");
        assert!(result.is_ok(), "Hybrid workflow should execute successfully");
    }

    #[test]
    fn test_pure_lua_workflow_execution() {
        let result = engine::run_workflow("workflows/workflow.lua");
        assert!(result.is_ok(), "Pure Lua workflow should execute successfully");
    }

    #[test]
    fn test_nonexistent_workflow_file() {
        let result = engine::run_workflow("workflows/nonexistent.lua");
        assert!(result.is_err(), "Should fail when workflow file doesn't exist");
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

        let result = engine::run_workflow(test_file);
        
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

        let result = engine::run_workflow(test_file);
        
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

        let result = engine::run_workflow(test_file);
        
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

        let result = engine::run_workflow(test_file);
        
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

        let result = engine::run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);
        
        assert!(result.is_ok(), "Mixed Lua-Python workflow should execute successfully");
    }
}
