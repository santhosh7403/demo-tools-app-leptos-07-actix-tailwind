use leptos::{ev::MouseEvent, prelude::*};

use crate::components::parent_child::TW_BUTTON;

#[component]
pub fn Child(
    counter: ReadSignal<i32>,
    on_increment: impl Fn(MouseEvent) + 'static,
    on_decrement: impl Fn(MouseEvent) + 'static,
) -> impl IntoView {
    let tw_button_class = view! { <{..} class=&*TW_BUTTON /> };

    view! {
        <div class="border border-solid border-black m-1">
            <div class="m-2">
                <h1 class="text-gray-600 text-xl font-bold">Callback - Child Component</h1>
                <p class="text-gray-600 text-xl">
                    "Counter: " <span class="text-red-500 font-bold">{counter}</span>
                </p>
                // <div class="m-[5px] space-x-1">
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-y-4 p-4">
                    <div>
                        <button {tw_button_class.clone()} on:click=on_increment>
                            "Child +1"
                        </button>
                    </div>
                    <div>
                        <button {tw_button_class.clone()} on:click=on_decrement>
                            "Child -1"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let tw_button_class = view! { <{..} class=&*TW_BUTTON /> };
    // let tw_button_class = view! {<{..} class="bg-gray-500 text-white mx-4 px-4 py-2 rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 active:bg-gray-700"/>};

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div class="border border-solid border-black m-1">
            <div class="m-2">
                <h1 class="text-gray-600 text-xl font-bold">Callback - Parent Component</h1>
                <p class="text-gray-600 text-xl">
                    "Counter: " <span class="text-red-500 font-bold">{counter}</span>
                </p>
                // <div class="m-[5px] space-x-1">
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
        <Child counter=counter on_increment=increment_counter on_decrement=decrement_counter />
    }
}
