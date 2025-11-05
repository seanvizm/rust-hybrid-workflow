use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found-container">
            <div class="not-found-content">
                <h1 class="not-found-title">"404"</h1>
                <h2 class="not-found-subtitle">"Page Not Found"</h2>
                <p class="not-found-message">
                    "The page you're looking for doesn't exist or has been moved."
                </p>
                <div class="not-found-actions">
                    <a href="/" class="btn btn-primary">
                        "← Back to Workflows"
                    </a>
                </div>
                <div class="not-found-illustration">
                    <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="100" cy="100" r="80" fill="none" stroke="currentColor" stroke-width="4" opacity="0.2"/>
                        <text x="100" y="110" text-anchor="middle" font-size="48" fill="currentColor" opacity="0.5">
                            "?"
                        </text>
                    </svg>
                </div>
            </div>
        </div>
    }
}
