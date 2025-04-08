use leptos::prelude::*;

use crate::components::car_tool::{car_list::CarList, car_modal::CarModal, ShowCarModalSignal};
use crate::components::shared::tool_header::ToolHeader;
use crate::repositories::car_repository::{all_cars, AppendCar, RemoveCar, UpdateCar};

#[component]
pub fn CarHome() -> impl IntoView {
    let append_car = ServerAction::<AppendCar>::new();
    let remove_car = ServerAction::<RemoveCar>::new();
    let update_car = ServerAction::<UpdateCar>::new();

    let car_resource = Resource::new(
        move || {
            (
                append_car.version().get(),
                remove_car.version().get(),
                update_car.version().get(),
            )
        },
        |_| all_cars(),
    );

    provide_context(update_car);

    view! {
        <div>
            <ToolHeader header="Car - Home".to_owned() />
            <CarList car_resource=car_resource remove_car=remove_car />
            // <CarForm append_car=append_car />
            <CarAddForm append_car=append_car />
        </div>
    }
}

#[component]
fn CarAddForm(append_car: ServerAction<AppendCar>) -> impl IntoView {
    let show_modal: ShowCarModalSignal = RwSignal::new(false);

    let on_show_modal_add_event = move |_| {
        show_modal.set(true);
    };

    let on_add_add_event = move |car: AppendCar| {
        show_modal.set(false);
        append_car.dispatch(car);
    };

    let on_cancel_add_event = move || {
        show_modal.set(false);
    };

    view! {
        <div class="container mx-auto m-5 p-6">
            <div class="mb-4 text-xl font-extrabold text-center text-gray-600 pb-5">
                Add a new Car:
                <button
                    on:click=on_show_modal_add_event
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mx-2"
                >
                    // <i class="bx bx-plus"></i>
                    <i class="fa-solid fa-plus"></i>
                </button>
            </div>
        </div>
        <Show when=move || show_modal.get()>
            <CarModal on_add=on_add_add_event on_cancel=on_cancel_add_event />
        </Show>
    }
}
