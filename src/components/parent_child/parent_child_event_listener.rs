use crate::components::parent_child::TW_BUTTON;
use leptos::prelude::*;

#[component]
pub fn Child(
    // counter: ReadSignal<i32>,
    button_label: String,
) -> impl IntoView {
    let tw_button_class = view! { <{..} class=&*TW_BUTTON /> };

    view! {
        <div>
            <button {tw_button_class.clone()}>{button_label}</button>
        </div>
    }
}

#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = signal(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);
    let tw_button_class = view! { <{..} class=&*TW_BUTTON /> };

    view! {
        <div>
            <div class="border border-solid border-black m-1">
                <div class="m-2">
                    <h1 class="text-gray-600 text-xl font-bold">
                        Event Listener - Parent Component
                    </h1>
                    <p class="text-gray-600 text-xl">
                        "Counter: " <span class="text-red-500 font-bold">{counter}</span>
                    </p>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-y-4 p-4">
                        <div>
                            <button {tw_button_class.clone()} on:click=increment_counter>
                                "Parent +1"
                            </button>
                        </div>
                        <div>
                            <button {tw_button_class.clone()} on:click=decrement_counter>
                                "Parent -1"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="border border-solid border-black m-1">
                <div class="m-2">
                    <h1 class="text-gray-600 text-xl font-bold">
                        Event Listener - Child Component
                    </h1>
                    <p class="text-gray-600 text-xl">
                        "Counter: " <span class="text-red-500 font-bold">{counter}</span>
                    </p>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-y-4 p-4">
                        <Child
                            // counter=counter
                            on:click=move |_| set_counter.update(|c| *c += 1)
                            button_label="Child +1".to_owned()
                        />
                        <Child
                            // counter=counter
                            on:click=move |_| set_counter.update(|c| *c -= 1)
                            button_label="Child -1".to_owned()
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
