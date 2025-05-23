use crate::components::header::{Header, TopBar};
use leptos::{html::inner_html, prelude::*};
use leptos_reactive::{SignalGet, SignalWith, create_signal};
use markdown::to_html;
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
    let (test, set_test) = signal("# Hello, Markdown!\n\nThis is a **bold** text.");

    view! { <div class=style::bodyText inner_html=move || to_html(test.get())></div> }
}
