use leptos::prelude::*;
use stylance::import_crate_style;

use crate::components::header::{Header, TopBar};
import_crate_style!(style, "src/main.module.css");

#[component]
pub fn PostPage() -> impl IntoView {
    view! {
        <div>
            <Header />
            <TopBar />
            <PostPageContent />
        </div>
    }
}

#[component]
pub fn PostPageContent() -> impl IntoView {
    view! {
        <div class=style::bodyContainer>
            <h1 class=style::headerText># Test For a Post</h1>
            <p class=style::bodyText>beasdasd.</p>
        </div>
    }
}
