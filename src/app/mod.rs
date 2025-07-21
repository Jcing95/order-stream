use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment, 
    // ParamSegment,
    // params::Params,
    // hooks::use_params,
};

pub mod components;
pub mod pages;
pub mod states;

use pages::{signin::SignIn, signup::SignUp};

// use crate::frontend::pages::admin::AdminPage;
// use crate::frontend::pages::home::Home;
// use crate::frontend::pages::design_system::DesignSystemPage;
// use crate::frontend::pages::cashier::CashierPage;
// use crate::frontend::pages::station::{DynamicStationPage, StationsOverviewPage};
// use crate::frontend::state::theme::ThemeState;
// use crate::frontend::design_system::{Theme, ThemeContext, Navbar};

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
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    // Provide user state context
    states::user::provide();
    
    // Initialize the old theme state system (for compatibility)
    // let theme_state = ThemeState::new();
    // provide_context(theme_state);

    // Initialize design system theme based on old theme state (untracked for initialization)
    // let initial_theme = if theme_state.is_dark().get_untracked() {
    //     Theme::dark()
    // } else {
    //     Theme::light()
    // };
    // ThemeContext::provide(initial_theme);

    // Sync the old theme state with the design system theme
    // Effect::new(move |_| {
    //     let is_dark = theme_state.is_dark().get();
    //     let new_theme = if is_dark {
    //         Theme::dark()
    //     } else {
    //         Theme::light()
    //     };
    //     ThemeContext::set_theme(new_theme);
    // });

    // Create reactive page background based on design system theme
    // let page_bg_class = Signal::derive(move || {
    //     let theme = ThemeContext::use_theme().get();
    //     format!("min-h-screen transition-colors duration-200 {}", theme.colors.background.page)
    // });

    view! {
        // <div class=move || page_bg_class.get()>
            <Router>
                // <Navbar />
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=Home/>
                    <Route path=StaticSegment("signin") view=SignIn/>
                    <Route path=StaticSegment("signup") view=SignUp/>
                    // <Route path=StaticSegment("admin") view=AdminPage/>
                    // <Route path=StaticSegment("design-system") view=DesignSystemPage/>
                    // <Route path=StaticSegment("cashier") view=CashierPage/>
                    // <Route path=StaticSegment("stations") view=StationsOverviewPage/>                    
                    // // Dynamic station routes (database-driven)
                    // <Route path=(StaticSegment("stations"), ParamSegment("name")) view=DynamicStationRoute/>
                </FlatRoutes>
            </Router>
        // </div>
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
