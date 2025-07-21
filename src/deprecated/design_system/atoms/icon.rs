use leptos::prelude::*;

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent,
    tokens::FromIntent,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum IconVariant {
    #[default]
    Solid,
    Outline,
}

#[component]
pub fn Icon(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    #[prop(default = IconVariant::Outline)]
    variant: IconVariant,
    
    name: &'static str,
    
    #[prop(optional)]
    class: Option<&'static str>,
    
    #[prop(optional)]
    aria_label: Option<&'static str>,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    let final_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Size-based dimensions
        let icon_size = match size {
            Size::Xs => "w-3 h-3",
            Size::Sm => "w-4 h-4",
            Size::Md => "w-5 h-5",  
            Size::Lg => "w-6 h-6",
            Size::Xl => "w-8 h-8",
        };
        
        // Intent-based color
        let text_color = theme.colors.text.from_intent(intent);
        
        let mut classes = vec![
            icon_size,
            text_color,
            "inline-block flex-shrink-0",
        ];
        
        if let Some(additional_class) = class {
            classes.push(additional_class);
        }
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
    });
    
    // Basic SVG icons - you can expand this with your preferred icon library
    let svg_content = match name {
        "check" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m1 9 4 4L5 13"/>"#,
        },
        "x" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M6 6l12 12M6 18L18 6"/>"#,
        },
        "plus" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>"#,
        },
        "minus" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M5 10a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M6 12h12"/>"#,
        },
        "chevron-down" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m6 9 6 6 6-6"/>"#,
        },
        "chevron-up" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m18 15-6-6-6 6"/>"#,
        },
        "chevron-left" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m15 18-6-6 6-6"/>"#,
        },
        "chevron-right" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m9 18 6-6-6-6"/>"#,
        },
        "search" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m21 21-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>"#,
        },
        "menu" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>"#,
        },
        "home" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" points="9,22 9,12 15,12 15,22"/>"#,
        },
        "sun" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M12 2.25a.75.75 0 01.75.75v2.25a.75.75 0 01-1.5 0V3a.75.75 0 01.75-.75zM7.5 12a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM18.894 6.166a.75.75 0 00-1.06-1.06l-1.591 1.59a.75.75 0 101.06 1.061l1.591-1.59zM21.75 12a.75.75 0 01-.75.75h-2.25a.75.75 0 010-1.5H21a.75.75 0 01.75.75zM17.834 18.894a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 10-1.061 1.06l1.59 1.591zM12 18a.75.75 0 01.75.75V21a.75.75 0 01-1.5 0v-2.25A.75.75 0 0112 18zM7.758 17.303a.75.75 0 00-1.061-1.06l-1.591 1.59a.75.75 0 001.06 1.061l1.591-1.59zM6 12a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h2.25A.75.75 0 016 12zM6.697 7.757a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 00-1.061 1.06l1.59 1.591z"/>"#,
            IconVariant::Outline => r#"<circle cx="12" cy="12" r="5" stroke="currentColor" stroke-width="2" fill="none"/><line x1="12" y1="1" x2="12" y2="3" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="12" y1="21" x2="12" y2="23" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="1" y1="12" x2="3" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="21" y1="12" x2="23" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>"#,
        },
        "moon" => match variant {
            IconVariant::Solid => r#"<path fill="currentColor" d="M9.528 1.718a.75.75 0 01.162.819A8.97 8.97 0 009 6a9 9 0 009 9 8.97 8.97 0 003.463-.69.75.75 0 01.981.98 10.503 10.503 0 01-9.694 6.46c-5.799 0-10.5-4.701-10.5-10.5 0-4.368 2.667-8.112 6.46-9.694a.75.75 0 01.818.162z"/>"#,
            IconVariant::Outline => r#"<path stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>"#,
        },
        _ => r#"<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none"/>"#, // fallback circle
    };
    
    view! {
        <svg
            class=move || final_classes.get()
            fill="none"
            viewBox="0 0 24 24"
            aria-label=aria_label.unwrap_or("")
            role=if aria_label.is_some() { "img" } else { "presentation" }
            inner_html=svg_content
        />
    }
}