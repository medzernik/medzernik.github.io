use crate::pages::about_page::load_data;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use markdown::CompileOptions;
use markdown::{Options, to_html_with_options};
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.css");

#[component]
pub fn PostPageContent() -> impl IntoView {
    let params = use_params_map();

    let id = move || params.get().get("id").unwrap_or_default();
    //
    let async_data = LocalResource::new(move || {
        let path = format!("content/posts/{}.md", id());
        load_data(path)
    });
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
