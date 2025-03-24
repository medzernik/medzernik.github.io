use leptos::prelude::*;
use stylance::import_crate_style;

use crate::components::header::{Header, TopBar};
import_crate_style!(style, "src/main.module.css");

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <Header></Header>
            <TopBar></TopBar>
        </div>
    }
}
