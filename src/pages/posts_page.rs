use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_params_map};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn AllPostsPage() -> impl IntoView {
    view! {
        <div>
            <AllPostsPageContent />
        </div>
    }
}

#[component]
pub fn AllPostsPageContent() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    view! {
        <div class=style::bodyContainer>
            <h1>ID IS: {id}</h1>
            <h1 class=style::headerText># Posts overview</h1>
            <p class=style::bodyText>This is a page where all posts should be listed.</p>
            <Outlet />
        </div>
    }
}
