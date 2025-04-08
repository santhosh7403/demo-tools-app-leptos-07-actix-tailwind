use leptos::{html::Input, prelude::*};

#[component]
pub fn ColorModal<A, C>(on_add: A, on_cancel: C) -> impl IntoView
where
    A: Fn(String, String) + 'static + Send,
    C: Fn() + 'static + Send,
{
    let color_name: NodeRef<Input> = NodeRef::new();
    let color_hexcode: NodeRef<Input> = NodeRef::new();

    let on_add_event = move |_| {
        let name = color_name.get().expect("<input> to exist").value();
        let hexcode = color_hexcode.get().expect("<input> to exist").value();

        on_add(name, hexcode);
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900 bg-opacity-60">

            <div class="block rounded-lg bg-white w-2/5 p-4 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] z-70">

                <h5 class="mb-5 text-xl font-medium leading-tight text-neutral-800">
                    Add a new Color
                </h5>

                <form>
                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="name">
                            Name
                        </label>
                        <input
                            node_ref=color_name
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="name"
                            type="text"
                            value=move || { String::new() }
                            placeholder="Color name"
                            required=true
                        />
                    </div>
                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="hexcode">
                            Hexcode
                        </label>
                        <input
                            node_ref=color_hexcode
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="hexcode"
                            type="text"
                            value=move || { String::new() }
                            placeholder="Color hexcode"
                            required=true
                        />
                    </div>
                    <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                        <button
                            // type="button"
                            type="submit"
                            class="bg-blue-700 hover:bg-blue-800 px-5 py-3 text-white rounded-lg"
                            on:click=on_add_event
                        >
                            Add
                        </button>
                        <button
                            type="cancel"
                            class="bg-gray-300 hover:bg-gray-400 px-5 py-3 text-white rounded-lg"
                            on:click=move |_| on_cancel()
                        >
                            Cancel
                        </button>
                    </div>

                </form>

            </div>
        </div>
    }
}
