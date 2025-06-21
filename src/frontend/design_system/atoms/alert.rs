use leptos::prelude::*;

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent,
    tokens::FromSize,
};

#[component]
pub fn Alert(
    #[prop(default = Intent::Info)]
    intent: Intent,
    
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(optional)]
    title: Option<&'static str>,
    
    #[prop(optional)]
    dismissible: Option<bool>,
    
    #[prop(optional)]
    on_dismiss: Option<Callback<()>>,
    
    #[prop(optional)]
    class: Option<&'static str>,
    
    children: Children,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    let is_dismissed = RwSignal::new(false);
    
    let final_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Size-based padding and text
        let (px, py) = match size {
            Size::Xs => (theme.spacing.px.from_size(Size::Sm), theme.spacing.py.from_size(Size::Xs)),
            Size::Sm => (theme.spacing.px.from_size(Size::Md), theme.spacing.py.from_size(Size::Sm)),
            Size::Md => (theme.spacing.px.from_size(Size::Lg), theme.spacing.py.from_size(Size::Md)),
            Size::Lg => (theme.spacing.px.from_size(Size::Xl), theme.spacing.py.from_size(Size::Lg)),
            Size::Xl => (theme.spacing.px.from_size(Size::Xl), theme.spacing.py.from_size(Size::Xl)),
        };
        
        // Intent-based colors using theme tokens
        let (bg_color, border_color, text_color) = match intent {
            Intent::Primary => (theme.colors.background.alert_info, theme.colors.border.info, theme.colors.text.alert_info),
            Intent::Secondary => (theme.colors.background.surface, theme.colors.border.muted, theme.colors.text.secondary),
            Intent::Success => (theme.colors.background.alert_success, theme.colors.border.success, theme.colors.text.alert_success),
            Intent::Danger => (theme.colors.background.alert_danger, theme.colors.border.danger, theme.colors.text.alert_danger),
            Intent::Warning => (theme.colors.background.alert_warning, theme.colors.border.warning, theme.colors.text.alert_warning),
            Intent::Info => (theme.colors.background.alert_info, theme.colors.border.info, theme.colors.text.alert_info),
        };
        
        let mut classes = vec![
            "flex items-start gap-3 transition-colors duration-200",
            px,
            py,
            theme.borders.radius.from_size(size),
            theme.borders.width.thin,
            bg_color,
            border_color,
            text_color,
        ];
        
        if let Some(additional_class) = class {
            classes.push(additional_class);
        }
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
    });
    
    let handle_dismiss = move |_| {
        is_dismissed.set(true);
        if let Some(callback) = on_dismiss {
            callback.run(());
        }
    };
    
    view! {
        <div 
            class=move || if is_dismissed.get() { "hidden".to_string() } else { final_classes.get() }
        >
            <div class="flex-1">
                {title.map(|t| view! {
                    <div class="font-semibold mb-1">{t}</div>
                })}
                <div class="text-sm">
                    {children()}
                </div>
            </div>
            
            {dismissible.unwrap_or(false).then(|| view! {
                <button
                    type="button"
                    class="flex-shrink-0 ml-2 inline-flex text-sm rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 opacity-70 hover:opacity-100 transition-opacity"
                    on:click=handle_dismiss
                >
                    <span class="sr-only">"Dismiss"</span>
                    <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
                    </svg>
                </button>
            })}
        </div>
    }
}