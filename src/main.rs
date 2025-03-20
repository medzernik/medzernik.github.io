use leptos::prelude::*;
mod app;
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
