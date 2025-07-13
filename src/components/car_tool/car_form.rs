use leptos::prelude::*;

use crate::repositories::car_repository::AppendCar;

#[allow(dead_code)]
#[component]
pub fn CarForm(append_car: ServerAction<AppendCar>) -> impl IntoView {
    view! {
        <div class="border border-solid border-black m-1">
            <div class="m-2">
                <h1 class="text-gray-600 text-base font-bold">Add Form</h1>

                <div class="shadow overflow-hidden sm:rounded-md mt-4">
                    <ActionForm action=append_car>
                        // Card Content
                        <div class="px-4 py-5 bg-white sm:p-6 grid grid-cols-1 gap-6">
                            // Make
                            <div class="col-span-1">
                                <label for="make" class="block text-sm font-medium text-gray-700">
                                    "Make:"
                                </label>
                                <input
                                    type="text"
                                    name="make"
                                    id="make"
                                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md capitalize"
                                />
                            </div>

                            // Model
                            <div class="col-span-1">
                                <label for="model" class="block text-sm font-medium text-gray-700">
                                    "Model:"
                                </label>
                                <input
                                    type="text"
                                    name="model"
                                    id="model"
                                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md capitalize"
                                />
                            </div>
                            // Year
                            <div class="col-span-1">
                                <label for="year" class="block text-sm font-medium text-gray-700">
                                    "Year:"
                                </label>
                                <input
                                    type="number"
                                    name="year"
                                    id="year"
                                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                                />
                            </div>
                            // Color
                            <div class="col-span-1">
                                <label for="color" class="block text-sm font-medium text-gray-700">
                                    "Color:"
                                </label>
                                <input
                                    type="text"
                                    name="color"
                                    id="color"
                                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md capitalize"
                                />
                            </div>
                            // Price
                            <div class="col-span-1">
                                <label for="price" class="block text-sm font-medium text-gray-700">
                                    "Price:"
                                </label>
                                <input
                                    type="number"
                                    name="price"
                                    id="price"
                                    class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                                />
                            </div>
                        </div>
                        <div class="px-4 py-3 bg-gray-50 text-right sm:px-6">
                            <button
                                type="submit"
                                class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500"
                            >
                                "Add Car"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}
