use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkflowInfo {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub path: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ViewMode {
    Table,
    Grid,
}

#[component]
pub fn WorkflowList() -> impl IntoView {
    let (workflows, set_workflows) = create_signal(Vec::<WorkflowInfo>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (view_mode, set_view_mode) = create_signal(ViewMode::Table);

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
                <div class="header-content">
                    <div>
                        <h2>"Available Workflows"</h2>
                        <p>"Select a workflow to run and view its execution steps"</p>
                    </div>
                    <div class="view-toggle">
                        <button
                            class=move || {
                                if view_mode.get() == ViewMode::Table {
                                    "view-btn active"
                                } else {
                                    "view-btn"
                                }
                            }
                            on:click=move |_| set_view_mode.set(ViewMode::Table)
                            title="Table View"
                        >
                            "☰"
                        </button>
                        <button
                            class=move || {
                                if view_mode.get() == ViewMode::Grid {
                                    "view-btn active"
                                } else {
                                    "view-btn"
                                }
                            }
                            on:click=move |_| set_view_mode.set(ViewMode::Grid)
                            title="Grid View"
                        >
                            "▦"
                        </button>
                    </div>
                </div>
            </div>

            <Show
                when=move || loading.get()
                fallback=move || {
                    view! {
                        <Show
                            when=move || error.get().is_some()
                            fallback=move || {
                                view! {
                                    <Show
                                        when=move || view_mode.get() == ViewMode::Table
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
                                        <WorkflowTable workflows=workflows/>
                                    </Show>
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

/// Format workflow name for display: replace underscores with spaces and capitalize each word
fn format_display_name(name: &str) -> String {
    name.replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[component]
fn WorkflowTable(workflows: ReadSignal<Vec<WorkflowInfo>>) -> impl IntoView {
    view! {
        <div class="workflow-table-container">
            <table class="workflow-table">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Description"</th>
                        <th>"Type"</th>
                        <th>"Path"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || workflows.get()
                        key=|w| w.name.clone()
                        children=move |workflow: WorkflowInfo| {
                            let name = workflow.name.clone();
                            let navigate_url = format!("/workflow/{}", name);
                            let formatted_name = format_display_name(&workflow.display_name);
                            let description = workflow
                                .description
                                .clone()
                                .unwrap_or_else(|| "No description available".to_string());
                            view! {
                                <tr class="workflow-row">
                                    <td class="workflow-name" data-label="Name">
                                        {formatted_name}
                                    </td>
                                    <td class="workflow-description" data-label="Description">
                                        {description}
                                    </td>
                                    <td data-label="Type">
                                        <span class="workflow-badge-small">"Lua"</span>
                                    </td>
                                    <td class="workflow-path" data-label="Path">
                                        {workflow.path.clone()}
                                    </td>
                                    <td data-label="Actions">
                                        <a href=navigate_url class="btn btn-sm btn-primary">
                                            "▶ Run"
                                        </a>
                                    </td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn WorkflowCard(workflow: WorkflowInfo) -> impl IntoView {
    let name = workflow.name.clone();
    let navigate_url = format!("/workflow/{}", name);
    let has_description = workflow.description.is_some();
    let description = workflow.description.clone().unwrap_or_default();
    let formatted_name = format_display_name(&workflow.display_name);

    view! {
        <div class="workflow-card">
            <div class="workflow-card-header">
                <h3>{formatted_name}</h3>
                <span class="workflow-badge">"Lua"</span>
            </div>

            <div class="workflow-card-body">
                <Show
                    when=move || has_description
                    fallback=|| {
                        view! { <p class="workflow-description">"No description available"</p> }
                    }
                >
                    <p class="workflow-description">{description.clone()}</p>
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
