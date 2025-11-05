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

// Use the library from this crate
use workflow_engine::run_workflow;
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
    
    // For now, we'll create a simplified execution result
    // In a real implementation, you'd want to capture step-by-step output
    match run_workflow(&workflow_path) {
        Ok(_) => {
            let duration = start_time.elapsed();
            let execution = WorkflowExecution {
                workflow_name: name.clone(),
                status: ExecutionStatus::Completed,
                steps: extract_steps_from_workflow(&workflow_path),
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
                steps: extract_steps_from_workflow(&workflow_path),
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

fn extract_steps_from_workflow(path: &str) -> Vec<WorkflowStep> {
    // This is a simplified version. In production, you'd want to actually
    // parse the Lua file and track real execution
    if let Ok(content) = fs::read_to_string(path) {
        let mut steps = Vec::new();
        let mut step_number = 1;

        // Simple regex-like parsing for step names
        for line in content.lines() {
            if line.trim_start().starts_with("-- Step") || 
               (line.contains("=") && line.contains("{") && !line.contains("workflow")) {
                if let Some(step_name) = extract_step_name(line) {
                    let language = extract_language(&content, &step_name);
                    steps.push(WorkflowStep {
                        step_number,
                        name: step_name,
                        language,
                        output: Some("Step executed successfully".to_string()),
                        status: StepStatus::Success,
                        duration_ms: Some((step_number * 100) as u64), // Mock duration
                    });
                    step_number += 1;
                }
            }
        }

        steps
    } else {
        vec![]
    }
}

fn extract_step_name(line: &str) -> Option<String> {
    if let Some(pos) = line.find('=') {
        let name = line[..pos].trim();
        if !name.is_empty() && name != "workflow" && name != "steps" {
            return Some(name.to_string());
        }
    }
    None
}

fn extract_language(content: &str, step_name: &str) -> String {
    content
        .lines()
        .skip_while(|line| !line.contains(step_name))
        .take_while(|line| !line.trim().starts_with('}'))
        .find(|line| line.contains("language ="))
        .and_then(|line| line.split('"').nth(1).map(|s| s.to_string()))
        .unwrap_or_else(|| "lua".to_string())
}
