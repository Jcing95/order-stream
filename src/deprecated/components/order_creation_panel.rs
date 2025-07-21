use leptos::prelude::*;
use crate::common::types::{Order, Item, Category};
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight}
};
use crate::frontend::components::{
    item_selector::ItemSelector,
    cart_display::CartDisplay,
};

#[component]
pub fn OrderCreationPanel(
    current_order: ReadSignal<Option<Order>>,
    on_create_order: Callback<leptos::ev::MouseEvent>,
    on_cancel_order: Callback<leptos::ev::MouseEvent>,
    is_creating: ReadSignal<bool>,
    
    // Cart management
    cart_items: Signal<Vec<(Item, u32)>>,
    on_add_to_cart: Callback<(String, u32)>, // (item_id, quantity)
    on_remove_from_cart: Callback<String>, // item_id
    on_process_payment: Callback<leptos::ev::MouseEvent>,
    
    // Data
    items: Signal<Vec<Item>>,
    categories: Signal<Vec<Category>>,
    
    // Error handling
    error_message: ReadSignal<Option<String>>,
) -> impl IntoView {
    
    view! {
        <Card variant=CardVariant::Default>
            <div class="space-y-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Lg 
                    weight=FontWeight::Semibold
                >
                    "New Order"
                </Text>

                {move || {
                    if let Some(order) = current_order.get() {
                        view! {
                            <div class="space-y-4">
                                <Alert intent=Intent::Info size=Size::Md>
                                    "Creating Order #" {format!("{:03}", order.sequential_id)}
                                </Alert>

                                // Item selection section
                                <ItemSelector
                                    items=items
                                    categories=categories
                                    on_add_to_cart=on_add_to_cart
                                    _error_message=error_message
                                />

                                // Cart display section
                                <CartDisplay
                                    cart_items=cart_items
                                    on_remove_from_cart=on_remove_from_cart
                                    on_process_payment=on_process_payment
                                    on_cancel_order=on_cancel_order
                                    is_processing=is_creating
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <Button
                                size=Size::Lg
                                intent=Intent::Primary
                                on_click=on_create_order
                            >
                                {move || if is_creating.get() { "Creating..." } else { "Start New Order" }}
                            </Button>
                        }.into_any()
                    }
                }}
            </div>
        </Card>
    }
}