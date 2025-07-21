use leptos::prelude::*;
use crate::frontend::design_system::ThemeSwitcher;
use crate::frontend::design_system::theme::Size;

/// Legacy theme toggle component - now uses the design system ThemeSwitcher
#[component]
pub fn ThemeToggle() -> impl IntoView {
    view! {
        <ThemeSwitcher size=Size::Md show_label=true />
    }
}