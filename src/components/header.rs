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
            <a href="home" class=style::topBarButton>
                home
            </a>
            <a href="posts" class=style::topBarButton>
                posts
            </a>
            <a href="about" class=style::topBarButton>
                about
            </a>
        </div>
    }
}
