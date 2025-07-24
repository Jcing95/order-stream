use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    ParamSegment, StaticSegment,
};

pub mod components;
pub mod pages;

pub mod states;

use components::{
    navbar::Navbar, route_guard::RouteGuard, state_provider::StateProvider, ws_bridge::WsBridge,
};
use pages::{
    admin::Admin, cashier::Cashier, signin::SignIn, signup::SignUp, station::StationPage,
    stations::StationsPage, home::Home,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no"/>
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
                    <Route path=StaticSegment("cashier") view=move || view! {
                        <RouteGuard roles=vec![crate::common::types::Role::Admin, crate::common::types::Role::Cashier] children=move || view! {
                            <Cashier/>
                        }.into_any() />
                    }/>
                    <Route path=StaticSegment("station") view=move || view! {
                        <RouteGuard roles=vec![crate::common::types::Role::Admin, crate::common::types::Role::Cashier, crate::common::types::Role::Staff] children=move || view! {
                            <StationsPage/>
                        }.into_any() />
                    }/>
                    <Route path=StaticSegment("stations") view=move || view! {
                        <RouteGuard roles=vec![crate::common::types::Role::Admin, crate::common::types::Role::Cashier, crate::common::types::Role::Staff] children=move || view! {
                            <StationsPage/>
                        }.into_any() />
                    }/>
                    <Route path=(StaticSegment("station"), ParamSegment("id")) view=move || view! {
                        <RouteGuard roles=vec![crate::common::types::Role::Admin, crate::common::types::Role::Cashier, crate::common::types::Role::Staff] children=move || view! {
                            <StationPage/>
                        }.into_any() />
                    }/>
                </FlatRoutes>
            </Router>
        }.into_any() />
    }
}
