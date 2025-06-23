use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::frontend::design_system::{
    Button, Text, ThemeSwitcher,
    theme::{Size, Intent, ThemeContext},
    atoms::{TextVariant, FontWeight},
};

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
                    // Left side - Main navigation
                    <div class="flex items-center space-x-1">
                        {nav_button("/admin", "Admin")}
                        {nav_button("/cashier", "Cashier")}
                        {nav_button("/stations", "Stations")}
                    </div>
                    
                    // Right side - Theme switcher
                    <div class="flex items-center">
                        <ThemeSwitcher size=Size::Sm />
                    </div>
                </div>
            </div>
        </nav>
    }
}