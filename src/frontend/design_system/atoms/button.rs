use leptos::prelude::*;
use leptos::ev::MouseEvent;

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent, ComponentState,
    tokens::{FromSize, FromIntent},
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
        
        // Size-based tokens
        let (px, py) = match size {
            Size::Xs => (theme.spacing.from_size(Size::Xs), theme.spacing.from_size(Size::Xs)),
            Size::Sm => (theme.spacing.from_size(Size::Sm), theme.spacing.from_size(Size::Xs)),
            Size::Md => (theme.spacing.from_size(Size::Md), theme.spacing.from_size(Size::Sm)),
            Size::Lg => (theme.spacing.from_size(Size::Lg), theme.spacing.from_size(Size::Md)),
            Size::Xl => (theme.spacing.from_size(Size::Xl), theme.spacing.from_size(Size::Lg)),
        };
        let text_size = theme.typography.from_size(size);
        let radius = theme.borders.radius.from_size(size);
        
        // Intent-based tokens
        let bg = theme.colors.background.from_intent(intent);
        let bg_hover = match intent {
            Intent::Primary => theme.colors.background.primary_hover,
            Intent::Secondary => theme.colors.background.secondary_hover,
            Intent::Success => theme.colors.background.success_hover,
            Intent::Danger => theme.colors.background.danger_hover,
            Intent::Warning => theme.colors.background.warning_hover,
            Intent::Info => theme.colors.background.info_hover,
        };
        
        let text_color = match intent {
            Intent::Secondary => theme.colors.text.from_intent(intent),
            _ => "text-white",
        };
        
        let border = match intent {
            Intent::Secondary => format!("{} {}", theme.borders.width.thin, theme.colors.border.default),
            _ => String::new(),
        };
        
        let focus_ring = match intent {
            Intent::Primary => theme.colors.focus_ring.primary,
            Intent::Secondary => theme.colors.focus_ring.secondary, 
            Intent::Success => theme.colors.focus_ring.success,
            Intent::Danger => theme.colors.focus_ring.danger,
            Intent::Warning => theme.colors.focus_ring.warning,
            Intent::Info => theme.colors.focus_ring.info,
        };
        
        let state_classes = state.get_modifier_classes();
        
        // Create owned strings for dynamic values
        let px_class = format!("px-{}", px);
        let py_class = format!("py-{}", py);
        
        let mut classes = vec![
            "inline-flex items-center justify-center font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2",
            &px_class,
            &py_class,
            text_size,
            radius,
            bg,
            bg_hover,
            text_color,
            focus_ring,
            state_classes,
        ];
        
        if !border.is_empty() {
            classes.push(&border);
        }
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
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

