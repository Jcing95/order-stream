use leptos::prelude::*;
use crate::common::types::{CreateStationRequest, Category, OrderStatus};
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert,
    theme::{Size, Intent, ComponentState},
    atoms::{InputType, TextVariant, FontWeight},
};

#[component]
pub fn StationForm<F>(
    categories: Signal<Vec<Category>>,
    on_submit: F,
) -> impl IntoView 
where
    F: Fn(CreateStationRequest) + 'static + Clone + Send + Sync,
{
    let name = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);

    // Create Action for form submission - proper Leptos pattern
    let submit_action = Action::new({
        let on_submit = on_submit.clone();
        move |request: &CreateStationRequest| {
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

    // Handle form submission with proper Leptos event type
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        error.set(None);
        
        // For now, create a minimal station with default values
        let request = CreateStationRequest {
            name: name.get().trim().to_string(),
            category_ids: Vec::new(), // TODO: Add proper category selection
            input_statuses: vec![OrderStatus::Ordered], // Default input status
            output_status: OrderStatus::Ready, // Default output status
        };

        submit_action.dispatch(request);
    };

    // Handle action results
    Effect::new_isomorphic({
        let name = name;
        let error = error;
        move |_| {
            if let Some(result) = submit_action.value().get() {
                match result {
                    Ok(_) => {
                        // Success - clear form
                        name.set(String::new());
                    },
                    Err(err) => {
                        // Show validation error
                        error.set(Some(err));
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
                    "Add New Station"
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
                        "Station Name"
                    </Text>
                    <Input
                        input_type=InputType::Text
                        size=Size::Md
                        intent=Intent::Primary
                        value=name
                        placeholder="e.g., Kitchen, Bar, Drinks"
                        required=true
                        state=if submit_action.pending().get() { ComponentState::Disabled } else { ComponentState::Enabled }
                        on_input=Callback::new(move |ev| name.set(event_target_value(&ev)))
                    />
                </div>
                
                <Text 
                    variant=TextVariant::Caption 
                    size=Size::Xs 
                    intent=Intent::Secondary
                >
                    "Station configuration will be expanded in future updates"
                </Text>
                
                <Button
                    size=Size::Md
                    intent=Intent::Primary
                    state=if submit_action.pending().get() { ComponentState::Loading } else { ComponentState::Enabled }
                >
                    {move || if submit_action.pending().get() { "Adding..." } else { "Add Station" }}
                </Button>
            </form>
        </Card>
    }
}