use leptos::prelude::*;
use crate::common::types::CreateCategoryRequest;
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert,
    theme::{Size, Intent, ComponentState},
    atoms::{InputType, TextVariant, FontWeight},
};

#[component]
pub fn CategoryForm<F>(on_submit: F) -> impl IntoView 
where
    F: Fn(CreateCategoryRequest) + 'static + Clone + Send + Sync,
{
    let name = RwSignal::new(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    // Create Action for form submission - proper Leptos pattern
    let submit_action = Action::new({
        let on_submit = on_submit.clone();
        move |request: &CreateCategoryRequest| {
            let on_submit = on_submit.clone();
            let request = request.clone();
            async move {
                // Validate
                if let Err(err) = request.validate() {
                    return Err(err);
                }
                
                // Submit to parent callback
                on_submit(request);
                Ok(())
            }
        }
    });
    
    // Derived signals for form validation state
    let is_form_valid = Signal::derive(move || {
        !name.get().trim().is_empty()
    });
    
    let is_submitting = Signal::derive(move || {
        submit_action.pending().get()
    });

    // Handle form submission with proper Leptos event type
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        set_error.set(None);
        
        let request = CreateCategoryRequest {
            name: name.get().trim().to_string(),
        };

        submit_action.dispatch(request);
    };

    // Handle action results
    Effect::new_isomorphic({
        let name = name;
        let set_error = set_error;
        move |_| {
            if let Some(result) = submit_action.value().get() {
                match result {
                    Ok(_) => {
                        // Success - clear form
                        name.set(String::new());
                    },
                    Err(err) => {
                        // Show validation error
                        set_error.set(Some(err));
                    }
                }
            }
        }
    });

    view! {
        <Card variant=CardVariant::Default>
            <form on:submit=handle_submit class="space-y-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Lg 
                    weight=FontWeight::Semibold
                >
                    "Add New Category"
                </Text>
                
                {move || error.get().map(|err| view! {
                    <Alert intent=Intent::Danger size=Size::Sm>
                        {err}
                    </Alert>
                })}
                
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Category Name"
                    </Text>
                    <Input
                        input_type=InputType::Text
                        size=Size::Md
                        intent=Intent::Primary
                        value=name
                        placeholder="e.g., Drinks, Food, Snacks"
                        required=true
                        state=if is_submitting.get() { ComponentState::Disabled } else { ComponentState::Enabled }
                        on_input=Callback::new(move |ev| name.set(event_target_value(&ev)))
                    />
                </div>
                
                <Button
                    size=Size::Md
                    intent=Intent::Primary
                    state=if is_submitting.get() { ComponentState::Loading } else if is_form_valid.get() { ComponentState::Enabled } else { ComponentState::Disabled }
                >
                    {move || if is_submitting.get() { "Adding..." } else { "Add Category" }}
                </Button>
            </form>
        </Card>
    }
}