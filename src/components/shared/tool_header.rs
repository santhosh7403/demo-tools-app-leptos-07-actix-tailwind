use leptos::prelude::*;

#[component]
pub fn ToolHeader(header: String) -> impl IntoView {
    view! {
        <header>
            <h1 class="text-green-600 text-3xl font-bold">{header}</h1>
        </header>
    }
}
