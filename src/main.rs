use leptos::prelude::*;

mod app;
mod components;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
