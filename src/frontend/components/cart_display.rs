use leptos::prelude::*;
use crate::common::types::Item;
use crate::frontend::design_system::{
    Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight}
};

#[component]
pub fn CartDisplay(
    cart_items: Signal<Vec<(Item, u32)>>,
    on_remove_from_cart: Callback<String>, // item_id
    on_process_payment: Callback<leptos::ev::MouseEvent>,
    on_cancel_order: Callback<leptos::ev::MouseEvent>,
    is_processing: ReadSignal<bool>,
) -> impl IntoView {
    // Calculate cart total
    let cart_total = Signal::derive(move || {
        cart_items.get().iter().map(|(item, qty)| item.price * (*qty as f64)).sum::<f64>()
    });

    view! {
        <div class="space-y-4">
            <Text 
                variant=TextVariant::Body 
                size=Size::Md 
                weight=FontWeight::Medium
            >
                "Cart Items:"
            </Text>
            
            // Cart items list
            {move || {
                let cart = cart_items.get();
                if cart.is_empty() {
                    view! {
                        <Alert intent=Intent::Info size=Size::Sm>
                            "Cart is empty"
                        </Alert>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-2">
                            {cart.into_iter().map(|(item, quantity)| {
                                let _item_id = item.id.clone();
                                view! {
                                    <CartItem
                                        item=item
                                        quantity=quantity
                                        on_remove=move |id| on_remove_from_cart.run(id)
                                    />
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }}

            // Cart total and actions
            {move || {
                if !cart_items.get().is_empty() {
                    view! {
                        <div class="space-y-4 border-t pt-4">
                            <div class="flex justify-between items-center">
                                <Text 
                                    variant=TextVariant::Body 
                                    size=Size::Lg 
                                    weight=FontWeight::Bold
                                >
                                    "Total: $" {move || format!("{:.2}", cart_total.get())}
                                </Text>
                            </div>
                            
                            <div class="flex gap-2">
                                <Button
                                    size=Size::Md
                                    intent=Intent::Primary
                                    on_click=on_process_payment
                                >
                                    {move || if is_processing.get() { "Processing..." } else { "Process Payment" }}
                                </Button>
                                
                                <Button
                                    size=Size::Md
                                    intent=Intent::Danger
                                    on_click=on_cancel_order
                                >
                                    "Cancel Order"
                                </Button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn CartItem(
    item: Item,
    quantity: u32,
    on_remove: impl Fn(String) + 'static + Clone + Send + Sync,
) -> impl IntoView {
    let item_id = item.id.clone();
    let handle_remove = move |_| {
        on_remove(item_id.clone());
    };

    view! {
        <div class="flex justify-between items-center py-2 border-b border-opacity-20">
            <div>
                <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                    {item.name}
                </Text>
                <Text variant=TextVariant::Caption size=Size::Xs>
                    "Qty: " {quantity.to_string()} " × $" {format!("{:.2}", item.price)}
                </Text>
            </div>
            <div class="flex items-center gap-2">
                <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                    "$" {format!("{:.2}", item.price * quantity as f64)}
                </Text>
                <Button
                    size=Size::Xs
                    intent=Intent::Danger
                    on_click=Callback::new(handle_remove)
                >
                    "×"
                </Button>
            </div>
        </div>
    }
}