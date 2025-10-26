use mlua::{Lua, Table};

#[derive(Clone)]
pub struct Step {
    pub name: String,
    pub language: String,
    pub code: String,
    pub depends_on: Vec<String>,
}

pub fn load_workflow(path: &str) -> anyhow::Result<Vec<Step>> {
    let lua = Lua::new();
    let script = std::fs::read_to_string(path)?;
    lua.load(&script).exec()?;

    let globals = lua.globals();
    let workflow: Table = globals.get("workflow")?;
    let steps: Table = workflow.get("steps")?;

    let mut result = vec![];

    for pair in steps.pairs::<String, Table>() {
        let (name, step) = pair?;
        
        // Default to "lua" if language is not specified
        let language: String = step.get("language").unwrap_or_else(|_| "lua".to_string());
        
        // For Lua steps, extract the function and convert to code string
        let code: String = if language == "lua" {
            // For pure Lua workflows, we need to handle the run function differently
            // For now, we'll create a placeholder - this will need more sophisticated handling
            format!("-- Lua function for step: {}", name)
        } else {
            step.get("code")?
        };
        
        let depends_on: Option<Vec<String>> = step.get("depends_on").ok();

        result.push(Step {
            name,
            language,
            code,
            depends_on: depends_on.unwrap_or_default(),
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_valid_lua_workflow() {
        let test_workflow = r#"
workflow = {
  name = "test_workflow",
  description = "Test workflow",
  steps = {
    test_step = {
      run = function()
        return { result = "success" }
      end
    }
  }
}
"#;
        let test_file = "workflows/test_lua_loader.lua";
        fs::write(test_file, test_workflow).expect("Should write test file");

        let result = load_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_ok());
        let steps = result.unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].name, "test_step");
        assert_eq!(steps[0].language, "lua");
        assert!(steps[0].depends_on.is_empty());
    }

    #[test]
    fn test_load_python_workflow() {
        let test_workflow = r#"
workflow = {
  name = "python_test",
  description = "Python test workflow",
  steps = {
    python_step = {
      language = "python",
      code = [[
def run():
    return {"result": "success"}
]]
    }
  }
}
"#;
        let test_file = "workflows/test_python_loader.lua";
        fs::write(test_file, test_workflow).expect("Should write test file");

        let result = load_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_ok());
        let steps = result.unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].name, "python_step");
        assert_eq!(steps[0].language, "python");
        assert!(steps[0].code.contains("def run():"));
    }

    #[test]
    fn test_load_workflow_with_dependencies() {
        let test_workflow = r#"
workflow = {
  name = "dependency_test",
  description = "Test workflow with dependencies",
  steps = {
    first = {
      run = function() return {data = 1} end
    },
    second = {
      depends_on = {"first"},
      run = function(inputs) return {result = inputs.first.data * 2} end
    }
  }
}
"#;
        let test_file = "workflows/test_deps_loader.lua";
        fs::write(test_file, test_workflow).expect("Should write test file");

        let result = load_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_ok());
        let steps = result.unwrap();
        assert_eq!(steps.len(), 2);
        
        // Find the steps (order might vary)
        let first_step = steps.iter().find(|s| s.name == "first").unwrap();
        let second_step = steps.iter().find(|s| s.name == "second").unwrap();
        
        assert!(first_step.depends_on.is_empty());
        assert_eq!(second_step.depends_on, vec!["first"]);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_workflow("workflows/nonexistent_file.lua");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_lua_syntax() {
        let invalid_workflow = r#"
workflow = {
  name = "invalid"
  -- missing comma above should cause syntax error
  description = "Invalid workflow"
}
"#;
        let test_file = "workflows/test_invalid_syntax.lua";
        fs::write(test_file, invalid_workflow).expect("Should write test file");

        let result = load_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_load_shell_workflow() {
        let test_workflow = r#"
workflow = {
  name = "shell_test",
  description = "Shell test workflow",
  steps = {
    shell_step = {
      language = "bash",
      code = [[
run() {
    echo "Hello from bash"
    echo '{"result": "success"}'
}
]]
    }
  }
}
"#;
        let test_file = "workflows/test_shell_loader.lua";
        fs::write(test_file, test_workflow).expect("Should write test file");

        let result = load_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_ok());
        let steps = result.unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].name, "shell_step");
        assert_eq!(steps[0].language, "bash");
        assert!(steps[0].code.contains("echo \"Hello from bash\""));
    }

    #[test]
    fn test_load_mixed_shell_python_workflow() {
        let test_workflow = r#"
workflow = {
  name = "mixed_shell_python_test",
  description = "Mixed shell and python workflow",
  steps = {
    shell_init = {
      language = "shell",
      code = [[
run() {
    echo '{"data": [1, 2, 3]}'
}
]]
    },
    python_process = {
      depends_on = {"shell_init"},
      language = "python",
      code = [[
def run(inputs):
    data = inputs["shell_init"]["data"]
    return {"processed": [x * 2 for x in data]}
]]
    }
  }
}
"#;
        let test_file = "workflows/test_mixed_shell_python.lua";
        fs::write(test_file, test_workflow).expect("Should write test file");

        let result = load_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_ok());
        let steps = result.unwrap();
        assert_eq!(steps.len(), 2);
        
        let shell_step = steps.iter().find(|s| s.name == "shell_init").unwrap();
        let python_step = steps.iter().find(|s| s.name == "python_process").unwrap();
        
        assert_eq!(shell_step.language, "shell");
        assert_eq!(python_step.language, "python");
        assert_eq!(python_step.depends_on, vec!["shell_init"]);
    }
}
