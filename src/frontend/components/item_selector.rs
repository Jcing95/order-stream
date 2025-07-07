use leptos::prelude::*;
use crate::common::types::{Item, Category};
use crate::frontend::design_system::{
    Button, Text, Alert, Input,
    theme::{Size, Intent, ComponentState},
    atoms::{TextVariant, FontWeight, InputType}
};

#[component]
pub fn ItemSelector(
    items: Signal<Vec<Item>>,
    categories: Signal<Vec<Category>>,
    on_add_to_cart: Callback<(String, u32)>, // (item_id, quantity)
    _error_message: ReadSignal<Option<String>>,
) -> impl IntoView {
    let selected_item_id = RwSignal::new(String::new());
    let quantity_input = RwSignal::new(1u32);
    let quantity_string = RwSignal::new("1".to_string());

    // Derived signal for form validation
    let can_add_to_cart = Signal::derive(move || {
        !selected_item_id.get().is_empty() && quantity_input.get() > 0
    });

    // Action for adding to cart with proper error handling
    let add_to_cart_action = Action::new(move |_: &()| {
        let item_id = selected_item_id.get_untracked();
        let quantity = quantity_input.get_untracked();
        
        async move {
            if can_add_to_cart.get_untracked() {
                on_add_to_cart.run((item_id, quantity));
                // Reset form after successful add
                selected_item_id.set(String::new());
                quantity_input.set(1);
                quantity_string.set("1".to_string());
                Ok(())
            } else {
                Err("Please select an item and valid quantity")
            }
        }
    });

    // Event handler for native select element - must use closure pattern
    let handle_item_change = move |ev| {
        selected_item_id.set(event_target_value(&ev));
    };

    let handle_quantity_change = Callback::new(move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        quantity_string.set(val.clone());
        if let Ok(qty) = val.parse::<u32>() {
            quantity_input.set(qty.max(1));
        }
    });
    
    let handle_add_click = Callback::new(move |_: leptos::ev::MouseEvent| {
        add_to_cart_action.dispatch(());
    });

    view! {
        <div class="space-y-4">
            {move || {
                let items_list = items.get();
                let categories_list = categories.get();
                
                if items_list.is_empty() {
                    view! {
                        <Alert intent=Intent::Warning size=Size::Md>
                            "Loading items..."
                        </Alert>
                    }.into_any()
                } else if categories_list.is_empty() {
                    view! {
                        <Alert intent=Intent::Warning size=Size::Md>
                            "Loading categories..."
                        </Alert>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="space-y-2">
                                <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                    "Item"
                                </Text>
                                <select 
                                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    on:change=handle_item_change
                                    prop:value=move || selected_item_id.get()
                                >
                                    <option value="">"Select an item..."</option>
                                    {categories_list.iter().map(|category| {
                                        let category_items: Vec<_> = items_list.iter()
                                            .filter(|item| item.category_id == category.id && item.active)
                                            .collect();
                                        
                                        if category_items.is_empty() {
                                            view! {}.into_any()
                                        } else {
                                            view! {
                                                <optgroup label=&category.name>
                                                    {category_items.into_iter().map(|item| {
                                                        view! {
                                                            <option value=&item.id>
                                                                {item.name.clone()} " - $" {format!("{:.2}", item.price)}
                                                            </option>
                                                        }
                                                    }).collect_view()}
                                                </optgroup>
                                            }.into_any()
                                        }
                                    }).collect_view()}
                                </select>
                            </div>

                            <div class="space-y-2">
                                <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                    "Quantity"
                                </Text>
                                <Input
                                    input_type=InputType::Number
                                    value=quantity_string
                                    on_input=handle_quantity_change
                                    size=Size::Md
                                    state=ComponentState::Enabled
                                />
                            </div>
                        </div>

                        <Button
                            size=Size::Md
                            intent=Intent::Primary
                            on_click=handle_add_click
                            state=if add_to_cart_action.pending().get() { 
                                ComponentState::Loading 
                            } else if can_add_to_cart.get() { 
                                ComponentState::Enabled 
                            } else { 
                                ComponentState::Disabled 
                            }
                        >
                            {move || if add_to_cart_action.pending().get() { "Adding..." } else { "Add to Cart" }}
                        </Button>
                    }.into_any()
                }
            }}
        </div>
    }
}