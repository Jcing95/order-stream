use crate::backend::services::{
    categories::get_categories,
    items::get_items,
    order_items::{create_order_item, get_all_order_items},
    orders::{create_order, delete_order, get_orders, update_order_status},
};
use crate::common::types::{CreateOrderItemRequest, Order, OrderStatus};
use crate::frontend::components::{
    order_creation_panel::OrderCreationPanel, recent_orders_panel::RecentOrdersPanel,
};
use crate::frontend::design_system::{
    atoms::{FontWeight, TextVariant},
    theme::{Intent, Size},
    Alert, Text,
};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn CashierPage() -> impl IntoView {
    // State for current draft order
    let current_order = RwSignal::new(None::<Order>);

    // Data resources
    let orders = Resource::new(|| (), |_| get_orders());
    let order_items = Resource::new(|| (), |_| get_all_order_items());
    let items = Resource::new(|| (), |_| get_items());
    let categories = Resource::new(|| (), |_| get_categories());

    // Error and loading state
    let error_message = RwSignal::new(None::<String>);
    let is_creating_order = RwSignal::new(false);

    // Derived signals for components
    let items_signal = Signal::derive(move || items.get().and_then(|r| r.ok()).unwrap_or_default());
    let categories_signal =
        Signal::derive(move || categories.get().and_then(|r| r.ok()).unwrap_or_default());

    // Get current order items (only for the current draft order)
    let current_order_items = Signal::derive(move || {
        if let Some(order) = current_order.get() {
            if let Some(Ok(all_order_items)) = order_items.get() {
                return all_order_items
                    .into_iter()
                    .filter(|oi| oi.order_id == order.id)
                    .collect::<Vec<_>>();
            }
        }
        Vec::new()
    });

    // Convert order items to cart format for compatibility
    let cart_items_signal = Signal::derive(move || {
        let current_items = current_order_items.get();
        let items_list = items_signal.get();

        current_items
            .into_iter()
            .filter_map(|order_item| {
                items_list
                    .iter()
                    .find(|item| item.id == order_item.item_id)
                    .map(|item| (item.clone(), order_item.quantity))
            })
            .collect::<Vec<_>>()
    });

    // Create new draft order
    let create_new_order = Action::new(move |_: &()| async move {
        is_creating_order.set(true);
        error_message.set(None);

        match create_order().await {
            Ok(order) => {
                current_order.set(Some(order));
                error_message.set(None);
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to create order: {}", e)));
            }
        }

        is_creating_order.set(false);
    });

    // Add item to cart (immediately to database)
    let add_to_cart = move |(item_id, quantity): (String, u32)| {
        if item_id.is_empty() || quantity == 0 {
            error_message.set(Some("Please select an item and quantity".to_string()));
            return;
        }

        if let Some(order) = current_order.get() {
            let order_id = order.id.clone();
            spawn_local(async move {
                let request = CreateOrderItemRequest {
                    order_id,
                    item_id,
                    quantity,
                };

                match create_order_item(request).await {
                    Ok(_) => {
                        order_items.refetch();
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to add item: {}", e)));
                    }
                }
            });
        } else {
            error_message.set(Some("No active order to add items to".to_string()));
        }
    };

    // Process payment (just change status from Draft to Ordered)
    let process_payment = Action::new(move |_: &()| async move {
        if let Some(order) = current_order.get() {
            is_creating_order.set(true);
            error_message.set(None);

            // Simply update order status to Ordered (payment processed)
            match update_order_status(order.id.clone(), OrderStatus::Ordered).await {
                Ok(_) => {
                    current_order.set(None);
                    orders.refetch();
                    order_items.refetch();
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to process payment: {}", e)));
                }
            }

            is_creating_order.set(false);
        }
    });

    // Remove item from cart (remove from database)
    let remove_from_cart = move |_order_item_id: String| {
        spawn_local(async move {
            // We'll need to implement delete_order_item service
            // For now, let's just refetch to show current state
            order_items.refetch();
        });
    };

    // Cancel order (delete the draft order)
    let cancel_order = move |_: leptos::ev::MouseEvent| {
        if let Some(order) = current_order.get() {
            let order_id = order.id.clone();
            spawn_local(async move {
                match delete_order(order_id).await {
                    Ok(_) => {
                        current_order.set(None);
                        orders.refetch();
                        order_items.refetch();
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to cancel order: {}", e)));
                    }
                }
            });
        } else {
            current_order.set(None);
        }
    };

    // Update order status (for completed orders)
    let update_status = move |order_id: String, new_status: OrderStatus| {
        spawn_local(async move {
            match update_order_status(order_id, new_status).await {
                Ok(_) => {
                    orders.refetch();
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to update order: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="container mx-auto p-6 space-y-6">
            <div class="flex justify-between items-center">
                <Text
                    variant=TextVariant::Heading
                    size=Size::Xl
                    weight=FontWeight::Bold
                >
                    "Cashier Station"
                </Text>
            </div>

            // Error display
            {move || {
                error_message.get().map(|msg| {
                    view! {
                        <Alert intent=Intent::Danger size=Size::Md>
                            {msg}
                        </Alert>
                    }
                })
            }}

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // Order Creation Panel
                <div class="space-y-6">
                    <OrderCreationPanel
                        current_order=current_order.read_only()
                        on_create_order=Callback::new(move |_: leptos::ev::MouseEvent| { create_new_order.dispatch(()); })
                        on_cancel_order=Callback::new(cancel_order)
                        is_creating=is_creating_order.read_only()
                        cart_items=cart_items_signal
                        on_add_to_cart=Callback::new(add_to_cart)
                        on_remove_from_cart=Callback::new(remove_from_cart)
                        on_process_payment=Callback::new(move |_: leptos::ev::MouseEvent| { process_payment.dispatch(()); })
                        items=items_signal
                        categories=categories_signal
                        error_message=error_message.read_only()
                    />
                </div>

                // Recent Orders Panel
                <div class="space-y-4">
                    <Suspense>
                    <RecentOrdersPanel
                        orders=Signal::derive(move || orders.get())
                        order_items=Signal::derive(move || order_items.get())
                        items=Signal::derive(move || items.get())
                        on_status_update=update_status
                    />
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
