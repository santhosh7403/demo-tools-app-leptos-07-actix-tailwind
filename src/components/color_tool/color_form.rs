use leptos::prelude::*;

use crate::repositories::color_repository::AppendColor;

#[component]
pub fn ColorForm(append_color: ServerAction<AppendColor>) -> impl IntoView {
    view! {
        <ActionForm action=append_color>
            // Card container
            <div class="shadow overflow-hidden sm:rounded-md">
                // Card content
                <div class="px-4 py-5 bg-white sm:p-6">
                    // Overall Grid Layout
                    <div class="grid grid-cols-6 gap-6">

                        // Color Name Field
                        <div class="col-span-6 sm:col-span-3">
                            <label for="name" class="block text-sm font-medium text-gray-700">
                                "Color Name:"
                            </label>
                            <input
                                type="text"
                                name="name"
                                id="name"
                                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                                required=true
                            />
                        </div>

                        // Hexcode Field
                        <div class="col-span-6 sm:col-span-3">
                            <label for="hexcode" class="block text-sm font-medium text-gray-700">
                                "Hexcode:"
                            </label>
                            <input
                                type="text"
                                name="hexcode"
                                id="hexcode"
                                class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                                required=true
                            />
                        </div>
                    </div>
                </div>
                <div class="px-4 py-3 bg-gray-50 text-right sm:px-6">
                    <button
                        type="submit"
                        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500"
                    >
                        "Add Color"
                    </button>
                </div>
            </div>
        </ActionForm>
    }
}
