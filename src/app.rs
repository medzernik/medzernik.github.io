use leptos::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| *set_count.write() += 1>"Click me"</button>
        // now we use our component!
        <ProgressBar progress=count />
    }
}

#[component]
fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max="50"
            // now this works
            value=progress
        />
    }
}
