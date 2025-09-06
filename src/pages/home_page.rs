use leptos::prelude::*;
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <HomePageContent />
        </div>
    }
}

#[component]
pub fn HomePageContent() -> impl IntoView {
    view! {
        <div class=style::bodyContainer>
            <h1 class=style::headerText># Test</h1>
            <p class=style::bodyText>Lorem ipld be distracting.</p>
        </div>
    }
}
