use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStep {
    pub step_number: usize,
    pub name: String,
    pub language: String,
    pub output: Option<String>,
    pub status: StepStatus,
    pub duration_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub workflow_name: String,
    pub status: ExecutionStatus,
    pub steps: Vec<WorkflowStep>,
    pub total_duration_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Completed,
    Failed,
}

#[component]
pub fn WorkflowRunner() -> impl IntoView {
    let params = use_params_map();
    let workflow_name = move || {
        params.with(|p| p.get("name").cloned().unwrap_or_default())
    };

    let (execution, set_execution) = create_signal(None::<WorkflowExecution>);
    let (running, set_running) = create_signal(false);
    let (expanded_steps, set_expanded_steps) = create_signal(Vec::<usize>::new());

    let run_workflow = move || {
        let name = workflow_name();
        set_running.set(true);

        spawn_local(async move {
            match execute_workflow(&name).await {
                Ok(exec) => {
                    set_execution.set(Some(exec));
                    set_running.set(false);
                }
                Err(e) => {
                    let error_exec = WorkflowExecution {
                        workflow_name: name.clone(),
                        status: ExecutionStatus::Failed,
                        steps: vec![],
                        total_duration_ms: None,
                        error: Some(e),
                    };
                    set_execution.set(Some(error_exec));
                    set_running.set(false);
                }
            }
        });
    };

    let toggle_step = move |step_num: usize| {
        set_expanded_steps.update(|steps| {
            if steps.contains(&step_num) {
                steps.retain(|&s| s != step_num);
            } else {
                steps.push(step_num);
            }
        });
    };

    view! {
        <div class="workflow-runner-container">
            <div class="workflow-header">
                <a href="/" class="back-link">
                    "← Back to Workflows"
                </a>
                <h2>{move || workflow_name()}</h2>
            </div>

            <div class="workflow-controls">
                <button
                    class="btn btn-primary btn-large"
                    on:click=move |_| run_workflow()
                    disabled=move || running.get()
                >
                    <Show
                        when=move || running.get()
                        fallback=|| view! { <span>"▶ Run Workflow"</span> }
                    >
                        <span class="spinner-small"></span>
                        <span>"Running..."</span>
                    </Show>
                </button>
            </div>

            <Show when=move || execution.get().is_some()>
                {move || {
                    execution
                        .get()
                        .map(|exec| {
                            view! { <ExecutionResults execution=exec toggle_step=toggle_step expanded_steps=expanded_steps/> }
                        })
                }}
            </Show>
        </div>
    }
}

