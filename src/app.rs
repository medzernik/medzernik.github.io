use crate::pages::{
    about_page::AboutPage, home_page::HomePage, post_page::PostPage, posts_page::PostsPage,
};
use leptos::{attr::Scope, prelude::*};
use leptos_router::{components::*, path};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class=style::headerText>"404 - NOT FOUND"</div> }>
                <Route path=path!("/") view=HomePage />
                <ParentRoute path=path!("/users") view=PostsPage>
                    <Route path=path!(":id") view=PostPage />
                // <Route path=path!("") view=NoUser />
                </ParentRoute>
                <Route path=path!("/:about") view=AboutPage />
            </Routes>
        </Router>
    }
}
