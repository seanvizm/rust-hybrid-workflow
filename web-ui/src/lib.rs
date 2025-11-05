use leptos::*;
use wasm_bindgen::prelude::*;

mod app;
mod components;

use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Log to console for debugging
    web_sys::console::log_1(&"🚀 WASM module loaded, starting Leptos app...".into());
    
    mount_to_body(|| view! { <App/> });
    
    web_sys::console::log_1(&"✅ Leptos app mounted to body".into());
}
