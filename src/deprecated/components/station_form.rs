use leptos::prelude::*;
use crate::common::types::{CreateStationRequest, Category, OrderStatus};
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert, Select, SelectOption,
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
    let selected_categories = RwSignal::new(Vec::<String>::new());
    let input_statuses = RwSignal::new(vec![OrderStatus::Ordered]);
    let output_status = RwSignal::new(OrderStatus::Ready);
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
        
        let request = CreateStationRequest {
            name: name.get().trim().to_string(),
            category_ids: selected_categories.get(),
            input_statuses: input_statuses.get(),
            output_status: output_status.get(),
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
                        selected_categories.set(Vec::new());
                        input_statuses.set(vec![OrderStatus::Ordered]);
                        output_status.set(OrderStatus::Ready);
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
                    {move || {
                        view! {
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
                        }
                    }}
                </div>
                
                // Category Selection
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Categories"
                    </Text>
                    {move || {
                        let category_options = categories.get().into_iter().map(|cat| {
                            SelectOption::new(cat.id, cat.name)
                        }).collect::<Vec<_>>();
                        
                        view! {
                            <Select
                                size=Size::Md
                                intent=Intent::Primary
                                value=RwSignal::new(String::new()) // Placeholder for multi-select
                                state=if submit_action.pending().get() { ComponentState::Disabled } else { ComponentState::Enabled }
                                placeholder="Select categories for this station"
                                required=false
                                options=category_options
                            />
                        }
                    }}
                    <Text 
                        variant=TextVariant::Caption 
                        size=Size::Xs 
                        intent=Intent::Secondary
                    >
                        "This station will only show orders containing items from these categories"
                    </Text>
                </div>
                
                // Input/Output Status Info
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                    >
                        "Workflow Configuration"
                    </Text>
                    <div class="p-3 bg-gray-50 dark:bg-gray-800 rounded-md space-y-2">
                        <Text variant=TextVariant::Caption size=Size::Xs>
                            "Shows orders with status: Ordered"
                        </Text>
                        <Text variant=TextVariant::Caption size=Size::Xs>
                            "Updates items to status: Ready when processed"
                        </Text>
                    </div>
                    <Text 
                        variant=TextVariant::Caption 
                        size=Size::Xs 
                        intent=Intent::Secondary
                    >
                        "Advanced status configuration will be available in future updates"
                    </Text>
                </div>
                
                {move || {
                    view! {
                        <Button
                            size=Size::Md
                            intent=Intent::Primary
                            state=if submit_action.pending().get() { ComponentState::Loading } else { ComponentState::Enabled }
                        >
                            {move || if submit_action.pending().get() { "Adding..." } else { "Add Station" }}
                        </Button>
                    }
                }}
            </form>
        </Card>
    }
}