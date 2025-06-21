use leptos::prelude::*;
use leptos::ev::{Event, FocusEvent};

use crate::frontend::design_system::{
    Input, Select, Text, Alert,
    theme::{Size, Intent, ComponentState},
    atoms::{InputType, TextVariant, FontWeight, SelectOption},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldType {
    Input(InputType),
    Select,
    TextArea,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Input(InputType::Text)
    }
}

#[component]
pub fn FormField(
    /// Field label text
    label: &'static str,
    
    /// Type of field to render
    #[prop(default = FieldType::Input(InputType::Text))]
    field_type: FieldType,
    
    /// Size of the field
    #[prop(default = Size::Md)]
    size: Size,
    
    /// Intent/color variant
    #[prop(default = Intent::Primary)]
    intent: Intent,
    
    /// Field state
    #[prop(default = ComponentState::Enabled)]
    state: ComponentState,
    
    /// Field value signal
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    
    /// Options for select field
    #[prop(optional)]
    options: Option<Vec<SelectOption>>,
    
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<&'static str>,
    
    /// Error message to display
    #[prop(optional)]
    error: Option<Signal<Option<String>>>,
    
    /// Helper text to display below field
    #[prop(optional)]
    helper_text: Option<&'static str>,
    
    /// Whether field is required
    #[prop(optional)]
    required: Option<bool>,
    
    /// Field name attribute
    #[prop(optional)]
    name: Option<&'static str>,
    
    /// Field id attribute
    #[prop(optional)]
    id: Option<&'static str>,
    
    /// Input event handler
    #[prop(optional)]
    on_input: Option<Callback<Event>>,
    
    /// Change event handler (for selects)
    #[prop(optional)]
    on_change: Option<Callback<Event>>,
    
    /// Focus event handler
    #[prop(optional)]
    on_focus: Option<Callback<FocusEvent>>,
    
    /// Blur event handler
    #[prop(optional)]
    on_blur: Option<Callback<FocusEvent>>,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    // Determine field intent based on error state
    let field_intent = Signal::derive(move || {
        if let Some(error_signal) = error {
            if error_signal.get().is_some() {
                Intent::Danger
            } else {
                intent
            }
        } else {
            intent
        }
    });
    
    // Generate field id
    let field_id = id.unwrap_or_else(|| {
        // Simple id generation - in real app might want uuid
        Box::leak(format!("field-{}", label.replace(" ", "-").to_lowercase()).into_boxed_str())
    });
    
    view! {
        <div class=format!("space-y-2 {}", class.unwrap_or(""))>
            // Label
            <Text 
                variant=TextVariant::Label 
                size=Size::Sm 
                weight=FontWeight::Medium
                as_element="label"
                class="block"
            >
                {label}
                {required.unwrap_or(false).then(|| view! {
                    <span class="text-red-500 ml-1">"*"</span>
                })}
            </Text>
            
            // Field input
            {match field_type {
                FieldType::Input(input_type) => {
                    let current_intent = field_intent.get();
                    view! {
                        <Input
                            input_type=input_type
                            size=size
                            intent=current_intent
                            state=state
                            value=value
                            placeholder=placeholder.unwrap_or("")
                            required=required.unwrap_or(false)
                            name=name.unwrap_or("")
                            id=field_id
                            on_input=on_input
                            on_focus=on_focus
                            on_blur=on_blur
                        />
                    }.into_any()
                },
                FieldType::Select => {
                    let current_intent = field_intent.get();
                    view! {
                        <Select
                            size=size
                            intent=current_intent
                            state=state
                            options=options.unwrap_or_default()
                            placeholder=placeholder.unwrap_or("")
                            value=value
                            required=required.unwrap_or(false)
                            name=name.unwrap_or("")
                            id=field_id
                            on_change=on_change
                            on_focus=on_focus
                            on_blur=on_blur
                        />
                    }.into_any()
                },
                FieldType::TextArea => {
                    let current_intent = field_intent.get();
                    view! {
                        // For now, use Input - could create TextArea atom later
                        <Input
                            input_type=InputType::Text
                            size=size
                            intent=current_intent
                            state=state
                            value=value
                            placeholder=placeholder.unwrap_or("")
                            required=required.unwrap_or(false)
                            name=name.unwrap_or("")
                            id=field_id
                            on_input=on_input
                            on_focus=on_focus
                            on_blur=on_blur
                        />
                    }.into_any()
                },
            }}
            
            // Error message
            {move || error.and_then(|e| e.get()).map(|err| view! {
                <Alert intent=Intent::Danger size=Size::Sm>
                    {err}
                </Alert>
            })}
            
            // Helper text
            {helper_text.map(|text| view! {
                <Text 
                    variant=TextVariant::Caption 
                    intent=Intent::Secondary
                    size=Size::Sm
                >
                    {text}
                </Text>
            })}
        </div>
    }
}