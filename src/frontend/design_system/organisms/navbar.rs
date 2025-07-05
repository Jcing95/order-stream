use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::frontend::design_system::{
    Button, Text, ThemeSwitcher,
    theme::{Size, Intent, ThemeContext},
    atoms::{TextVariant, FontWeight},
};
use crate::frontend::state::auth::use_auth_context;
use crate::common::types::UserRole;

/// Navigation bar organism component
/// 
/// Provides primary navigation across the application with integrated theme switching.
/// Designed for tablet/station interfaces with clear, accessible navigation.
/// 
/// # Example
/// ```rust
/// use crate::frontend::design_system::organisms::Navbar;
/// 
/// view! {
///     <Navbar />
/// }
/// ```
#[component]
pub fn Navbar() -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    let location = use_location();
    let auth = use_auth_context();
    
    let user = auth.user();
    let is_authenticated = auth.is_authenticated();
    
    // Navigation button component
    let nav_button = move |href: &'static str, label: &'static str| {
        let href_str = href.to_string();
        view! {
            <A href=href>
                {move || {
                    let current_path = location.pathname.get();
                    let is_active = if href_str == "/" {
                        current_path == "/"
                    } else {
                        current_path.starts_with(&href_str)
                    };
                    let intent = if is_active { Intent::Primary } else { Intent::Secondary };
                    
                    view! {
                        <Button
                            size=Size::Md
                            intent=intent
                        >
                            <Text 
                                variant=TextVariant::Body 
                                size=Size::Sm
                                weight=FontWeight::Medium
                            >
                                {label}
                            </Text>
                        </Button>
                    }
                }}
            </A>
        }
    };

    // Logout handler - create a new one each time to avoid FnOnce issues
    let create_logout_handler = {
        let auth = auth.clone();
        move || {
            let auth = auth.clone();
            move |_| {
                let auth = auth.clone();
                leptos::task::spawn_local(async move {
                    auth.logout().await;
                });
            }
        }
    };

    view! {
        <nav class=move || {
            let theme = theme_signal.get();
            format!(
                "w-full border-b transition-colors duration-200 {} {}",
                theme.colors.background.elevated,
                theme.colors.border.default
            )
        }>
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between h-16">
                    // Left side - App brand and main navigation
                    <div class="flex items-center space-x-4">
                        // App brand
                        <A href="/">
                            <Text 
                                variant=TextVariant::Heading 
                                size=Size::Lg
                                weight=FontWeight::Bold
                            >
                                "Order Stream"
                            </Text>
                        </A>
                        
                        // Role-based navigation
                        {move || {
                            if is_authenticated.get() {
                                if let Some(user) = user.get() {
                                    view! {
                                        <div class="flex items-center space-x-1">
                                            // Admin can see everything
                                            {move || {
                                                if matches!(user.role, UserRole::Admin) {
                                                    view! {
                                                        <div class="flex items-center space-x-1">
                                                            {nav_button("/admin", "Admin")}
                                                            {nav_button("/cashier", "Cashier")}
                                                            {nav_button("/stations", "Stations")}
                                                        </div>
                                                    }.into_any()
                                                } else if matches!(user.role, UserRole::Cashier) {
                                                    // Cashier can see cashier and stations
                                                    view! {
                                                        <div class="flex items-center space-x-1">
                                                            {nav_button("/cashier", "Cashier")}
                                                            {nav_button("/stations", "Stations")}
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    // Staff can only see stations
                                                    view! {
                                                        <div class="flex items-center space-x-1">
                                                            {nav_button("/stations", "Stations")}
                                                        </div>
                                                    }.into_any()
                                                }
                                            }}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                    
                    // Right side - User info, logout, and theme switcher
                    <div class="flex items-center space-x-2">
                        {move || {
                            if is_authenticated.get() {
                                if let Some(user) = user.get() {
                                    view! {
                                        <div class="flex items-center space-x-2">
                                            // User info
                                            <div class="text-right">
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Sm
                                                    weight=FontWeight::Medium
                                                >
                                                    {user.email.clone()}
                                                </Text>
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Xs
                                                    intent=Intent::Secondary
                                                >
                                                    {format!("{:?}", user.role)}
                                                </Text>
                                            </div>
                                            
                                            // Logout button
                                            <Button
                                                size=Size::Sm
                                                intent=Intent::Secondary
                                                on:click=create_logout_handler()
                                            >
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Sm
                                                    weight=FontWeight::Medium
                                                >
                                                    "Logout"
                                                </Text>
                                            </Button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            } else {
                                // Show login button for unauthenticated users
                                view! {
                                    <A href="/signin">
                                        <Button
                                            size=Size::Sm
                                            intent=Intent::Primary
                                        >
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm
                                                weight=FontWeight::Medium
                                            >
                                                "Sign In"
                                            </Text>
                                        </Button>
                                    </A>
                                }.into_any()
                            }
                        }}
                        
                        // Theme switcher
                        <ThemeSwitcher size=Size::Sm />
                    </div>
                </div>
            </div>
        </nav>
    }
}