use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{WorkflowList, WorkflowRunner, NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/workflow-web.css"/>
        <Title text="Hybrid Workflow Engine - Web UI"/>
        <Meta
            name="description"
            content="Interactive web interface for running and monitoring hybrid workflows"
        />

        <Router>
            <main class="container">
                            <div class="header">
                <div class="container">
                    <h1>"Hybrid Workflow Engine"</h1>
                    <p class="subtitle">"Multi-language workflow orchestration powered by Rust"</p>
                </div>
            </div>

                <Routes>
                    <Route path="/" view=WorkflowList/>
                    <Route path="/workflow/:name" view=WorkflowRunner/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
