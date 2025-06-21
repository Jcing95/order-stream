use leptos::prelude::*;

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent,
    tokens::{FromSize, FromIntent},
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TextVariant {
    #[default]
    Body,
    Heading,
    Label,
    Caption,
    Code,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FontWeight {
    Normal,
    #[default]
    Medium,
    Semibold,
    Bold,
}

impl FontWeight {
    pub fn as_class(&self) -> &'static str {
        match self {
            FontWeight::Normal => "font-normal",
            FontWeight::Medium => "font-medium",
            FontWeight::Semibold => "font-semibold",
            FontWeight::Bold => "font-bold",
        }
    }
}

#[component]
pub fn Text(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    #[prop(default = TextVariant::Body)]
    variant: TextVariant,
    
    #[prop(default = FontWeight::Medium)]
    weight: FontWeight,
    
    #[prop(optional)]
    as_element: Option<&'static str>,
    
    #[prop(optional)]
    class: Option<&'static str>,
    
    children: Children,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    let final_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Size-based typography
        let text_size = match variant {
            TextVariant::Heading => match size {
                Size::Xs => theme.typography.lg,
                Size::Sm => theme.typography.xl,
                Size::Md => theme.typography.xl2,
                Size::Lg => theme.typography.xl3,
                Size::Xl => theme.typography.xl3,
            },
            TextVariant::Caption => match size {
                Size::Xs => theme.typography.xs,
                Size::Sm => theme.typography.xs,
                Size::Md => theme.typography.sm,
                Size::Lg => theme.typography.sm,
                Size::Xl => theme.typography.base,
            },
            TextVariant::Code => theme.typography.from_size(size),
            _ => theme.typography.from_size(size),
        };
        
        // Intent-based color
        let text_color = theme.colors.text.from_intent(intent);
        
        // Font weight
        let font_weight = weight.as_class();
        
        // Variant-specific styles
        let variant_classes = match variant {
            TextVariant::Body => "",
            TextVariant::Heading => "leading-tight tracking-tight",
            TextVariant::Label => "leading-none",
            TextVariant::Caption => "leading-relaxed",
            TextVariant::Code => "font-mono bg-gray-100 px-1 py-0.5 rounded text-gray-800",
        };
        
        let mut classes = vec![
            text_size,
            text_color,
            font_weight,
            variant_classes,
        ];
        
        if let Some(additional_class) = class {
            classes.push(additional_class);
        }
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
    });
    
    let element = as_element.unwrap_or(match variant {
        TextVariant::Heading => match size {
            Size::Xs => "h6",
            Size::Sm => "h5", 
            Size::Md => "h4",
            Size::Lg => "h3",
            Size::Xl => "h2",
        },
        TextVariant::Label => "label",
        TextVariant::Caption => "small",
        TextVariant::Code => "code",
        TextVariant::Body => "p",
    });
    
    match element {
        "h1" => view! { <h1 class=move || final_classes.get()>{children()}</h1> }.into_any(),
        "h2" => view! { <h2 class=move || final_classes.get()>{children()}</h2> }.into_any(),
        "h3" => view! { <h3 class=move || final_classes.get()>{children()}</h3> }.into_any(),
        "h4" => view! { <h4 class=move || final_classes.get()>{children()}</h4> }.into_any(),
        "h5" => view! { <h5 class=move || final_classes.get()>{children()}</h5> }.into_any(),
        "h6" => view! { <h6 class=move || final_classes.get()>{children()}</h6> }.into_any(),
        "label" => view! { <label class=move || final_classes.get()>{children()}</label> }.into_any(),
        "small" => view! { <small class=move || final_classes.get()>{children()}</small> }.into_any(),
        "code" => view! { <code class=move || final_classes.get()>{children()}</code> }.into_any(),
        "span" => view! { <span class=move || final_classes.get()>{children()}</span> }.into_any(),
        "div" => view! { <div class=move || final_classes.get()>{children()}</div> }.into_any(),
        _ => view! { <p class=move || final_classes.get()>{children()}</p> }.into_any(),
    }
}