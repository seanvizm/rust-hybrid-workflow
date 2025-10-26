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
