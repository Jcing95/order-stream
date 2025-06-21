use leptos::prelude::*;
use leptos::ev::{Event, FocusEvent};

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent, ComponentState,
    tokens::FromSize,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
        }
    }
}

#[component]
pub fn Input(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    #[prop(default = ComponentState::Enabled)]
    state: ComponentState,
    
    #[prop(default = InputType::Text)]
    input_type: InputType,
    
    #[prop(optional)]
    placeholder: Option<&'static str>,
    
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    
    #[prop(optional)]
    on_input: Option<Callback<Event>>,
    
    #[prop(optional)]
    on_focus: Option<Callback<FocusEvent>>,
    
    #[prop(optional)]
    on_blur: Option<Callback<FocusEvent>>,
    
    #[prop(optional)]
    id: Option<&'static str>,
    
    #[prop(optional)]
    name: Option<&'static str>,
    
    #[prop(optional)]
    required: Option<bool>,
) -> impl IntoView {
    let theme_signal = ThemeContext::use_theme();
    
    let final_classes = Signal::derive(move || {
        let theme = theme_signal.get();
        
        // Size-based tokens
        let (px, py) = match size {
            Size::Xs => (theme.spacing.px.from_size(Size::Xs), theme.spacing.py.from_size(Size::Xs)),
            Size::Sm => (theme.spacing.px.from_size(Size::Sm), theme.spacing.py.from_size(Size::Xs)),
            Size::Md => (theme.spacing.px.from_size(Size::Md), theme.spacing.py.from_size(Size::Sm)),
            Size::Lg => (theme.spacing.px.from_size(Size::Lg), theme.spacing.py.from_size(Size::Md)),
            Size::Xl => (theme.spacing.px.from_size(Size::Xl), theme.spacing.py.from_size(Size::Lg)),
        };
        let text_size = theme.typography.from_size(size);
        let radius = theme.borders.radius.from_size(size);
        
        // Intent-based tokens
        let border_color = match intent {
            Intent::Primary => theme.colors.border.default,
            Intent::Secondary => theme.colors.border.muted,
            Intent::Success => theme.colors.border.success,
            Intent::Danger => theme.colors.border.danger,
            Intent::Warning => theme.colors.border.warning,
            Intent::Info => theme.colors.border.info,
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
        
        let classes = vec![
            "w-full appearance-none transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1",
            px,
            py,
            text_size,
            radius,
            theme.borders.width.thin,
            border_color,
            theme.colors.background.surface,
            theme.colors.text.primary,
            focus_ring,
            state_classes,
        ];
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
    });
    
    let is_disabled = state == ComponentState::Disabled;
    let is_required = required.unwrap_or(false);
    
    view! {
        <input
            type=input_type.as_str()
            class=move || final_classes.get()
            placeholder=placeholder.unwrap_or("")
            disabled=is_disabled
            required=is_required
            id=id.unwrap_or("")
            name=name.unwrap_or("")
            prop:value=move || value.map(|v| v.get()).unwrap_or_default()
            on:input=move |ev| {
                if let Some(v) = value {
                    v.set(event_target_value(&ev));
                }
                if let Some(handler) = on_input {
                    if state == ComponentState::Enabled {
                        handler.run(ev);
                    }
                }
            }
            on:focus=move |ev| {
                if let Some(handler) = on_focus {
                    if state == ComponentState::Enabled {
                        handler.run(ev);
                    }
                }
            }
            on:blur=move |ev| {
                if let Some(handler) = on_blur {
                    if state == ComponentState::Enabled {
                        handler.run(ev);
                    }
                }
            }
        />
    }
}