use leptos::prelude::*;
use stylance::import_crate_style;

use crate::components::header::{Header, TopBar};
import_crate_style!(style, "src/main.module.css");

#[component]
pub fn PostsPage() -> impl IntoView {
    view! {
        <div>
            <Header />
            <TopBar />
            <PostsPageContent />
        </div>
    }
}

#[component]
pub fn PostsPageContent() -> impl IntoView {
    view! {
        <div class=style::bodyContainer>
            <h1 class=style::headerText># Test</h1>
            <p class=style::bodyText>be distracting.</p>
        </div>
    }
}
