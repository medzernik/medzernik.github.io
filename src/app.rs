use crate::pages::{about_page::AboutPage, home_page::HomePage, posts_page::PostsPage};
use leptos::prelude::*;
use leptos_router::{components::*, path};
use std::{io::Read, path::PathBuf};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/posts") view=PostsPage />
                <Route path=path!("/about") view=AboutPage />
            </Routes>
        </Router>
    }
}

pub async fn load_md_file(path: PathBuf) -> String {
    std::fs::read_to_string(path.to_str().unwrap()).unwrap()
}
