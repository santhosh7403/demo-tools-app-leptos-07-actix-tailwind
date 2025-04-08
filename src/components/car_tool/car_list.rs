use crate::components::errors_fallback::error_fallback;
use leptos::prelude::*;

use crate::models::car::Car;
use crate::repositories::car_repository::{RemoveCar, UpdateCar};

#[component]
fn CarViewRow(
    car: Car,
    remove_car: ServerAction<RemoveCar>,
    set_edit_car: WriteSignal<Option<String>>,
) -> impl IntoView {
    let car_id = car.id.clone();
    view! {
        <tr>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.id.clone()}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.make}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.model}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.year}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.color}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.price}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                <ActionForm action=remove_car>
                    <input type="hidden" name="id" value=car.id />
                    <button
                        class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded-md focus:outline-none focus:shadow-outline text-xs"
                        type="submit"
                    >
                        "X"
                    </button>
                </ActionForm>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded-md focus:outline-none focus:shadow-outline text-xs"
                    type="button"
                    on:click=move |_| { set_edit_car.set(Some(car_id.clone())) }
                >
                    "Edit"
                </button>
            </td>
        </tr>
    }
}

#[component]
fn CarEditRow(car: Car, set_edit_car: WriteSignal<Option<String>>) -> impl IntoView {
    let (make, set_make) = signal(car.make.clone());
    let (model, set_model) = signal(car.model.clone());
    let (color, set_color) = signal(car.color.clone());
    let (year, set_year) = signal(car.year.to_string());
    let (price, set_price) = signal(car.price.to_string());

    let car_id = car.id.clone();

    let update_car = expect_context::<ServerAction<UpdateCar>>();

    let save_car = move || {
        update_car.dispatch(UpdateCar {
            id: car_id.clone(),
            make: make.get(),
            model: model.get(),
            year: year.get().parse::<u16>().unwrap(),
            color: color.get(),
            price: price.get().parse::<f32>().unwrap(),
        });
        set_edit_car.update(|edit_car_id| *edit_car_id = None);
    };

    view! {
        <tr>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{car.id.clone()}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <input
                    type="text"
                    name="make"
                    class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
                    value=make
                    on:input=move |e| { set_make.set(event_target_value(&e)) }
                />
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <input
                    type="text"
                    name="model"
                    class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
                    value=model
                    on:input=move |e| { set_model.set(event_target_value(&e)) }
                />
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <input
                    type="number"
                    name="year"
                    class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
                    value=year
                    on:input=move |e| { set_year.set(event_target_value(&e)) }
                />
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <input
                    type="text"
                    name="color"
                    class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
                    value=color
                    on:input=move |e| { set_color.set(event_target_value(&e)) }
                />
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <input
                    type="text"
                    name="price"
                    class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
                    value=price
                    on:input=move |e| { set_price.set(event_target_value(&e)) }
                />
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                <button
                    class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded-md focus:outline-none focus:shadow-outline text-xs"
                    on:click=move |_| { save_car() }
                >
                    "Save"
                </button>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                <button
                    class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-1 px-2 rounded-md focus:outline-none focus:shadow-outline text-xs"
                    type="button"
                    on:click=move |_| { set_edit_car.set(None) }
                >
                    "Cancel"
                </button>
            </td>
        </tr>
    }
}

#[component]
pub fn CarList(
    car_resource: Resource<Result<Vec<Car>, ServerFnError>>,
    remove_car: ServerAction<RemoveCar>,
) -> impl IntoView {
    let (edit_car_id, set_edit_car_id) = signal::<Option<String>>(None);

    let car_rows = move || {
        car_resource.and_then(|cars| {
            cars.iter()
                .map(|car| {
                    if edit_car_id
                        .get()
                        .map(|id| id == car.id.to_string())
                        .unwrap_or(false)
                    {
                    view! { <CarEditRow car=car.clone() set_edit_car=set_edit_car_id /> }.into_any()

                    } else {
                    view! { <CarViewRow car=car.clone() remove_car=remove_car set_edit_car=set_edit_car_id /> }.into_any()
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="container mx-auto py-4">
            // Styled loading text
            <Transition fallback=move || view! { <p class="text-gray-600">"Loading..."</p> }>
                <ErrorBoundary fallback=error_fallback>
                    // Edit Car Id Display
                    <div class="mb-4 text-gray-700">
                        "Edit Car Id: "
                        {move || edit_car_id.get().unwrap_or_else(|| "None".to_string())}
                    </div>

                    // Table styling
                    <table class="min-w-full divide-y divide-gray-200 shadow overflow-hidden rounded-md">
                        // Table header
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Id
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Make
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Model
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Year
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Color
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Price
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Remove
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                    Edit
                                </th>
                            </tr>
                        </thead>
                        // Table body
                        <tbody class="bg-white divide-y divide-gray-200">{car_rows}</tbody>
                    </table>

                </ErrorBoundary>
            </Transition>
        </div>
    }
}
