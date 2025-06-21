use leptos::prelude::*;
use leptos::ev::MouseEvent;

use crate::frontend::design_system::{
    Button, Icon, 
    theme::{Size, Intent, ThemeContext, Theme},
    atoms::IconVariant,
};
use crate::frontend::state::theme::{ThemeState, ThemePreference};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }

    pub fn to_theme(&self) -> Theme {
        match self {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            ThemeMode::Light => "sun",
            ThemeMode::Dark => "moon",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ThemeMode::Light => "Switch to Dark Mode",
            ThemeMode::Dark => "Switch to Light Mode",
        }
    }
}

#[component]
pub fn ThemeSwitcher(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = false)]
    show_label: bool,
    
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    // Get the current theme from both systems
    let theme_state = use_context::<ThemeState>().expect("ThemeState context not found");
    let design_system_theme = ThemeContext::use_theme();
    
    // Track current theme mode based on design system theme
    let current_mode = Signal::derive(move || {
        let theme = design_system_theme.get();
        if theme.name == "dark" {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        }
    });
    
    // Handle theme switching - update both systems
    let toggle_theme = move |_: MouseEvent| {
        let new_mode = current_mode.get().toggle();
        
        // Update design system theme
        ThemeContext::set_theme(new_mode.to_theme());
        
        // Update old theme state to stay in sync
        let new_preference = match new_mode {
            ThemeMode::Light => ThemePreference::Light,
            ThemeMode::Dark => ThemePreference::Dark,
        };
        theme_state.preference.set(new_preference);
        
        // Save to localStorage
        #[cfg(feature = "hydrate")]
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme-preference", new_preference.to_string());
            }
        }
    };

    view! {
        <div class=format!("flex items-center gap-2 {}", class.unwrap_or(""))>
            <Button
                size=size
                intent=Intent::Secondary
                on_click=Callback::new(toggle_theme)
            >
                {move || match current_mode.get() {
                    ThemeMode::Light => view! { 
                        <Icon name="sun" size=size variant=IconVariant::Outline /> 
                    },
                    ThemeMode::Dark => view! { 
                        <Icon name="moon" size=size variant=IconVariant::Outline /> 
                    },
                }}
                {show_label.then(|| view! {
                    <span class="ml-2">
                        {move || match current_mode.get() {
                            ThemeMode::Light => "Light",
                            ThemeMode::Dark => "Dark",
                        }}
                    </span>
                })}
            </Button>
        </div>
    }
}