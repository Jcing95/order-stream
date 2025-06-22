use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::backend::services::{
    categories::get_categories,
    items::get_items,
    order_items::{create_order_item, get_all_order_items, delete_order_item},
    orders::{create_order, delete_order, get_orders, update_order_status},
};
use crate::common::types::{CreateOrderItemRequest, Order, OrderStatus, Item, OrderItem};
use crate::frontend::components::{
    cashier_header::CashierHeader,
    category_grid::CategoryGrid,
    cart_sidebar::CartSidebar,
};

#[component]
pub fn CashierPage() -> impl IntoView {
    // State for current draft order
    let current_order = RwSignal::new(None::<Order>);
    
    // State for pending item to add after order creation
    let pending_item = RwSignal::new(None::<(String, u32)>);

    // Error and loading state
    let error_message = RwSignal::new(None::<String>);
    let is_creating_order = RwSignal::new(false);

    // Data resources
    let orders = Resource::new(|| (), |_| get_orders());
    let order_items = Resource::new(|| (), |_| get_all_order_items());
    let items = Resource::new(|| (), |_| get_items());
    let categories = Resource::new(|| (), |_| get_categories());

    // Create new draft order
    let create_new_order = Action::new(move |_: &()| async move {
        is_creating_order.set(true);
        error_message.set(None);

        match create_order().await {
            Ok(order) => {
                current_order.set(Some(order.clone()));
                error_message.set(None);
                
                // If there's a pending item, add it now
                if let Some((item_id, quantity)) = pending_item.get_untracked() {
                    pending_item.set(None);
                    
                    let request = CreateOrderItemRequest {
                        order_id: order.id,
                        item_id,
                        quantity,
                    };

                    match create_order_item(request).await {
                        Ok(_) => {
                            order_items.refetch();
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Failed to add item: {}", e)));
                        }
                    }
                }
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to create order: {}", e)));
                pending_item.set(None); // Clear pending item on error
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
        if let Some(order) = current_order.get_untracked() {
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
    let remove_from_cart = move |order_item_id: String| {
        spawn_local(async move {
            match delete_order_item(order_item_id).await {
                Ok(_) => {
                    order_items.refetch();
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to remove item: {}", e)));
                }
            }
        });
    };

    // Cancel order (delete the draft order)
    let cancel_order = move |_: leptos::ev::MouseEvent| {
        if let Some(order) = current_order.get_untracked() {
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

    // Helper function to get cart items from resources (to be used within Suspense)
    let get_cart_items = move |current_order: Option<Order>, all_order_items: Vec<OrderItem>, items_list: Vec<Item>| -> Vec<(Item, u32, Vec<String>)> {
        if let Some(order) = current_order {
            // Filter order items for current order
            let current_items: Vec<_> = all_order_items
                .into_iter()
                .filter(|oi| oi.order_id == order.id)
                .collect();

            // Group order items by item_id and collect quantities and IDs
            let mut item_data: std::collections::HashMap<String, (u32, Vec<String>)> = std::collections::HashMap::new();
            
            for order_item in current_items {
                let entry = item_data.entry(order_item.item_id.clone()).or_insert((0, Vec::new()));
                entry.0 += order_item.quantity;
                entry.1.push(order_item.id);
            }

            // Convert to cart format with grouped quantities and order item IDs
            item_data
                .into_iter()
                .filter_map(|(item_id, (total_quantity, order_item_ids))| {
                    items_list
                        .iter()
                        .find(|item| item.id == item_id)
                        .map(|item| (item.clone(), total_quantity, order_item_ids))
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    };

    view! {
        <div class="h-screen flex flex-col">
            // Header with order info and total
            <Suspense fallback=|| view! { <div class="p-4">"Loading header..."</div> }>
                {move || {
                    match (order_items.get(), items.get()) {
                        (Some(Ok(all_order_items)), Some(Ok(items_list))) => {
                            let cart_items = get_cart_items(
                                current_order.get(), 
                                all_order_items, 
                                items_list
                            );
                            let total = cart_items.iter()
                                .map(|(item, qty, _)| item.price * (*qty as f64))
                                .sum::<f64>();
                            
                            view! {
                                <CashierHeader
                                    current_order=current_order.read_only()
                                    is_creating_order=is_creating_order.read_only()
                                    pending_item=pending_item.read_only()
                                    error_message=error_message.read_only()
                                    total=total
                                    on_create_order=Callback::new(move |_| { create_new_order.dispatch(()); })
                                />
                            }.into_any()
                        }
                        _ => view! {
                            <div class="p-4">"Loading header..."</div>
                        }.into_any()
                    }
                }}
            </Suspense>

            // Main content area
            <div class="flex-1 flex overflow-hidden">
                // Category panes (left side)
                <Suspense fallback=|| view! { <div class="flex-1 p-4">"Loading categories..."</div> }>
                    {move || {
                        match (categories.get(), items.get()) {
                            (Some(Ok(cats)), Some(Ok(items_list))) => {
                                view! {
                                    <CategoryGrid
                                        categories=cats
                                        items=items_list
                                        current_order=current_order.read_only()
                                        pending_item=pending_item
                                        is_creating_order=is_creating_order.read_only()
                                        on_item_click=Callback::new(add_to_cart)
                                        create_new_order=create_new_order.clone()
                                    />
                                }.into_any()
                            }
                            _ => view! {
                                <div class="flex-1 p-4">"Loading categories..."</div>
                            }.into_any()
                        }
                    }}
                </Suspense>

                // Cart sidebar (right side)
                <Suspense fallback=|| view! { <div></div> }>
                    {move || {
                        if let Some(order) = current_order.get() {
                            match (order_items.get(), items.get()) {
                                (Some(Ok(all_order_items)), Some(Ok(items_list))) => {
                                    let cart_items = get_cart_items(Some(order.clone()), all_order_items, items_list);
                                    view! {
                                        <div class="w-80 bg-gray-50 dark:bg-slate-800 border-l border-gray-200 dark:border-slate-700 flex flex-col">
                                            <CartSidebar 
                                                cart_items=cart_items
                                                on_remove_item=Callback::new(remove_from_cart)
                                                on_add_to_cart=Callback::new(add_to_cart)
                                                on_process_payment=Callback::new(move |_| { process_payment.dispatch(()); })
                                                on_cancel_order=Callback::new(cancel_order)
                                                is_processing=is_creating_order.get()
                                            />
                                        </div>
                                    }.into_any()
                                }
                                _ => view! {
                                    <div class="w-80 bg-gray-50 dark:bg-slate-800 border-l border-gray-200 dark:border-slate-700 flex flex-col">
                                        <div class="p-4">"Loading cart..."</div>
                                    </div>
                                }.into_any()
                            }
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}