use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment, ParamSegment,
    params::Params,
    hooks::{use_params, use_navigate},
};
use crate::frontend::pages::admin::AdminPage;
use crate::frontend::pages::home::Home;
use crate::frontend::pages::design_system::DesignSystemPage;
use crate::frontend::pages::cashier::CashierPage;
use crate::frontend::pages::station::{DynamicStationPage, StationsOverviewPage};
use crate::frontend::pages::login::LoginPage;
use crate::frontend::state::theme::ThemeState;
use crate::frontend::state::auth::{provide_auth_context, use_auth_context};
use crate::frontend::design_system::{Theme, ThemeContext, Navbar, Spinner, theme::Size};
use crate::common::types::UserRole;

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
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
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
    
    // Initialize auth on app startup - only on client side
    let auth_state_init = auth_state.clone();
    Effect::new({
        let auth_state_init = auth_state_init.clone();
        move |_| {
            let auth_state_init = auth_state_init.clone();
            leptos::task::spawn_local(async move {
                auth_state_init.initialize().await;
            });
        }
    });
    
    // Initialize the old theme state system (for compatibility)
    let theme_state = ThemeState::new();
    provide_context(theme_state);

    // Initialize design system theme based on old theme state (untracked for initialization)
    let initial_theme = if theme_state.is_dark().get_untracked() {
        Theme::dark()
    } else {
        Theme::light()
    };
    ThemeContext::provide(initial_theme);

    // Sync the old theme state with the design system theme
    Effect::new(move |_| {
        let is_dark = theme_state.is_dark().get();
        let new_theme = if is_dark {
            Theme::dark()
        } else {
            Theme::light()
        };
        ThemeContext::set_theme(new_theme);
    });

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
    let auth = use_auth_context();
    let navigate = use_navigate();
    let params = use_params::<StationParams>();
    let user = auth.user();
    let is_loading = auth.is_loading();

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if !is_loading.get() && user.get().is_none() {
                navigate("/signin", Default::default());
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else if user.get().is_some() {
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
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}

// Protected route wrapper components
#[component]
fn ProtectedLoginPage() -> impl IntoView {
    let auth = use_auth_context();
    let navigate = use_navigate();
    let user = auth.user();
    let is_loading = auth.is_loading();

    // Effect to redirect authenticated users away from login
    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if !is_loading.get() {
                if let Some(user) = user.get() {
                    let redirect_path = match user.role {
                        UserRole::Admin => "/admin",
                        UserRole::Cashier => "/cashier",
                        UserRole::Staff => "/stations",
                    };
                    navigate(redirect_path, Default::default());
                }
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else if user.get().is_none() {
                view! { <LoginPage/> }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}

#[component]
fn ProtectedAdminPage() -> impl IntoView {
    let auth = use_auth_context();
    let navigate = use_navigate();
    let user = auth.user();
    let is_loading = auth.is_loading();

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if !is_loading.get() {
                if let Some(user) = user.get() {
                    if !matches!(user.role, UserRole::Admin) {
                        let redirect_path = match user.role {
                            UserRole::Admin => "/admin",
                            UserRole::Cashier => "/cashier",
                            UserRole::Staff => "/stations",
                        };
                        navigate(redirect_path, Default::default());
                    }
                } else {
                    navigate("/signin", Default::default());
                }
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else if let Some(user) = user.get() {
                if matches!(user.role, UserRole::Admin) {
                    view! { <AdminPage/> }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}

#[component]
fn ProtectedCashierPage() -> impl IntoView {
    let auth = use_auth_context();
    let navigate = use_navigate();
    let user = auth.user();
    let is_loading = auth.is_loading();

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if !is_loading.get() {
                if let Some(user) = user.get() {
                    if !matches!(user.role, UserRole::Admin | UserRole::Cashier) {
                        navigate("/stations", Default::default());
                    }
                } else {
                    navigate("/signin", Default::default());
                }
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else if let Some(user) = user.get() {
                if matches!(user.role, UserRole::Admin | UserRole::Cashier) {
                    view! { <CashierPage/> }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}

#[component]
fn ProtectedStationsPage() -> impl IntoView {
    let auth = use_auth_context();
    let navigate = use_navigate();
    let user = auth.user();
    let is_loading = auth.is_loading();

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if !is_loading.get() && user.get().is_none() {
                navigate("/signin", Default::default());
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else if user.get().is_some() {
                view! { <StationsOverviewPage/> }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}

#[component]
fn ProtectedDesignSystemPage() -> impl IntoView {
    let auth = use_auth_context();
    let navigate = use_navigate();
    let user = auth.user();
    let is_loading = auth.is_loading();

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if !is_loading.get() && user.get().is_none() {
                navigate("/signin", Default::default());
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else if user.get().is_some() {
                view! { <DesignSystemPage/> }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}
