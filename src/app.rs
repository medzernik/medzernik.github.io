use leptos::prelude::*;
use leptos_router::{components::*, path};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=TopBar />
                <Route path=path!("/home") view=TopBar />
                <Route path=path!("/test") view=TopBar />

            </Routes>
        </Router>
    }
}

#[component]
fn TopBar() -> impl IntoView {
    view! {
        <div class=style::topBar>
            <p class=style::topBarButton>HOME</p>
            <p class=style::topBarButton>POSTS</p>
            <p class=style::topBarButton>ABOUT</p>
        </div>
    }
}
