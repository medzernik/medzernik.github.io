use leptos::prelude::*;
use leptos_router::{components::*, path};
use stylance::import_crate_style;

use crate::{components::header::TopBar, pages::about_page::AboutPage};

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=AboutPage />
                <Route path=path!("/home") view=AboutPage />
                <Route path=path!("/about") view=AboutPage />
            </Routes>
        </Router>
    }
}
