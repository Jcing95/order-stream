use leptos::prelude::*;

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SpinnerVariant {
    #[default]
    Circle,
    Pulse,
    Dots,
}

#[component]
pub fn Spinner(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    #[prop(default = SpinnerVariant::Circle)]
    variant: SpinnerVariant,
    
    #[prop(optional)]
    label: Option<&'static str>,
    
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    let final_classes = Signal::derive(move || {
        let _theme = theme_signal.get();
        
        // Size-based dimensions
        let spinner_size = match size {
            Size::Xs => "w-3 h-3",
            Size::Sm => "w-4 h-4",
            Size::Md => "w-5 h-5",
            Size::Lg => "w-6 h-6",
            Size::Xl => "w-8 h-8",
        };
        
        // Intent-based colors for the spinner
        let color_class = match intent {
            Intent::Primary => "border-blue-600 dark:border-blue-400",
            Intent::Secondary => "border-gray-600 dark:border-gray-400",
            Intent::Success => "border-green-600 dark:border-green-400",
            Intent::Danger => "border-red-600 dark:border-red-400",
            Intent::Warning => "border-yellow-600 dark:border-yellow-400",
            Intent::Info => "border-blue-600 dark:border-blue-400",
        };
        
        let mut classes = vec![
            spinner_size,
            "inline-block",
        ];
        
        if let Some(additional_class) = class {
            classes.push(additional_class);
        }
        
        (classes.join(" "), color_class)
    });
    
    let spinner_content = move || match variant {
        SpinnerVariant::Circle => {
            let (base_classes, color_class) = final_classes.get();
            view! {
                <div class=format!("{} animate-spin rounded-full border-2 border-gray-200 dark:border-gray-700 border-t-transparent {}", base_classes, color_class)
                     role="status"
                     aria-label=label.unwrap_or("Loading")
                >
                    <span class="sr-only">{label.unwrap_or("Loading...")}</span>
                </div>
            }
        },
        SpinnerVariant::Pulse => {
            let (base_classes, color_class) = final_classes.get();
            view! {
                <div class=format!("{} animate-pulse rounded-full {}", base_classes, color_class.replace("border", "bg"))
                     role="status"
                     aria-label=label.unwrap_or("Loading")
                >
                    <span class="sr-only">{label.unwrap_or("Loading...")}</span>
                </div>
            }
        },
        SpinnerVariant::Dots => {
            let (base_classes, color_class) = final_classes.get();
            let dot_size = match size {
                Size::Xs => "w-1 h-1",
                Size::Sm => "w-1.5 h-1.5", 
                Size::Md => "w-2 h-2",
                Size::Lg => "w-2.5 h-2.5",
                Size::Xl => "w-3 h-3",
            };
            view! {
                <div class=format!("{} flex space-x-1", base_classes)
                     role="status"
                     aria-label=label.unwrap_or("Loading")
                >
                    <div class=format!("{} rounded-full animate-bounce [animation-delay:-0.3s] {}", dot_size, color_class.replace("border", "bg"))></div>
                    <div class=format!("{} rounded-full animate-bounce [animation-delay:-0.15s] {}", dot_size, color_class.replace("border", "bg"))></div>
                    <div class=format!("{} rounded-full animate-bounce {}", dot_size, color_class.replace("border", "bg"))></div>
                    <span class="sr-only">{label.unwrap_or("Loading...")}</span>
                </div>
            }
        },
    };
    
    view! {
        {spinner_content}
    }
}