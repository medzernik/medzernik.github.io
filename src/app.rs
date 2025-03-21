use leptos::prelude::*;
use leptos_router::{components::*, path};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=MainPage />
            </Routes>
        </Router>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    view! { <p class=style::test>AHOJS</p> }
}
