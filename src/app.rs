use crate::{
    components::header::{Header, TopBar},
    pages::{
        about_page::AboutPage, home_page::HomePage, post_page::SpecificPostPage,
        posts_page::AllPostsPage,
    },
};
use leptos::prelude::*;
use leptos_router::{components::*, path};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <Header />
                <TopBar />
            </nav>
            <main>
                <Routes fallback=|| view! { <div class=style::headerText>"404 - NOT FOUND"</div> }>
                    <Route path=path!("/") view=HomePage />
                    <ParentRoute path=path!("/posts") view=AllPostsPage>
                        <Route path=path!(":id") view=SpecificPostPage />
                    </ParentRoute>
                    <Route path=path!("/about") view=AboutPage />
                </Routes>
            </main>
        </Router>
    }
}
