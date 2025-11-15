use crate::core::lua_loader::{load_workflow, Step};
use crate::runners::{run_lua_step, run_python_step, run_shell_step, run_javascript_step, run_wasm_step};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[cfg(feature = "cli")]
use tokio::sync::RwLock;
#[cfg(feature = "cli")]
use tokio::task;
#[cfg(feature = "cli")]
use tokio::sync::Semaphore;
#[cfg(feature = "cli")]
use futures::future::join_all;

/// Execute a workflow with parallel execution for independent steps
#[cfg(feature = "cli")]
pub async fn run_workflow_parallel(
    path: &str,
    max_concurrent: usize,
) -> anyhow::Result<()> {
    let steps = load_workflow(path)?;
    let results: Arc<RwLock<HashMap<String, serde_json::Value>>> = Arc::new(RwLock::new(HashMap::new()));
    
    // Group steps by dependency level
    let execution_levels = group_by_dependency_level(&steps)?;
    
    // Create semaphore to limit concurrent execution
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    
    println!("🚀 Parallel execution mode enabled (max concurrent: {})", max_concurrent);
    println!("📊 Execution plan: {} levels", execution_levels.len());
    
    for (level_index, level) in execution_levels.iter().enumerate() {
        let level_number = level_index + 1;
        println!("\n=== Level {}/{}: {} step(s) {} ===", 
            level_number, 
            execution_levels.len(),
            level.len(),
            if level.len() > 1 { "(parallel)" } else { "(sequential)" }
        );
        
        let mut handles = vec![];
        
        for step in level {
            let permit = semaphore.clone().acquire_owned().await
                .map_err(|e| anyhow::anyhow!("Failed to acquire semaphore: {}", e))?;
            let results_clone = Arc::clone(&results);
            let step_owned = step.clone();
            
            let handle = task::spawn(async move {
                let _permit = permit; // Hold permit until task completes
                
                // Gather inputs from dependencies
                let inputs = {
                    let results_read = results_clone.read().await;
                    let mut inputs_map = HashMap::new();
                    for dep in &step_owned.depends_on {
                        if let Some(val) = results_read.get(dep) {
                            inputs_map.insert(dep.clone(), val.clone());
                        }
                    }
                    inputs_map
                };
                
                // Execute the step
                let output = execute_step(&step_owned, &inputs)?;
                
                // Store result
                {
                    let mut results_write = results_clone.write().await;
                    results_write.insert(step_owned.name.clone(), output.clone());
                }
                
                Ok::<(String, serde_json::Value), anyhow::Error>((step_owned.name.clone(), output))
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks in this level to complete
        let level_results = join_all(handles).await;
        
        // Check for errors and print results
        for result in level_results {
            match result {
                Ok(Ok((name, output))) => {
                    println!("  ✓ '{}' completed: {}", name, output);
                }
                Ok(Err(e)) => {
                    return Err(anyhow::anyhow!("Step failed: {}", e));
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Task panic: {}", e));
                }
            }
        }
    }
    
    println!("\n✅ Workflow completed successfully!");
    Ok(())
}

/// Execute a single step (shared logic with sequential execution)
#[cfg(feature = "cli")]
fn execute_step(
    step: &Step,
    inputs: &HashMap<String, serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    match step.language.as_str() {
        "python" => run_python_step(&step.name, &step.code, inputs),
        "lua" => run_lua_step(&step.name, &step.code, inputs),
        "bash" | "shell" | "sh" => run_shell_step(&step.name, &step.code, inputs),
        "javascript" | "js" | "node" | "nodejs" => run_javascript_step(&step.name, &step.code, inputs),
        "wasm" | "webassembly" => {
            let module_path = step.module_path.as_ref()
                .ok_or_else(|| anyhow::anyhow!("WASM step '{}' missing 'module' field", step.name))?;
            run_wasm_step(&step.name, module_path, step.function_name.as_deref(), inputs)
        }
        _ => Err(anyhow::anyhow!("Unsupported language: {}", step.language)),
    }
}

/// Group steps into execution levels based on dependencies
/// Steps in the same level can execute in parallel
#[cfg(feature = "cli")]
fn group_by_dependency_level(steps: &[Step]) -> anyhow::Result<Vec<Vec<Step>>> {
    let mut levels: Vec<Vec<Step>> = vec![];
    let mut step_levels: HashMap<String, usize> = HashMap::new();
    let step_map: HashMap<String, Step> = steps.iter()
        .map(|s| (s.name.clone(), s.clone()))
        .collect();
    
    // Calculate level for each step recursively
    fn calculate_level(
        step_name: &str,
        step_map: &HashMap<String, Step>,
        step_levels: &mut HashMap<String, usize>,
        visiting: &mut HashSet<String>,
    ) -> anyhow::Result<usize> {
        // Return cached level if already calculated
        if let Some(&level) = step_levels.get(step_name) {
            return Ok(level);
        }
        
        // Detect circular dependencies
        if visiting.contains(step_name) {
            return Err(anyhow::anyhow!("Circular dependency detected involving step '{}'", step_name));
        }
        
        visiting.insert(step_name.to_string());
        
        let step = step_map.get(step_name)
            .ok_or_else(|| anyhow::anyhow!("Step not found: {}", step_name))?;
        
        // Level is 0 if no dependencies, otherwise max(dependency levels) + 1
        let level = if step.depends_on.is_empty() {
            0
        } else {
            let dep_levels: Result<Vec<usize>, _> = step.depends_on.iter()
                .map(|dep| calculate_level(dep, step_map, step_levels, visiting))
                .collect();
            
            let max_dep_level = dep_levels?
                .into_iter()
                .max()
                .unwrap_or(0);
            
            max_dep_level + 1
        };
        
        visiting.remove(step_name);
        step_levels.insert(step_name.to_string(), level);
        Ok(level)
    }
    
    // Calculate levels for all steps
    let mut visiting = HashSet::new();
    for step in steps {
        calculate_level(&step.name, &step_map, &mut step_levels, &mut visiting)?;
    }
    
    // Group steps by level
    let max_level = step_levels.values().max().copied().unwrap_or(0);
    levels.resize(max_level + 1, vec![]);
    
    for step in steps {
        let level = step_levels[&step.name];
        levels[level].push(step.clone());
    }
    
    Ok(levels)
}

#[cfg(test)]
#[cfg(feature = "cli")]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_level_no_dependencies() {
        let steps = vec![
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
                module_path: None,
                function_name: None,
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
                module_path: None,
                function_name: None,
            },
        ];

        let result = group_by_dependency_level(&steps);
        assert!(result.is_ok());
        let levels = result.unwrap();
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].len(), 2); // Both in level 0
    }

    #[test]
    fn test_group_by_level_with_dependencies() {
        let steps = vec![
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
                module_path: None,
                function_name: None,
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string()],
                module_path: None,
                function_name: None,
            },
        ];

        let result = group_by_dependency_level(&steps);
        assert!(result.is_ok());
        let levels = result.unwrap();
        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0].len(), 1); // step1 in level 0
        assert_eq!(levels[1].len(), 1); // step2 in level 1
    }

    #[test]
    fn test_group_by_level_complex() {
        let steps = vec![
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
                module_path: None,
                function_name: None,
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec![],
                module_path: None,
                function_name: None,
            },
            Step {
                name: "step3".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string(), "step2".to_string()],
                module_path: None,
                function_name: None,
            },
        ];

        let result = group_by_dependency_level(&steps);
        assert!(result.is_ok());
        let levels = result.unwrap();
        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0].len(), 2); // step1, step2 in level 0 (can run in parallel)
        assert_eq!(levels[1].len(), 1); // step3 in level 1
    }

    #[test]
    fn test_group_by_level_circular_dependency() {
        let steps = vec![
            Step {
                name: "step1".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step2".to_string()],
                module_path: None,
                function_name: None,
            },
            Step {
                name: "step2".to_string(),
                language: "lua".to_string(),
                code: "".to_string(),
                depends_on: vec!["step1".to_string()],
                module_path: None,
                function_name: None,
            },
        ];

        let result = group_by_dependency_level(&steps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular dependency"));
    }
}
