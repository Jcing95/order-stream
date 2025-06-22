use leptos::prelude::*;
use crate::common::types::Item;
use crate::frontend::design_system::{
    atoms::{FontWeight, TextVariant},
    theme::{Intent, Size, ComponentState},
    Text, Button, Card, CardVariant,
};

#[component]
pub fn CartSidebar(
    cart_items: Vec<(Item, u32, Vec<String>)>,
    on_remove_item: Callback<String>,
    on_add_to_cart: Callback<(String, u32)>,
    on_process_payment: Callback<leptos::ev::MouseEvent>,
    on_cancel_order: Callback<leptos::ev::MouseEvent>,
    is_processing: bool,
) -> impl IntoView {
    view! {
        <div class="h-full flex flex-col">
            // Cart header
            <div class="p-4 border-b border-gray-200 dark:border-slate-600">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Md 
                    weight=FontWeight::Semibold
                >
                    "Current Order"
                </Text>
            </div>

            // Cart items
            <div class="flex-1 overflow-y-auto p-4">
                {
                    if cart_items.is_empty() {
                        view! {
                            <div class="text-center py-8">
                                <Text variant=TextVariant::Body intent=Intent::Secondary>
                                    "No items in cart"
                                </Text>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-2">
                                {cart_items.clone().into_iter().map(|(item, quantity, order_item_ids)| {
                                    let item_id = item.id.clone();
                                    let item_id_add = item_id.clone();
                                    let first_order_item_id = order_item_ids.first().cloned().unwrap_or_default();
                                    let item_price = item.price;
                                    let total_price = item_price * (quantity as f64);
                                    
                                    view! {
                                        <Card variant=CardVariant::Default>
                                            <div class="p-3">
                                                <div class="flex justify-between items-start mb-2">
                                                    <Text 
                                                        variant=TextVariant::Body 
                                                        size=Size::Sm 
                                                        weight=FontWeight::Medium
                                                        class="flex-1"
                                                    >
                                                        {item.name}
                                                    </Text>
                                                    <Text 
                                                        variant=TextVariant::Body 
                                                        size=Size::Sm
                                                        weight=FontWeight::Semibold
                                                        intent=Intent::Success
                                                    >
                                                        "$" {format!("{:.2}", total_price)}
                                                    </Text>
                                                </div>
                                                
                                                <div class="flex items-center justify-between">
                                                    <div class="flex items-center gap-2">
                                                        <button
                                                            class="w-6 h-6 flex items-center justify-center bg-gray-200 hover:bg-gray-300 dark:bg-slate-600 dark:hover:bg-slate-500 rounded text-sm font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                                            on:click=move |_| {
                                                                if quantity > 1 {
                                                                    // Remove one item using the first order item ID
                                                                    on_remove_item.run(first_order_item_id.clone());
                                                                }
                                                            }
                                                            disabled=move || quantity <= 1
                                                            title="Remove one"
                                                        >
                                                            "−"
                                                        </button>
                                                        
                                                        <Text 
                                                            variant=TextVariant::Body 
                                                            size=Size::Sm
                                                            weight=FontWeight::Medium
                                                            class="min-w-8 text-center"
                                                        >
                                                            {quantity.to_string()}
                                                        </Text>
                                                        
                                                        <button
                                                            class="w-6 h-6 flex items-center justify-center bg-green-200 hover:bg-green-300 dark:bg-green-700 dark:hover:bg-green-600 rounded text-sm font-semibold transition-colors"
                                                            on:click=move |_| {
                                                                // Add one more of this item
                                                                on_add_to_cart.run((item_id_add.clone(), 1));
                                                            }
                                                            title="Add one more"
                                                        >
                                                            "+"
                                                        </button>
                                                    </div>
                                                    
                                                    <div class="flex items-center gap-1">
                                                        <Text 
                                                            variant=TextVariant::Caption 
                                                            size=Size::Xs
                                                            intent=Intent::Secondary
                                                        >
                                                            "$" {format!("{:.2}", item_price)} " each"
                                                        </Text>
                                                        <button
                                                            class="ml-2 px-2 py-1 text-xs bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                                                            on:click=move |_| {
                                                                // Remove all of this item type using all order item IDs
                                                                for order_item_id in order_item_ids.iter() {
                                                                    on_remove_item.run(order_item_id.clone());
                                                                }
                                                            }
                                                            title="Remove all"
                                                        >
                                                            "×"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        </Card>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }
            </div>

            // Cart footer with total and payment button
            <div class="p-4 border-t border-gray-200 dark:border-slate-600 space-y-3">
                {
                    let total = cart_items.iter()
                        .map(|(item, qty, _)| item.price * (*qty as f64))
                        .sum::<f64>();
                    
                    view! {
                        <div class="space-y-3">
                            <div class="flex justify-between items-center">
                                <Text 
                                    variant=TextVariant::Body 
                                    size=Size::Lg 
                                    weight=FontWeight::Bold
                                >
                                    "Total:"
                                </Text>
                                <Text 
                                    variant=TextVariant::Body 
                                    size=Size::Lg 
                                    weight=FontWeight::Bold
                                    intent=Intent::Success
                                >
                                    "$" {format!("{:.2}", total)}
                                </Text>
                            </div>
                            
                            <div class="space-y-2">
                                <Button
                                    size=Size::Lg
                                    intent=Intent::Success
                                    on_click=on_process_payment
                                    state=if cart_items.is_empty() || is_processing { ComponentState::Disabled } else { ComponentState::Enabled }
                                >
                                    {if is_processing { "Processing..." } else { "Complete Order" }}
                                </Button>
                                
                                <Button
                                    size=Size::Md
                                    intent=Intent::Danger
                                    on_click=on_cancel_order
                                    state=if is_processing { ComponentState::Disabled } else { ComponentState::Enabled }
                                >
                                    "Cancel Order"
                                </Button>
                            </div>
                        </div>
                    }
                }
            </div>
        </div>
    }
}