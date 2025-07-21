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
    current_order: ReadSignal<Option<crate::common::types::Order>>,
    is_creating_order: ReadSignal<bool>,
    on_create_order: Callback<leptos::ev::MouseEvent>,
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
                {move || {
                    if current_order.get().is_some() {
                        view! {
                            <Text 
                                variant=TextVariant::Heading 
                                size=Size::Md 
                                weight=FontWeight::Semibold
                            >
                                "Current Order"
                            </Text>
                        }.into_any()
                    } else if is_creating_order.get() {
                        view! {
                            <Text 
                                variant=TextVariant::Heading 
                                size=Size::Md 
                                weight=FontWeight::Semibold
                            >
                                "Creating Order..."
                            </Text>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-2">
                                <Text 
                                    variant=TextVariant::Heading 
                                    size=Size::Md 
                                    weight=FontWeight::Semibold
                                >
                                    "Order Cart"
                                </Text>
                                <Button
                                    size=Size::Lg
                                    intent=Intent::Primary
                                    on_click=on_create_order
                                    state=ComponentState::Enabled
                                >
                                    "Start New Order"
                                </Button>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Cart items
            <div class="flex-1 overflow-y-auto p-4">
                {
                    // Use untracked access to avoid reactive warnings since cart_items isn't reactive
                    if current_order.get_untracked().is_none() {
                        view! {
                            <div class="text-center py-8">
                                <Text variant=TextVariant::Body intent=Intent::Secondary>
                                    "Start a new order to add items to cart"
                                </Text>
                            </div>
                        }.into_any()
                    } else if cart_items.is_empty() {
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
                                                        <Button
                                                            size=Size::Xs
                                                            intent=Intent::Secondary
                                                            state=if quantity <= 1 { ComponentState::Disabled } else { ComponentState::Enabled }
                                                            on_click=Callback::new(move |_| {
                                                                if quantity > 1 {
                                                                    // Remove one item using the first order item ID
                                                                    on_remove_item.run(first_order_item_id.clone());
                                                                }
                                                            })
                                                        >
                                                            "−"
                                                        </Button>
                                                        
                                                        <Text 
                                                            variant=TextVariant::Body 
                                                            size=Size::Sm
                                                            weight=FontWeight::Medium
                                                            class="min-w-8 text-center"
                                                        >
                                                            {quantity.to_string()}
                                                        </Text>
                                                        
                                                        <Button
                                                            size=Size::Xs
                                                            intent=Intent::Success
                                                            on_click=Callback::new(move |_| {
                                                                // Add one more of this item
                                                                on_add_to_cart.run((item_id_add.clone(), 1));
                                                            })
                                                        >
                                                            "+"
                                                        </Button>
                                                    </div>
                                                    
                                                    <div class="flex items-center gap-1">
                                                        <Text 
                                                            variant=TextVariant::Caption 
                                                            size=Size::Xs
                                                            intent=Intent::Secondary
                                                        >
                                                            "$" {format!("{:.2}", item_price)} " each"
                                                        </Text>
                                                        <Button
                                                            size=Size::Xs
                                                            intent=Intent::Danger
                                                            on_click=Callback::new(move |_| {
                                                                // Remove all of this item type using all order item IDs
                                                                for order_item_id in order_item_ids.iter() {
                                                                    on_remove_item.run(order_item_id.clone());
                                                                }
                                                            })
                                                        >
                                                            "×"
                                                        </Button>
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

            // Cart footer with total and payment button - Only show when there's an active order
            {move || {
                if current_order.get().is_some() {
                    let total = cart_items.iter()
                        .map(|(item, qty, _)| item.price * (*qty as f64))
                        .sum::<f64>();
                    
                    view! {
                        <div class="p-4 border-t border-gray-200 dark:border-slate-600 space-y-3">
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
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}