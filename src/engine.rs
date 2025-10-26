use crate::lua_loader::{load_workflow, Step};
use crate::lua_runner::run_lua_step;
use crate::python_runner::run_python_step;
use mlua::Lua;
use std::collections::{HashMap, HashSet};

pub fn run_workflow(path: &str) -> anyhow::Result<()> {
    let mut steps = load_workflow(path)?;
    let mut results: HashMap<String, serde_json::Value> = HashMap::new();

    // Sort steps by dependencies (topological sort)
    steps = sort_steps_by_dependencies(steps)?;

    // Initialize Lua context for Lua steps
    let lua = Lua::new();
    let workflow_table = if steps.iter().any(|s| s.language == "lua") {
        let script = std::fs::read_to_string(path)?;
        lua.load(&script).exec()?;
        Some(lua.globals().get("workflow")?)
    } else {
        None
    };

    for step in &steps {
        let mut inputs = HashMap::new();
        for dep in &step.depends_on {
            if let Some(val) = results.get(dep) {
                inputs.insert(dep.clone(), val.clone());
            }
        }

        let output = match step.language.as_str() {
            "python" => run_python_step(&step.name, &step.code, &inputs)?,
            "lua" => {
                if let Some(ref workflow) = workflow_table {
                    run_lua_step(&step.name, &lua, workflow, &inputs)?
                } else {
                    return Err(anyhow::anyhow!("Lua workflow context not available"));
                }
            }
            _ => return Err(anyhow::anyhow!("Unsupported language: {}", step.language)),
        };

        println!("Step '{}' output: {}", step.name, output);
        results.insert(step.name.clone(), output);
    }

    Ok(())
}

// Simple topological sort for step dependencies
fn sort_steps_by_dependencies(steps: Vec<Step>) -> anyhow::Result<Vec<Step>> {
    let mut sorted = Vec::new();
    let mut remaining: HashMap<String, Step> = steps.into_iter().map(|s| (s.name.clone(), s)).collect();
    let mut processed: HashSet<String> = HashSet::new();
    
    while !remaining.is_empty() {
        let mut progress = false;
        let mut to_remove = Vec::new();
        
        for (name, step) in &remaining {
            // Check if all dependencies are satisfied
            let can_process = step.depends_on.iter().all(|dep| processed.contains(dep));
            
            if can_process {
                sorted.push(step.clone());
                processed.insert(name.clone());
                to_remove.push(name.clone());
                progress = true;
            }
        }
        
        for name in to_remove {
            remaining.remove(&name);
        }
        
        if !progress {
            return Err(anyhow::anyhow!("Circular dependency detected in workflow steps"));
        }
    }
    
    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sort_steps_no_dependencies() {
        let steps = vec![
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
            },
        ];

        let result = sort_steps_by_dependencies(steps);
        assert!(result.is_ok());
        let sorted = result.unwrap();
        assert_eq!(sorted.len(), 2);
    }

    #[test]
    fn test_sort_steps_with_dependencies() {
        let steps = vec![
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string()],
            },
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
            },
        ];

        let result = sort_steps_by_dependencies(steps);
        assert!(result.is_ok());
        let sorted = result.unwrap();
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].name, "step1");
        assert_eq!(sorted[1].name, "step2");
    }

    #[test]
    fn test_sort_steps_circular_dependency() {
        let steps = vec![
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step2".to_string()],
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string()],
            },
        ];

        let result = sort_steps_by_dependencies(steps);
        assert!(result.is_err());
    }

    #[test]
    fn test_sort_steps_complex_dependencies() {
        let steps = vec![
            Step {
                name: "step3".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string(), "step2".to_string()],
            },
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string()],
            },
        ];

        let result = sort_steps_by_dependencies(steps);
        assert!(result.is_ok());
        let sorted = result.unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].name, "step1");
        assert_eq!(sorted[1].name, "step2");
        assert_eq!(sorted[2].name, "step3");
    }

    #[test]
    fn test_run_workflow_integration() {
        // Create a simple test workflow file
        let test_workflow = r#"
workflow = {
  name = "integration_test",
  description = "Integration test workflow",
  steps = {
    test_step = {
      run = function()
        return { status = "completed", test = true }
      end
    }
  }
}
"#;
        let test_file = "workflows/test_integration.lua";
        fs::write(test_file, test_workflow).expect("Should write test file");

        let result = run_workflow(test_file);
        
        // Cleanup
        let _ = fs::remove_file(test_file);

        assert!(result.is_ok(), "Integration test workflow should run successfully");
    }
}
