mod api;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tower_http::services::ServeDir;

use api::{ExecutionStatus, StepStatus, WorkflowExecution, WorkflowInfo, WorkflowStep};

#[tokio::main]
async fn main() {
    println!("🚀 Starting Hybrid Workflow Engine Web Server...");
    println!("📍 Server running at http://localhost:3000");
    println!();

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/workflows", get(list_workflows))
        .route("/api/workflows/{name}/run", post(run_workflow_handler))
        .nest_service("/assets", ServeDir::new("assets"))
        // Serve all static files from pkg directory (including WASM, JS, CSS)
        .fallback_service(ServeDir::new("pkg"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("✅ Server ready!");
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    // Serve the Trunk-built index.html
    match tokio::fs::read_to_string("pkg/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Error</title>
</head>
<body>
    <h1>Error: Could not load index.html</h1>
    <p>Make sure to build the frontend first with: cd web-ui && trunk build</p>
</body>
</html>"#.to_string(),
        ),
    }
}

async fn list_workflows() -> Result<Json<Vec<WorkflowInfo>>, StatusCode> {
    let workflows_dir = PathBuf::from("workflows");

    if !workflows_dir.exists() {
        return Ok(Json(vec![]));
    }

    let mut workflows = Vec::new();

    if let Ok(entries) = fs::read_dir(&workflows_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    let workflow_info = extract_workflow_info(&path);
                    workflows.push(WorkflowInfo {
                        name: file_name.to_string(),
                        display_name: workflow_info.0,
                        description: workflow_info.1,
                        path: path
                            .strip_prefix(".")
                            .unwrap_or(&path)
                            .display()
                            .to_string(),
                    });
                }
            }
        }
    }

    workflows.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    Ok(Json(workflows))
}

async fn run_workflow_handler(
    Path(name): Path<String>,
) -> Result<Json<WorkflowExecution>, StatusCode> {
    let workflow_path = format!("workflows/{}.lua", name);

    if !PathBuf::from(&workflow_path).exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let start_time = Instant::now();
    
    // Execute workflow and capture step-by-step results
    match execute_workflow_with_tracking(&workflow_path) {
        Ok(steps) => {
            let duration = start_time.elapsed();
            let execution = WorkflowExecution {
                workflow_name: name.clone(),
                status: ExecutionStatus::Completed,
                steps,
                total_duration_ms: Some(duration.as_millis() as u64),
                error: None,
            };
            Ok(Json(execution))
        }
        Err(e) => {
            let duration = start_time.elapsed();
            let execution = WorkflowExecution {
                workflow_name: name.clone(),
                status: ExecutionStatus::Failed,
                steps: vec![],
                total_duration_ms: Some(duration.as_millis() as u64),
                error: Some(e.to_string()),
            };
            Ok(Json(execution))
        }
    }
}

fn extract_workflow_info(path: &PathBuf) -> (String, Option<String>) {
    if let Ok(content) = fs::read_to_string(path) {
        let name = content
            .lines()
            .find(|line| line.contains("name ="))
            .and_then(|line| {
                line.split('"')
                    .nth(1)
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string()
            });

        let description = content.lines().find(|line| line.contains("description =")).and_then(
            |line| {
                line.split('"')
                    .nth(1)
                    .map(|s| s.to_string())
            },
        );

        (name, description)
    } else {
        (
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            None,
        )
    }
}

fn execute_workflow_with_tracking(path: &str) -> anyhow::Result<Vec<WorkflowStep>> {
    use workflow_engine::core::lua_loader::load_workflow;
    use workflow_engine::runners::{run_lua_step, run_python_step, run_shell_step, run_javascript_step, run_wasm_step};
    use std::collections::HashMap;
    use std::time::Instant;

    let mut workflow_steps = load_workflow(path)?;
    let mut results: HashMap<String, serde_json::Value> = HashMap::new();
    let mut tracked_steps = Vec::new();

    // Sort steps by dependencies (using the same logic as the engine)
    workflow_steps = sort_steps_for_execution(workflow_steps)?;

    for (step_index, step) in workflow_steps.iter().enumerate() {
        let step_number = step_index + 1;
        let step_start = Instant::now();
        
        let mut inputs = HashMap::new();
        for dep in &step.depends_on {
            if let Some(val) = results.get(dep) {
                inputs.insert(dep.clone(), val.clone());
            }
        }

        let result = match step.language.as_str() {
            "python" => run_python_step(&step.name, &step.code, &inputs),
            "lua" => run_lua_step(&step.name, &step.code, &inputs),
            "bash" | "shell" | "sh" => run_shell_step(&step.name, &step.code, &inputs),
            "javascript" | "js" | "node" | "nodejs" => run_javascript_step(&step.name, &step.code, &inputs),
            "wasm" | "webassembly" => {
                let module_path = step.module_path.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("WASM step '{}' missing 'module' field", step.name))?;
                run_wasm_step(&step.name, module_path, step.function_name.as_deref(), &inputs)
            }
            _ => Err(anyhow::anyhow!("Unsupported language: {}", step.language)),
        };

        let duration = step_start.elapsed();

        match result {
            Ok(output) => {
                let output_str = output.to_string();
                results.insert(step.name.clone(), output);
                
                tracked_steps.push(WorkflowStep {
                    step_number,
                    name: step.name.clone(),
                    language: step.language.clone(),
                    output: Some(output_str),
                    status: StepStatus::Success,
                    duration_ms: Some(duration.as_millis() as u64),
                });
            }
            Err(e) => {
                tracked_steps.push(WorkflowStep {
                    step_number,
                    name: step.name.clone(),
                    language: step.language.clone(),
                    output: Some(format!("Error: {}", e)),
                    status: StepStatus::Failed,
                    duration_ms: Some(duration.as_millis() as u64),
                });
                return Err(e);
            }
        }
    }

    Ok(tracked_steps)
}

fn sort_steps_for_execution(steps: Vec<workflow_engine::core::lua_loader::Step>) -> anyhow::Result<Vec<workflow_engine::core::lua_loader::Step>> {
    use std::collections::{HashMap, HashSet};
    
    let mut sorted = Vec::new();
    let mut remaining: HashMap<String, workflow_engine::core::lua_loader::Step> = 
        steps.into_iter().map(|s| (s.name.clone(), s)).collect();
    let mut processed: HashSet<String> = HashSet::new();
    
    while !remaining.is_empty() {
        let mut progress = false;
        let mut to_remove = Vec::new();
        
        for (name, step) in &remaining {
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
            return Err(anyhow::anyhow!("Circular dependency detected"));
        }
    }
    
    Ok(sorted)
}
