use crate::{
    components::header::{Header, TopBar},
    pages::{
        about_page::AboutPage, home_page::HomePage, post_page::PostPageContent,
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
                // In your main App component's view
                <main>
                    <Routes fallback=|| view! { <h1>"404 - Not Found"</h1> }>
                        // Home page
                        <Route path=path!("/") view=HomePage />

                        // 1. Route for the list of all posts
                        <Route path=path!("/posts") view=AllPostsPage />

                        // 2. Route for a single, specific post, using a dynamic :id
                        <Route path=path!("/posts/:id") view=PostPageContent />

                        // About page
                        <Route path=path!("/about") view=AboutPage />
                    </Routes>
                </main>
            </nav>
        </Router>
    }
}
