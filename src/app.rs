use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment, ParamSegment,
    params::Params,
    hooks::use_params,
};
use crate::frontend::pages::admin::AdminPage;
use crate::frontend::pages::home::Home;
use crate::frontend::pages::design_system::DesignSystemPage;
use crate::frontend::pages::cashier::CashierPage;
use crate::frontend::pages::station::{DynamicStationPage, StationsOverviewPage};
use crate::frontend::pages::login::LoginPage;
use crate::frontend::state::auth::provide_auth_context;
use crate::frontend::design_system::{Theme, ThemeContext, Navbar};
use crate::frontend::components::route_guard::{RouteGuard, RouteRequirement};
use crate::common::types::UserRole;

// Import server functions to ensure they're registered
#[allow(unused_imports)]
use crate::backend::services::auth::{
    register_user, login_user, logout_user, get_current_user,
    revoke_user_sessions, admin_lock_user_account, unlock_user_account,
    get_user_security_info, cleanup_expired_sessions, initialize_database_schema
};

#[allow(unused_imports)]
use crate::backend::services::items::{
    get_items, create_item, get_item, update_item, delete_item
};

#[allow(unused_imports)]
use crate::backend::services::categories::{
    get_categories, create_category, get_category, update_category, delete_category
};

#[allow(unused_imports)]
use crate::backend::services::orders::{
    get_orders, create_order, get_order, update_order, update_order_status, delete_order
};

#[allow(unused_imports)]
use crate::backend::services::order_items::{
    get_order_items, get_all_order_items, create_order_item, get_order_item,
    update_order_item, delete_order_item, bulk_update_order_items
};

#[allow(unused_imports)]
use crate::backend::services::stations::{
    get_stations, create_station, get_station, get_station_by_name,
    update_station, delete_station
};

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
    
    // Initialize authentication context
    let auth_state = provide_auth_context();
    
    // Initialize auth on app startup using proper Leptos isomorphic Effect pattern
    Effect::new_isomorphic({
        let auth_state = auth_state.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            {
                let auth_state = auth_state.clone();
                leptos::task::spawn_local(async move {
                    auth_state.initialize().await;
                });
            }
        }
    });
    
    // Initialize enhanced theme system with default light theme
    ThemeContext::provide(Theme::light());

    // Create reactive page background based on design system theme
    let page_bg_class = Signal::derive(move || {
        let theme = ThemeContext::use_theme().get();
        format!("min-h-screen transition-colors duration-200 {}", theme.colors.background.page)
    });

    view! {
        <div class=move || page_bg_class.get()>
            <Router>
                <Navbar />
                <FlatRoutes fallback=|| "Page not found.">
                    // Public routes (no auth required)
                    <Route path=StaticSegment("") view=Home/>
                    
                    // Public route that redirects authenticated users away
                    <Route path=StaticSegment("signin") view=ProtectedLoginPage/>
                    
                    // Protected routes with role-based access
                    <Route path=StaticSegment("admin") view=ProtectedAdminPage/>
                    <Route path=StaticSegment("cashier") view=ProtectedCashierPage/>
                    <Route path=StaticSegment("stations") view=ProtectedStationsPage/>
                    
                    // Dynamic station routes (database-driven) - all authenticated users
                    <Route path=(StaticSegment("stations"), ParamSegment("name")) view=DynamicStationRoute/>
                    
                    // Design system page - protected but available to all authenticated users
                    <Route path=StaticSegment("design-system") view=ProtectedDesignSystemPage/>
                </FlatRoutes>
            </Router>
        </div>
    }
}

// Route handler for dynamic station URLs /stations/:name
#[derive(Params, PartialEq, Clone)]
struct StationParams {
    name: String,
}

#[component]
fn DynamicStationRoute() -> impl IntoView {
    let params = use_params::<StationParams>();
    
    view! {
        <RouteGuard requirement=RouteRequirement::Authenticated children=move || {
            match params.with(|params| params.clone()) {
                Ok(StationParams { name }) => {
                    // Convert URL-friendly name back to potential station names
                    // URLs are generated as lowercase with spaces replaced by hyphens
                    // So we need to try both the URL format and converting back
                    let converted_name = name.replace("-", " ");
                    
                    view! {
                        <DynamicStationPage station_name=converted_name />
                    }.into_any()
                },
                Err(_) => {
                    view! {
                        <div class="container mx-auto p-6">
                            <div class="text-center">
                                <h1 class="text-2xl font-bold text-red-600 mb-4">
                                    "Invalid Station Route"
                                </h1>
                                <p class="text-gray-600">
                                    "Station name is required in the URL."
                                </p>
                            </div>
                        </div>
                    }.into_any()
                }
            }
        } />
    }
}

// Protected route wrapper components
#[component]
fn ProtectedLoginPage() -> impl IntoView {
    view! {
        <RouteGuard requirement=RouteRequirement::NotAuthenticated children=|| {
            view! { <LoginPage/> }.into_any()
        } />
    }
}

#[component]
fn ProtectedAdminPage() -> impl IntoView {
    view! {
        <RouteGuard requirement=RouteRequirement::Role(UserRole::Admin) children=|| {
            view! { <AdminPage/> }.into_any()
        } />
    }
}

#[component]
fn ProtectedCashierPage() -> impl IntoView {
    view! {
        <RouteGuard requirement=RouteRequirement::AnyRole(vec![UserRole::Admin, UserRole::Cashier]) children=|| {
            view! { <CashierPage/> }.into_any()
        } />
    }
}

#[component]
fn ProtectedStationsPage() -> impl IntoView {
    view! {
        <RouteGuard requirement=RouteRequirement::Authenticated children=|| {
            view! { <StationsOverviewPage/> }.into_any()
        } />
    }
}

#[component]
fn ProtectedDesignSystemPage() -> impl IntoView {
    view! {
        <RouteGuard requirement=RouteRequirement::Authenticated children=|| {
            view! { <DesignSystemPage/> }.into_any()
        } />
    }
}
