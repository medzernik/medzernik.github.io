use leptos::prelude::*;
use stylance::import_crate_style;
import_crate_style!(style, "src/main.module.css");

#[component]
pub fn about_page() -> impl IntoView {
    view! {
        <Header></Header>
        <TopBar></TopBar>
    }
}
