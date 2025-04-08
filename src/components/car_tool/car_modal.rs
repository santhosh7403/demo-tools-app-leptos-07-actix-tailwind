use leptos::{html::Input, prelude::*};

use crate::repositories::car_repository::AppendCar;

#[component]
pub fn CarModal<A, C>(on_add: A, on_cancel: C) -> impl IntoView
where
    A: Fn(AppendCar) + 'static + Send,
    C: Fn() + 'static + Send,
{
    let car_make: NodeRef<Input> = NodeRef::new();
    let car_model: NodeRef<Input> = NodeRef::new();
    let car_year: NodeRef<Input> = NodeRef::new();
    let car_color: NodeRef<Input> = NodeRef::new();
    let car_price: NodeRef<Input> = NodeRef::new();

    let on_add_event = move |_| {
        let make = car_make.get().expect("<input> to exist").value();
        let model = car_model.get().expect("<input> to exist").value();
        let year = car_year
            .get()
            .expect("<input> to exist")
            .value()
            .parse::<u16>()
            .unwrap();
        let color = car_color.get().expect("<input> to exist").value();
        let price = car_price
            .get()
            .expect("<input> to exist")
            .value()
            .parse::<f32>()
            .unwrap();

        let new_car = AppendCar {
            make,
            model,
            year,
            color,
            price,
        };
        on_add(new_car);
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900 bg-opacity-60">

            <div class="block rounded-lg bg-white w-2/5 p-4 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] z-70">

                <h5 class="mb-5 text-xl font-medium leading-tight text-neutral-800">
                    Add a new Car
                </h5>
                <form>
                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="Make">
                            Make
                        </label>
                        <input
                            node_ref=car_make
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="make"
                            type="text"
                            value=move || { String::new() }
                            placeholder="Car make"
                            required=true
                        />
                    </div>
                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="Model">
                            Model
                        </label>
                        <input
                            node_ref=car_model
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="model"
                            type="text"
                            value=move || { String::new() }
                            placeholder="Car model"
                            required=true
                        />
                    </div>

                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="year">
                            Year
                        </label>
                        <input
                            node_ref=car_year
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="year"
                            type="number"
                            // value=move || { String::new() }
                            placeholder="Year"
                            required=true
                        />
                    </div>
                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="color">
                            Color
                        </label>
                        <input
                            node_ref=car_color
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="color"
                            type="text"
                            value=move || { String::new() }
                            placeholder="Car color"
                            required=true
                        />
                    </div>
                    <div class="mb-5">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="price">
                            Price
                        </label>
                        <input
                            node_ref=car_price
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="price"
                            type="number"
                            // value=move || { String::new() }
                            placeholder="Car price"
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
