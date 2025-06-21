use leptos::prelude::*;

use crate::frontend::design_system::theme::{
    ThemeContext, Size,
    tokens::FromSize,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Elevated,
    Outlined,
}

#[component]
pub fn Card(
    #[prop(default = CardVariant::Default)]
    variant: CardVariant,
    
    #[prop(default = Size::Md)]
    padding: Size,
    
    #[prop(optional)]
    class: Option<&'static str>,
    
    children: Children,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    let final_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Padding based on size
        let card_padding = match padding {
            Size::Xs => format!("{} {}", theme.spacing.px.from_size(Size::Sm), theme.spacing.py.from_size(Size::Xs)),
            Size::Sm => format!("{} {}", theme.spacing.px.from_size(Size::Md), theme.spacing.py.from_size(Size::Sm)),
            Size::Md => format!("{} {}", theme.spacing.px.from_size(Size::Lg), theme.spacing.py.from_size(Size::Md)),
            Size::Lg => format!("{} {}", theme.spacing.px.from_size(Size::Xl), theme.spacing.py.from_size(Size::Lg)),
            Size::Xl => format!("{} {}", theme.spacing.px.from_size(Size::Xl), theme.spacing.py.from_size(Size::Xl)),
        };
        
        // Variant-specific styling
        let variant_classes = match variant {
            CardVariant::Default => format!(
                "{} {} {} {}",
                theme.colors.background.surface,
                theme.borders.radius.from_size(Size::Lg),
                theme.borders.width.thin,
                theme.colors.border.muted
            ),
            CardVariant::Elevated => format!(
                "{} {} {} {} {}",
                theme.colors.background.elevated,
                theme.borders.radius.from_size(Size::Lg),
                theme.borders.width.thin,
                theme.colors.border.muted,
                theme.shadows.lg
            ),
            CardVariant::Outlined => format!(
                "{} {} {} {}",
                theme.colors.background.surface,
                theme.borders.radius.from_size(Size::Lg),
                theme.borders.width.thick,
                theme.colors.border.default
            ),
        };
        
        let mut classes = vec![
            "block transition-colors duration-200",
            &card_padding,
            &variant_classes,
        ];
        
        if let Some(additional_class) = class {
            classes.push(additional_class);
        }
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
    });
    
    view! {
        <div class=move || final_classes.get()>
            {children()}
        </div>
    }
}