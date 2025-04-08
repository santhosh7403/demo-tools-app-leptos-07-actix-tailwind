use crate::{
    components::errors_fallback::error_fallback, models::color::Color,
    repositories::color_repository::RemoveColor,
};
use leptos::prelude::*;

#[component]
pub fn ColorListItem(color: Color, remove_color: ServerAction<RemoveColor>) -> impl IntoView {
    view! {
        <tr>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                {color.name}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{color.hexcode}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <ActionForm action=remove_color>
                    <input type="hidden" name="id" value=color.id />
                    <button
                        class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded-md focus:outline-none focus:shadow-outline text-xs"
                        type="submit"
                    >
                        "X"
                    </button>
                </ActionForm>
            </td>
        </tr>
    }
}

#[component]
pub fn ColorList(
    color_resource: Resource<Result<Vec<Color>, ServerFnError>>,
    remove_color: ServerAction<RemoveColor>,
) -> impl IntoView {
    let color_list_view = move || {
        color_resource.and_then(|colors| {
            colors
                .iter()
                .map(|color| {
                    view! { <ColorListItem color=color.clone() remove_color=remove_color /> }
                })
                .collect_view()
        })
    };
    view! {
        <div class="border border-solid border-black m-1">
            <div class="m-2">

                <p class="text-lg font-semibold mb-4 text-gray-700">"Color List:"</p>
                <Transition fallback=move || view! { "Loading..." }>
                    <ErrorBoundary fallback=error_fallback>
                        <table class="min-w-full bg-white rounded-md shadow overflow-hidden">
                            <thead>
                                <tr class="bg-gray-50">
                                    <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                        Name
                                    </th>
                                    <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                        Hexcode
                                    </th>
                                    <th class="px-6 py-3 text-left text-xs font-bold text-gray-500 uppercase tracking-wider">
                                        Remove
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="bg-white divide-y divide-gray-200">
                                {color_list_view}
                            </tbody>
                        </table>
                    </ErrorBoundary>
                </Transition>
            </div>
        </div>
    }
}
