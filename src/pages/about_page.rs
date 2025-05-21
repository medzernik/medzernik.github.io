use crate::components::header::{Header, TopBar};
use leptos::prelude::*;
use leptos_reactive::{SignalGet, SignalWith, create_signal};
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
    // let (data, set_data) =
    //     create_signal("# Hello, Markdown!\n\nThis is a **bold** text.".to_string());

    let text = move || "# Hello, Markdown!\n\nThis is a **bold** text.".to_string();

    // Effect::new(move |_| data.get());

    view! { <div class=style::bodyContainer>{move || { test.get() }}</div> }
}
