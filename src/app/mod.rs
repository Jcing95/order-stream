use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment, 
};

pub mod components;
pub mod pages;

pub mod states;

use pages::{signin::SignIn, signup::SignUp};
use components::{navbar::Navbar, ws_bridge::WsBridge, route_guard::RouteGuard, state_provider::StateProvider};

use crate::app::pages::admin::Admin;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/order-stream.css"/>
                <link rel="icon" type="image/svg+xml" href="/icon.svg"/>
                <MetaTags/>
            </head>
            <body class = "bg-back">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <StateProvider children=move || view! {
            <Router>
                <Navbar />
                <WsBridge />
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=Home/>
                    <Route path=StaticSegment("signin") view=SignIn/>
                    <Route path=StaticSegment("signup") view=SignUp/>
                    <Route path=StaticSegment("admin") view=move || view! {
                        <RouteGuard roles=vec![crate::common::types::Role::Admin] children=move || view! {
                            <Admin/>
                        }.into_any() />
                    }/>
                </FlatRoutes>
            </Router>
        }.into_any() />
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        Hello World!
    }
}

// // Route handler for dynamic station URLs /stations/:name
// #[derive(Params, PartialEq, Clone)]
// struct StationParams {
//     name: String,
// }

// #[component]
// fn DynamicStationRoute() -> impl IntoView {
//     let params = use_params::<StationParams>();
    
//     view! {
//         {move || {
//             match params.with(|params| params.clone()) {
//                 Ok(StationParams { name }) => {
//                     // Convert URL-friendly name back to potential station names
//                     // URLs are generated as lowercase with spaces replaced by hyphens
//                     // So we need to try both the URL format and converting back
//                     let converted_name = name.replace("-", " ");
                    
//                     view! {
//                         <DynamicStationPage station_name=converted_name />
//                     }.into_any()
//                 },
//                 Err(_) => {
//                     view! {
//                         <div class="container mx-auto p-6">
//                             <div class="text-center">
//                                 <h1 class="text-2xl font-bold text-red-600 mb-4">
//                                     "Invalid Station Route"
//                                 </h1>
//                                 <p class="text-gray-600">
//                                     "Station name is required in the URL."
//                                 </p>
//                             </div>
//                         </div>
//                     }.into_any()
//                 }
//             }
//         }}
//     }
// }
