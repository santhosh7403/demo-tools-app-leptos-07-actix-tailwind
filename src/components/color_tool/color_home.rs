use crate::components::color_tool::{
    color_form::ColorForm, color_list::ColorList, color_modal::ColorModal, ShowColorModalSignal,
};
use crate::components::shared::tool_header::ToolHeader;
use crate::repositories::color_repository::{all_colors, AppendColor, RemoveColor};
use leptos::prelude::*;

#[component]
pub fn ColorHome() -> impl IntoView {
    let append_color = ServerAction::<AppendColor>::new();
    let remove_color = ServerAction::<RemoveColor>::new();

    let color_resource = Resource::new(
        move || (append_color.version().get(), remove_color.version().get()),
        |_| all_colors(),
    );

    view! {
        <div>
            <ToolHeader header="Color - Home".to_owned() />
            <ColorList color_resource=color_resource remove_color=remove_color />
            <ColorForm append_color=append_color />
            <ColorAddForm append_color=append_color />
        </div>
    }
}

#[component]
fn ColorAddForm(append_color: ServerAction<AppendColor>) -> impl IntoView {
    let show_modal: ShowColorModalSignal = RwSignal::new(false);

    let on_show_modal_add_event = move |_| {
        show_modal.set(true);
    };

    let on_add_add_event = move |name, hexcode| {
        show_modal.set(false);
        append_color.dispatch(AppendColor { name, hexcode });
        // leptos::logging::log!(" Values = {:?}", c);
    };

    let on_cancel_add_event = move || {
        show_modal.set(false);
    };
    view! {
        <div class="container mx-auto m-5 p-6">
            <div class="mb-4 text-xl font-extrabold text-center text-gray-600 pb-5">
                Add a new Color:
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
            <ColorModal on_add=on_add_add_event on_cancel=on_cancel_add_event />
        </Show>
    }
}
