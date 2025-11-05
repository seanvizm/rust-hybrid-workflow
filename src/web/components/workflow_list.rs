use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkflowInfo {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub path: String,
}

#[component]
pub fn WorkflowList() -> impl IntoView {
    let (workflows, set_workflows) = create_signal(Vec::<WorkflowInfo>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // Load workflows on mount
    create_effect(move |_| {
        spawn_local(async move {
            match fetch_workflows().await {
                Ok(wf) => {
                    set_workflows.set(wf);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="workflow-list-container">
            <div class="page-header">
                <h2>"📋 Available Workflows"</h2>
                <p>"Select a workflow to run and view its execution steps"</p>
            </div>

            <Show
                when=move || loading.get()
                fallback=move || {
                    view! {
                        <Show
                            when=move || error.get().is_some()
                            fallback=move || {
                                view! {
                                    <div class="workflows-grid">
                                        <For
                                            each=move || workflows.get()
                                            key=|w| w.name.clone()
                                            children=move |workflow: WorkflowInfo| {
                                                view! { <WorkflowCard workflow=workflow/> }
                                            }
                                        />
                                    </div>
                                }
                            }
                        >
                            <div class="error-message">
                                <p>"❌ Error loading workflows: " {move || error.get()}</p>
                            </div>
                        </Show>
                    }
                }
            >
                <div class="loading-spinner">
                    <div class="spinner"></div>
                    <p>"Loading workflows..."</p>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn WorkflowCard(workflow: WorkflowInfo) -> impl IntoView {
    let name = workflow.name.clone();
    let navigate_url = format!("/workflow/{}", name);

    view! {
        <div class="workflow-card">
            <div class="workflow-card-header">
                <h3>{workflow.display_name.clone()}</h3>
                <span class="workflow-badge">"Lua"</span>
            </div>

            <div class="workflow-card-body">
                <Show
                    when=move || workflow.description.is_some()
                    fallback=|| {
                        view! { <p class="workflow-description">"No description available"</p> }
                    }
                >
                    <p class="workflow-description">
                        {workflow.description.clone().unwrap_or_default()}
                    </p>
                </Show>

                <div class="workflow-meta">
                    <span class="meta-item">
                        <span class="meta-icon">"📄"</span>
                        {workflow.path.clone()}
                    </span>
                </div>
            </div>

            <div class="workflow-card-footer">
                <a href=navigate_url class="btn btn-primary">
                    "▶ Run Workflow"
                </a>
            </div>
        </div>
    }
}

async fn fetch_workflows() -> Result<Vec<WorkflowInfo>, String> {
    let response = gloo_net::http::Request::get("/api/workflows")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch workflows: {}", e))?;

    if response.ok() {
        response
            .json::<Vec<WorkflowInfo>>()
            .await
            .map_err(|e| format!("Failed to parse workflows: {}", e))
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}
