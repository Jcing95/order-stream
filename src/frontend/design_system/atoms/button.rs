use leptos::prelude::*;
use leptos::ev::MouseEvent;

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent, ComponentState,
    variants::utils::*,
};

/// Button component with variant support
/// 
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use crate::frontend::design_system::{Button, Size, Intent};
/// 
/// view! {
///     <Button 
///         size=Size::Lg
///         intent=Intent::Primary
///         on_click=move |_| {
///             log!("Button clicked!");
///         }
///     >
///         "Save Changes"
///     </Button>
/// }
/// ```
#[component]
pub fn Button(
    /// Button size variant
    #[prop(default = Size::Md)]
    size: Size,
    
    /// Button intent/color variant
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    /// Button state
    #[prop(default = ComponentState::Enabled)]
    state: ComponentState,
    
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    
    /// Button content
    children: Children,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    // Make class computation reactive to theme changes
    let final_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Build component classes based on variants
        let base_classes = "inline-flex items-center justify-center font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2";
        
        let size_classes = combine_classes(&[
            &get_padding_classes(size, &theme.spacing),
            get_typography_classes(size, &theme.typography),
            get_border_radius_classes(size, &theme.borders),
        ]);
        
        let (bg_class, bg_hover_class) = get_background_classes(intent, &theme.colors.background);
        let text_class = match intent {
            Intent::Secondary => get_text_classes(intent, &theme.colors.text),
            _ => "text-white", // Primary, Success, Danger, Warning, Info use white text
        };
        
        let border_class = match intent {
            Intent::Secondary => combine_classes(&[
                theme.borders.width.thin,
                get_border_classes(intent, &theme.colors.border),
            ]),
            _ => String::new(),
        };
        
        let focus_ring_class = match intent {
            Intent::Primary | Intent::Info => "focus:ring-blue-500",
            Intent::Secondary => "focus:ring-gray-500",
            Intent::Success => "focus:ring-green-500",
            Intent::Danger => "focus:ring-red-500",
            Intent::Warning => "focus:ring-yellow-500",
        };
        
        let state_classes = state.get_modifier_classes();
        
        combine_classes(&[
            base_classes,
            &size_classes,
            bg_class,
            bg_hover_class,
            text_class,
            &border_class,
            focus_ring_class,
            state_classes,
        ])
    });
    
    let is_disabled = state == ComponentState::Disabled;
    
    view! {
        <button
            class=move || final_classes.get()
            disabled=is_disabled
            on:click=move |ev| {
                if let Some(handler) = on_click {
                    if state == ComponentState::Enabled {
                        handler.run(ev);
                    }
                }
            }
        >
            {children()}
        </button>
    }
}

/// Icon button variant for buttons with just icons
#[component]
pub fn IconButton(
    /// Button size variant
    #[prop(default = Size::Md)]
    size: Size,
    
    /// Button intent/color variant
    #[prop(default = Intent::Secondary)]
    intent: Intent,
    
    /// Button state
    #[prop(default = ComponentState::Enabled)]
    state: ComponentState,
    
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    
    /// Accessible label for screen readers
    #[prop(optional)]
    aria_label: Option<&'static str>,
    
    /// Button content (usually an icon)
    children: Children,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    // Make class computation reactive
    let icon_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Icon buttons are square with equal padding
        let padding = match size {
            Size::Xs => theme.spacing.xs,
            Size::Sm => theme.spacing.sm,
            Size::Md => theme.spacing.md,
            Size::Lg => theme.spacing.lg,
            Size::Xl => theme.spacing.xl,
        };
        
        format!(
            "inline-flex items-center justify-center p-{} {} {} transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 {}",
            padding,
            get_typography_classes(size, &theme.typography),
            get_border_radius_classes(size, &theme.borders),
            state.get_modifier_classes()
        )
    });
    
    view! {
        <button
            class=move || icon_classes.get()
            disabled=state == ComponentState::Disabled
            aria-label=aria_label
            on:click=move |ev| {
                if let Some(handler) = on_click {
                    if state == ComponentState::Enabled {
                        handler.run(ev);
                    }
                }
            }
        >
            {children()}
        </button>
    }
}