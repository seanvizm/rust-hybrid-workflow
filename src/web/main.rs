use leptos::mount::mount_to_body;
use workflow_web::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
