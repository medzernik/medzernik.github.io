use leptos::prelude::*;
use stylance::import_crate_style;
import_crate_style!(style, "src/main.module.css");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class=style::header>
            <p>"Medzernik's Dev Blog"</p>
        </div>
    }
}
#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class=style::topBar>
            <button class=style::topBarButton>HOME</button>
            <button class=style::topBarButton>POSTS</button>
            <button class=style::topBarButton>ABOUT</button>
        </div>
    }
}
