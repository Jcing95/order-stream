use leptos::prelude::*;
use leptos::ev::MouseEvent;

use crate::frontend::design_system::{
    Button, Icon, 
    theme::{Size, Intent, ThemeContext, ThemePreference},
    atoms::IconVariant,
};

#[component]
pub fn ThemeSwitcher(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = false)]
    show_label: bool,
    
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    // Get current theme preference and label
    let preference = ThemeContext::use_preference();
    let preference_label = ThemeContext::preference_label();
    
    // Handle theme cycling
    let cycle_theme = move |_: MouseEvent| {
        ThemeContext::cycle_preference();
    };

    // Dynamic icon based on current theme
    let icon_name = Signal::derive(move || {
        match preference.get() {
            ThemePreference::System => "monitor",
            ThemePreference::Light => "sun",
            ThemePreference::Dark => "moon",
        }
    });

    view! {
        <div class=format!("flex items-center gap-2 {}", class.unwrap_or(""))>
            <Button
                size=size
                intent=Intent::Secondary
                on_click=Callback::new(cycle_theme)
            >
                {move || {
                    let icon = icon_name.get();
                    view! { 
                        <Icon name=icon size=size variant=IconVariant::Outline /> 
                    }
                }}
                {show_label.then(|| view! {
                    <span class="ml-2">
                        {move || preference_label.get()}
                    </span>
                })}
            </Button>
        </div>
    }
}