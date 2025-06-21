use leptos::prelude::*;
use leptos::ev::{Event, FocusEvent};

use crate::frontend::design_system::theme::{
    ThemeContext, Size, Intent, ComponentState,
    tokens::FromSize,
};

#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[component]
pub fn Select(
    #[prop(default = Size::Md)]
    size: Size,
    
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    #[prop(default = ComponentState::Enabled)]
    state: ComponentState,
    
    options: Vec<SelectOption>,
    
    #[prop(optional)]
    placeholder: Option<&'static str>,
    
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    
    #[prop(optional)]
    on_change: Option<Callback<Event>>,
    
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
            "w-full appearance-none transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1 cursor-pointer",
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
            // Add dropdown arrow padding
            "pr-10",
        ];
        
        classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
    });
    
    let is_disabled = state == ComponentState::Disabled;
    let is_required = required.unwrap_or(false);
    
    view! {
        <div class="relative">
            <select
                class=move || final_classes.get()
                disabled=is_disabled
                required=is_required
                id=id.unwrap_or("")
                name=name.unwrap_or("")
                prop:value=move || value.map(|v| v.get()).unwrap_or_default()
                on:change=move |ev| {
                    if let Some(v) = value {
                        v.set(event_target_value(&ev));
                    }
                    if let Some(handler) = on_change {
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
            >
                {placeholder.map(|p| view! {
                    <option value="">{p}</option>
                })}
                
                {options.into_iter().map(|option| view! {
                    <option 
                        value=option.value 
                        disabled=option.disabled
                    >
                        {option.label}
                    </option>
                }).collect_view()}
            </select>
            
            // Dropdown arrow
            <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                <svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                </svg>
            </div>
        </div>
    }
}