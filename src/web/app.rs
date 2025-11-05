use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::web::components::{WorkflowList, WorkflowRunner};

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
                <header class="header">
                    <h1>"🚀 Hybrid Workflow Engine"</h1>
                    <p class="subtitle">"Multi-language workflow orchestration"</p>
                </header>

                <Routes>
                    <Route path="/" view=WorkflowList/>
                    <Route path="/workflow/:name" view=WorkflowRunner/>
                </Routes>
            </main>
        </Router>
    }
}
