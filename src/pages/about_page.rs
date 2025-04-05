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
                Lorem ipsum is a dummy or placeholder text commonly used in graphic design, publishing, and web development. Its purpose is to permit a page layout to be designed, independently of the copy that will subsequently populate it, or to demonstrate various fonts of a typeface without meaningful text that could be distracting.
            </p>
        </div>
    }
}
