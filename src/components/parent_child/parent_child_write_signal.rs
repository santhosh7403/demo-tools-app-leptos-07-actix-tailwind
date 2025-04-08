use crate::components::parent_child::TW_BUTTON;
use leptos::prelude::*;

#[component]
pub fn Child(counter: ReadSignal<i32>, set_counter: WriteSignal<i32>) -> impl IntoView {
    let increment = move |_| set_counter.update(|c| *c += 1);
    let decrement = move |_| set_counter.update(|c| *c -= 1);
    let tw_button_class = view! { <{..} class=&*TW_BUTTON /> };

    view! {
        <div class="border border-solid border-black m-1">
            <div class="m-2">
                <h1 class="text-gray-600 text-xl font-bold">Write Signal - Child Component</h1>
                <p class="text-gray-600 text-xl">
                    "Counter: " <span class="text-red-500 font-bold">{counter}</span>
                </p>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-y-4 p-4">
                    <div>
                        <button {tw_button_class.clone()} on:click=increment>
                            "Child +1"
                        </button>
                    </div>
                    <div>
                        <button {tw_button_class.clone()} on:click=decrement>
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

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);
    let tw_button_class = view! { <{..} class=&*TW_BUTTON /> };

    view! {
        <div class="border border-solid border-black m-1">
            <div class="m-2">
                <h1 class="text-gray-600 text-xl font-bold">Write Signal - Parent Component</h1>
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
        <Child counter=counter set_counter=set_counter />
    }
}
