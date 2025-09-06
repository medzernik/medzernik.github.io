use leptos::{prelude::*, server_fn::request::browser::Request};
use markdown::{CompileOptions, Options, to_html_with_options};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <AboutPageContent />
        </div>
    }
}

#[component]
pub fn AboutPageContent() -> impl IntoView {
    let async_data = LocalResource::new(async move || load_data("content/about/about.md").await);

    view! {
        <div class=style::bodyContainer>
            <div
                class=style::bodyText
                inner_html=move || {
                    async_data
                        .get()
                        .map(|markdown_content| {
                            to_html_with_options(
                                    &markdown_content,
                                    &Options {
                                        compile: CompileOptions {
                                            allow_dangerous_html: false,
                                            allow_dangerous_protocol: false,
                                            ..CompileOptions::default()
                                        },
                                        ..Options::default()
                                    },
                                )
                                .unwrap_or_default()
                        })
                        .unwrap_or_default()
                }
            ></div>
        </div>
    }
}

pub async fn load_data(url: &str) -> String {
    // Assumes <link data-trunk rel="copy-dir" href="public/content/about" />
    // makes files from "public/content/about/" available under "/content/about/"
    // For example, if path is "about.md", the URL will be "/content/about/about.md"

    Request::get(&url)
        .send()
        .await
        .expect("Failed to fetch file from server.")
        .text()
        .await
        .expect("Failed to convert fetched file content to text.")
}