#[component]
fn ExecutionResults(
    execution: WorkflowExecution,
    toggle_step: impl Fn(usize) + 'static + Copy,
    expanded_steps: ReadSignal<Vec<usize>>,
) -> impl IntoView {
    let status_class = match execution.status {
        ExecutionStatus::Completed => "status-success",
        ExecutionStatus::Failed => "status-error",
        ExecutionStatus::Running => "status-running",
        ExecutionStatus::NotStarted => "status-pending",
    };

    let status_icon = match execution.status {
        ExecutionStatus::Completed => "✅",
        ExecutionStatus::Failed => "❌",
        ExecutionStatus::Running => "⏳",
        ExecutionStatus::NotStarted => "⏸",
    };

    view! {
        <div class="execution-results">
            <div class=format!("execution-status {}", status_class)>
                <span class="status-icon">{status_icon}</span>
                <span class="status-text">
                    {match execution.status {
                        ExecutionStatus::Completed => "Workflow Completed Successfully",
                        ExecutionStatus::Failed => "Workflow Failed",
                        ExecutionStatus::Running => "Workflow Running...",
                        ExecutionStatus::NotStarted => "Ready to Run",
                    }}
                </span>
                {execution
                    .total_duration_ms
                    .map(|ms| {
                        view! {
                            <span class="duration">{format!("({:.2}s)", ms as f64 / 1000.0)}</span>
                        }
                    })}
            </div>

            <Show when={
                let err = execution.error.clone();
                move || err.is_some()
            }>
                <div class="error-details">
                    <h4>"Error Details:"</h4>
                    <pre>{execution.error.clone().unwrap_or_default()}</pre>
                </div>
            </Show>

            <div class="steps-container">
                <h3>"Workflow Steps"</h3>
                <div class="steps-list">
                    <For
                        each=move || execution.steps.clone()
                        key=|step| step.step_number
                        children=move |step: WorkflowStep| {
                            let step_num = step.step_number;
                            let is_expanded = move || expanded_steps.get().contains(&step_num);
                            view! {
                                <StepCard
                                    step=step
                                    is_expanded=is_expanded
                                    on_toggle=move || toggle_step(step_num)
                                />
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn StepCard(
    step: WorkflowStep,
    is_expanded: impl Fn() -> bool + 'static + Copy,
    on_toggle: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let status_class = match step.status {
        StepStatus::Success => "step-success",
        StepStatus::Failed => "step-failed",
        StepStatus::Running => "step-running",
        StepStatus::Pending => "step-pending",
    };

    let status_icon = match step.status {
        StepStatus::Success => "✅",
        StepStatus::Failed => "❌",
        StepStatus::Running => "⏳",
        StepStatus::Pending => "⏸",
    };

    view! {
        <div class=format!("step-card {}", status_class)>
            <div class="step-header" on:click=move |_| on_toggle()>
                <div class="step-title">
                    <span class="step-number">{format!("Step {}", step.step_number)}</span>
                    <span class="step-name">{step.name.clone()}</span>
                    <span class="step-language-badge">{step.language.clone()}</span>
                </div>
                <div class="step-status">
                    <span class="status-icon">{status_icon}</span>
                    {step
                        .duration_ms
                        .map(|ms| {
                            view! {
                                <span class="step-duration">
                                    {format!("{:.2}s", ms as f64 / 1000.0)}
                                </span>
                            }
                        })}
                    <span class="expand-icon">
                        {move || if is_expanded() { "▼" } else { "▶" }}
                    </span>
                </div>
            </div>

            <Show when=is_expanded>
                <div class="step-output">
                    <h4>"Output:"</h4>
                    {
                        let output = step.output.clone().unwrap_or_else(|| "No output".to_string());
                        let trimmed = output.trim();
                        
                        // Try to parse as JSON first
                        let parsed_output = if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&trimmed) {
                            // Check if it's a simple string wrapped in quotes
                            if let Some(s) = json_value.as_str() {
                                // It's a JSON string, unwrap it
                                s.to_string()
                            } else {
                                // It's a complex JSON (object or array), keep as is
                                output.clone()
                            }
                        } else {
                            // Not valid JSON, use as is
                            output.clone()
                        };
                        
                        // Now detect the format of the unwrapped output
                        let final_trimmed = parsed_output.trim();
                        let is_json = (final_trimmed.starts_with('{') && final_trimmed.ends_with('}')) || 
                                     (final_trimmed.starts_with('[') && final_trimmed.ends_with(']'));
                        
                        if is_json {
                            // Try to parse and prettify JSON
                            match serde_json::from_str::<serde_json::Value>(&final_trimmed) {
                                Ok(value) => {
                                    let prettified = serde_json::to_string_pretty(&value)
                                        .unwrap_or_else(|_| parsed_output.clone());
                                    view! {
                                        <div>
                                            <div class="output-format-badge">"JSON"</div>
                                            <pre class="output-content output-json">{prettified}</pre>
                                        </div>
                                    }.into_view()
                                }
                                Err(_) => {
                                    // Not valid JSON, treat as Text/HTML
                                    view! {
                                        <div>
                                            <div class="output-format-badge output-format-badge-text">"Text / HTML"</div>
                                            <div class="output-content output-html-text" inner_html=parsed_output.clone()></div>
                                        </div>
                                    }.into_view()
                                }
                            }
                        } else {
                            // Not JSON - render as HTML (text will display as-is)
                            view! {
                                <div>
                                    <div class="output-format-badge output-format-badge-text">"Text / HTML"</div>
                                    <div class="output-content output-html-text" inner_html=parsed_output></div>
                                </div>
                            }.into_view()
                        }
                    }
                </div>
            </Show>
        </div>
    }
}

async fn execute_workflow(name: &str) -> Result<WorkflowExecution, String> {
    let response = gloo_net::http::Request::post(&format!("/api/workflows/{}/run", name))
        .send()
        .await
        .map_err(|e| format!("Failed to execute workflow: {}", e))?;

    if response.ok() {
        response
            .json::<WorkflowExecution>()
            .await
            .map_err(|e| format!("Failed to parse execution result: {}", e))
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}
