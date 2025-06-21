use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::{CreateItemRequest, Category};
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{InputType, TextVariant, FontWeight},
};

#[component]
pub fn ItemForm<F>(
    categories: ReadSignal<Vec<Category>>,
    on_submit: F,
) -> impl IntoView 
where
    F: Fn(CreateItemRequest) + 'static + Clone + Send,
{
    let name = RwSignal::new(String::new());
    let category = RwSignal::new(String::new());
    let price = RwSignal::new(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    let on_submit_clone = on_submit.clone();
    let submit_form = move |ev: web_sys::SubmitEvent| {
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

        // Validate
        if let Err(err) = request.validate() {
            set_error.set(Some(err));
            return;
        }

        // Submit
        on_submit_clone(request);
        
        // Clear form
        name.set(String::new());
        category.set(String::new());
        price.set(String::new());
    };

    view! {
        <Card variant=CardVariant::Default>
            <form on:submit=submit_form class="space-y-4">
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
                        placeholder="e.g., Coffee, Sandwich, Pizza"
                        required=true
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
                    // Use a regular HTML select for now due to reactivity constraints
                    <select
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        prop:value=move || category.get()
                        on:change=move |ev| category.set(event_target_value(&ev))
                        required
                    >
                        <option value="">"Select a category..."</option>
                        {move || {
                            categories.get().into_iter().map(|cat| {
                                view! {
                                    <option value={cat.id.clone()}>{cat.name.clone()}</option>
                                }
                            }).collect_view()
                        }}
                    </select>
                </div>
                
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Price"
                    </Text>
                    <Input
                        input_type=InputType::Number
                        size=Size::Md
                        intent=Intent::Primary
                        value=price
                        placeholder="0.00"
                        required=true
                        on_input=Callback::new(move |ev| price.set(event_target_value(&ev)))
                    />
                </div>
                
                <Button
                    size=Size::Md
                    intent=Intent::Primary
                    on_click=Callback::new(move |_| {
                        // The form submit will handle this
                    })
                >
                    "Add Item"
                </Button>
            </form>
        </Card>
    }
}