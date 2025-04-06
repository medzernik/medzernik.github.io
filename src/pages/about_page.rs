use std::{path::PathBuf, str::FromStr};

use crate::{
    app::load_md_file,
    components::header::{Header, TopBar},
};
use leptos::prelude::*;

use leptos_reactive::SignalWith;
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
    let html = leptos_reactive::create_resource(
        || (),
        |_| async move {
            markdown::to_html(
                &load_md_file(PathBuf::from_str("/public/content/about/about.md").unwrap()).await,
            )
        },
    );

    view! {
        <div class=style::bodyContainer>
            <Suspense fallback=|| {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    html.with(|data| {
                        view! { <div class=style::bodyText inner_html=data.clone().unwrap()></div> }
                    })
                }},

            </Suspense>
        </div>
    }
}
