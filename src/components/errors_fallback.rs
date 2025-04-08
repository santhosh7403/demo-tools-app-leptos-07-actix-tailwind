use leptos::prelude::*;

pub fn error_fallback(errors: ArcRwSignal<Errors>) -> impl IntoView {
    view! {
        <div>
            <p>"Error!!!"</p>
            <ul>
                {move || {
                    errors
                        .get()
                        .into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                        .collect::<Vec<_>>()
                }}
            </ul>
        </div>
    }
}
