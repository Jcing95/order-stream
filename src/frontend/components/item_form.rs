use leptos::prelude::*;
use crate::common::types::{CreateItemRequest, Category};
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert,
    theme::{Size, Intent, ComponentState},
    atoms::{InputType, TextVariant, FontWeight},
};

#[component]
pub fn ItemForm<F>(
    categories: Signal<Vec<Category>>,
    on_submit: F,
) -> impl IntoView 
where
    F: Fn(CreateItemRequest) + 'static + Clone + Send + Sync,
{
    let name = RwSignal::new(String::new());
    let category = RwSignal::new(String::new());
    let price = RwSignal::new(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    // Create Action for form submission - proper Leptos pattern
    let submit_action = Action::new({
        let on_submit = on_submit.clone();
        move |request: &CreateItemRequest| {
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
        !name.get().trim().is_empty() && 
        !category.get().trim().is_empty() && 
        !price.get().trim().is_empty() &&
        price.get().parse::<f64>().is_ok()
    });
    
    let is_submitting = Signal::derive(move || {
        submit_action.pending().get()
    });

    // Handle form submission with proper Leptos event type
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        set_error.set(None);
        
        // Parse price
        let price_value = match price.get().parse::<f64>() {
            Ok(p) if p >= 0.0 => p,
            Ok(_) => {
                set_error.set(Some("Price cannot be negative".to_string()));
                return;
            }
            Err(_) => {
                set_error.set(Some("Please enter a valid price".to_string()));
                return;
            }
        };

        let request = CreateItemRequest {
            name: name.get().trim().to_string(),
            category_id: category.get().trim().to_string(),
            price: price_value,
        };

        submit_action.dispatch(request);
    };

    // Handle action results
    Effect::new_isomorphic({
        let name = name;
        let category = category;
        let price = price;
        let set_error = set_error;
        move |_| {
            if let Some(result) = submit_action.value().get() {
                match result {
                    Ok(_) => {
                        // Success - clear form
                        name.set(String::new());
                        category.set(String::new());
                        price.set(String::new());
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
                    "Add New Item"
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
                        "Item Name"
                    </Text>
                    <Input
                        input_type=InputType::Text
                        size=Size::Md
                        intent=Intent::Primary
                        value=name
                        placeholder="e.g., Coffee, Burger, Soda"
                        required=true
                        state=if is_submitting.get() { ComponentState::Disabled } else { ComponentState::Enabled }
                        on_input=Callback::new(move |ev| name.set(event_target_value(&ev)))
                    />
                </div>
                
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Category"
                    </Text>
                    <select 
                        class="w-full p-2 border border-gray-300 rounded-md focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                        prop:value=move || category.get()
                        on:change=move |ev| category.set(event_target_value(&ev))
                        prop:disabled=move || submit_action.pending().get()
                        required
                    >
                        <option value="">"Select a category"</option>
                        {move || categories.get().into_iter().map(|cat| view! {
                            <option value=cat.id.clone()>{cat.name}</option>
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Price ($)"
                    </Text>
                    <Input
                        input_type=InputType::Number
                        size=Size::Md
                        intent=Intent::Primary
                        value=price
                        placeholder="0.00"
                        required=true
                        state=if is_submitting.get() { ComponentState::Disabled } else { ComponentState::Enabled }
                        on_input=Callback::new(move |ev| price.set(event_target_value(&ev)))
                    />
                </div>
                
                <Button
                    size=Size::Md
                    intent=Intent::Primary
                    state=if is_submitting.get() { ComponentState::Loading } else if is_form_valid.get() { ComponentState::Enabled } else { ComponentState::Disabled }
                >
                    {move || if is_submitting.get() { "Adding..." } else { "Add Item" }}
                </Button>
            </form>
        </Card>
    }
}