use crate::components::header::{Header, TopBar};
use leptos::prelude::*;
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <Header />
            <TopBar />
            <AboutPageContent />
        </div>
    }
}

#[component]
pub fn AboutPageContent() -> impl IntoView {
    view! {
        <div class=style::bodyContainer>
            <h1 class=style::headerText># Test</h1>
            <p class=style::bodyText>
                This is the about page.
            </p>
        </div>
    }
}
