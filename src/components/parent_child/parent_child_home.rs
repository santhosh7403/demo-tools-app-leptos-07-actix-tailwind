use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

use crate::components::shared::tool_header::ToolHeader;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    let tw_a_class = view! { <{..} class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600" /> };
    view! {
        <ToolHeader header="Parent Child - Home".to_owned() />
        <div>
            // <h1 class="text-gray-600 text-3xl font-bold">"Parent Child Home"</h1>
            <ul>
                <li>
                    <a {tw_a_class.clone()} href="/parent-child/write-signal">
                        "Write Signal"
                    </a>
                </li>
                <li>
                    <a {tw_a_class.clone()} href="/parent-child/event-listener">
                        "Event-listener"
                    </a>
                </li>
                <li>
                    <a {tw_a_class.clone()} href="/parent-child/callback-child">
                        "Callback passing to child"
                    </a>
                </li>
                <li>
                    <a {tw_a_class.clone()} href="/parent-child/context-child">
                        "Context passing to child"
                    </a>
                </li>
            </ul>
            <Outlet />
        </div>
    }
}
