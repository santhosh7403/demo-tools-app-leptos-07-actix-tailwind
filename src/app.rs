use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, StaticSegment, WildcardSegment,
};

use crate::components::parent_child::{
    parent_child_callback::Parent as CallbackParent,
    parent_child_event_listener::Parent as EventListenerParent, parent_child_home::ParentChildHome,
    parent_child_using_context::Parent as ContextParent,
    parent_child_write_signal::Parent as WriteSignalParent,
};

use crate::components::car_tool::car_home::CarHome;
use crate::components::color_tool::color_home::ColorHome;

#[component]
pub fn PageFooter() -> impl IntoView {
    view! {
        <footer class="sticky bottom-0 bg-gray-700 p-4 text-center text-sm text-gray-200">
            // <small>
            // <i class="bx bxs-copyright"></i>
            // 2025 Your Company Name. All rights reserved.
            // </small>
            <small>
                <i class="fa-solid fa-copyright"></i>
                2025 Your Company Name. All rights reserved.
            </small>
        </footer>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! { <aside class="bg-gray-800 p-4 text-gray-200">Sidebar</aside> }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="border-b border-gray-300 bg-gray-200">
            <ul class="m-0 flex h-full list-none items-center justify-start p-0">
                <li class="px-3 py-1">
                    <a
                        href="/"
                        class="text-gray-800 no-underline font-bold hover:text-amber-500 transition-colors duration-300"
                    >
                        "Home"
                    </a>
                </li>
                <li class="px-3 py-1">
                    <a
                        href="/parent-child"
                        class="text-gray-800 no-underline font-bold hover:text-amber-500 transition-colors duration-300"
                    >
                        "Parent Child"
                    </a>
                </li>
                <li class="px-3 py-1">
                    <a
                        href="/color-tool"
                        class="text-gray-800 no-underline font-bold hover:text-amber-500 transition-colors duration-300"
                    >
                        "Color Tool"
                    </a>
                </li>
                <li class="px-3 py-1">
                    <a
                        href="/car-tool"
                        class="text-gray-800 no-underline font-bold hover:text-amber-500 transition-colors duration-300"
                    >
                        "Car Home"
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
        <header class="flex items-center justify-between bg-gray-700 px-5 py-2 text-gray-200 shadow-md">
            <h1 class="text-blue-500 text-3xl font-bold">Tools App</h1>
        </header>
    }
}

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="h-full">{children()}</div> }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tailwind_actix.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main class="absolute inset-0 grid grid-cols-[1fr] grid-rows-[60px_40px_1fr_50px] gap-0 bg-gray-200">
                <Container>
                    <PageHeader />
                    <NavBar />
                    <div class="grid grid-cols-[1fr_6fr] min-h-screen">
                        // <div class="flex min-h-[500px]">
                        <SideBar />
                        <div class="bg-white p-4">
                            <Routes fallback=move || "Not found.">
                                <Route path=path!("/") view=HomePage />
                                <ParentRoute path=path!("parent-child") view=ParentChildHome>
                                    <Route path=path!("write-signal") view=WriteSignalParent />
                                    <Route path=path!("callback-child") view=CallbackParent />
                                    <Route path=path!("event-listener") view=EventListenerParent />
                                    <Route path=path!("context-child") view=ContextParent />
                                    <Route
                                        path=path!("")
                                        view=|| {
                                            view! {
                                                <div class="text-gray-600 font-bold p-36">
                                                    "Click a link"
                                                </div>
                                            }
                                        }
                                    />
                                </ParentRoute>
                                <Route path=StaticSegment("color-tool") view=ColorHome />
                                <Route path=StaticSegment("car-tool") view=CarHome />
                                <Route path=WildcardSegment("any") view=NotFound />
                            </Routes>
                        </div>
                    </div>
                    <PageFooter />
                </Container>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let tw_button_class = view! {
        <{..} class="bg-gray-500 text-white mx-4 px-4 py-2 rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 active:bg-gray-700" />
    };

    view! {
        <div>
            <h1 class="text-green-600 text-5xl font-bold">"Welcome to Leptos!"</h1>
            <div class="m-[20px]">
                <button {tw_button_class} on:click=on_click>
                    "Click Me: "
                    <span class="text-red-500 font-bold">{count}</span>
                </button>
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found View"</h1> }
}
