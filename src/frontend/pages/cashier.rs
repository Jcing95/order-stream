use crate::backend::services::{
    categories::get_categories,
    items::get_items,
    order_items::{create_order_item, get_all_order_items, delete_order_item},
    orders::{create_order, delete_order, get_orders, update_order_status},
};
use crate::common::types::{CreateOrderItemRequest, Order, OrderStatus, Item, Category};
use crate::frontend::design_system::{
    atoms::{FontWeight, TextVariant},
    theme::{Intent, Size, ComponentState},
    Alert, Text, Button, Card, CardVariant,
};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn CashierPage() -> impl IntoView {
    // State for current draft order
    let current_order = RwSignal::new(None::<Order>);
    
    // State for pending item to add after order creation
    let pending_item = RwSignal::new(None::<(String, u32)>);

    // Data resources
    let orders = Resource::new(|| (), |_| get_orders());
    let order_items = Resource::new(|| (), |_| get_all_order_items());
    let items = Resource::new(|| (), |_| get_items());
    let categories = Resource::new(|| (), |_| get_categories());

    // Error and loading state
    let error_message = RwSignal::new(None::<String>);
    let is_creating_order = RwSignal::new(false);

    // Helper function to get cart items from resources (to be used within Suspense)
    // Returns (Item, quantity, order_item_ids) where order_item_ids are all the individual order item IDs for this item type
    let get_cart_items = move |current_order: Option<Order>, all_order_items: Vec<crate::common::types::OrderItem>, items_list: Vec<Item>| -> Vec<(Item, u32, Vec<String>)> {
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
    // We'll need to pass the order_item_id directly from the UI instead of looking it up here
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

    // Update order status (for completed orders) - unused in new interface
    let _update_status = move |order_id: String, new_status: OrderStatus| {
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
        <div class="h-screen flex flex-col">
            // Header
            <div class="bg-white dark:bg-slate-900 border-b border-gray-200 dark:border-slate-700 p-4">
                <div class="flex justify-between items-center">
                    <Text
                        variant=TextVariant::Heading
                        size=Size::Lg
                        weight=FontWeight::Bold
                    >
                        "Cashier Station"
                    </Text>
                    {move || {
                        if let Some(order) = current_order.get() {
                            view! {
                                <div class="text-right">
                                    <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary>
                                        "Order #" {format!("{:03}", order.sequential_id)}
                                    </Text>
                                    <Suspense fallback=|| view! { <Text variant=TextVariant::Body size=Size::Lg>"Total: $0.00"</Text> }>
                                        {move || {
                                            match (order_items.get(), items.get()) {
                                                (Some(Ok(all_order_items)), Some(Ok(items_list))) => {
                                                    let cart_items = get_cart_items(Some(order.clone()), all_order_items, items_list);
                                                    let total = cart_items.iter()
                                                        .map(|(item, qty, _)| item.price * (*qty as f64))
                                                        .sum::<f64>();
                                                    view! {
                                                        <Text variant=TextVariant::Body size=Size::Lg weight=FontWeight::Semibold>
                                                            "Total: $" {format!("{:.2}", total)}
                                                        </Text>
                                                    }.into_any()
                                                }
                                                _ => view! {
                                                    <Text variant=TextVariant::Body size=Size::Lg weight=FontWeight::Semibold>
                                                        "Total: $0.00"
                                                    </Text>
                                                }.into_any()
                                            }
                                        }}
                                    </Suspense>
                                </div>
                            }.into_any()
                        } else if is_creating_order.get() || pending_item.get().is_some() {
                            view! {
                                <Text variant=TextVariant::Body size=Size::Md intent=Intent::Secondary>
                                    "Creating order..."
                                </Text>
                            }.into_any()
                        } else {
                            view! {
                                <Button
                                    size=Size::Lg
                                    intent=Intent::Primary
                                    on_click=Callback::new(move |_| { create_new_order.dispatch(()); })
                                    state=ComponentState::Enabled
                                >
                                    "Start New Order"
                                </Button>
                            }.into_any()
                        }
                    }}
                </div>

                // Error display
                {move || {
                    error_message.get().map(|msg| {
                        view! {
                            <div class="mt-2">
                                <Alert intent=Intent::Danger size=Size::Sm>
                                    {msg}
                                </Alert>
                            </div>
                        }
                    })
                }}
            </div>

            // Main content area
            <div class="flex-1 flex overflow-hidden">
                // Category panes (left side)
                <div class="flex-1 p-4 overflow-y-auto">
                    <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                        {move || {
                            // Access resources safely within Suspense
                            match (categories.get(), items.get()) {
                                (Some(Ok(cats)), Some(Ok(items_list))) => {
                                    if cats.is_empty() {
                                        view! {
                                            <div class="text-center p-8">
                                                <Text variant=TextVariant::Body intent=Intent::Secondary>
                                                    "No categories available"
                                                </Text>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
                                                {cats.into_iter().map(|category| {
                                                    let category_items: Vec<Item> = items_list.iter()
                                                        .filter(|item| item.category_id == category.id)
                                                        .cloned()
                                                        .collect();
                                                    
                                                    view! {
                                                        <CategoryPane 
                                                            category=category
                                                            items=category_items
                                                            on_item_click=Callback::new(add_to_cart)
                                                            current_order=current_order.read_only()
                                                            pending_item=pending_item
                                                            is_creating_order=is_creating_order.read_only()
                                                            create_new_order=create_new_order.clone()
                                                        />
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_any()
                                    }
                                }
                                _ => {
                                    view! {
                                        <div class="text-center p-8">
                                            <Text variant=TextVariant::Body intent=Intent::Secondary>
                                                "Loading categories and items..."
                                            </Text>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }}
                    </Suspense>
                </div>

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

#[component]
fn CategoryPane(
    category: Category,
    items: Vec<Item>,
    on_item_click: Callback<(String, u32)>,
    current_order: ReadSignal<Option<Order>>,
    pending_item: RwSignal<Option<(String, u32)>>,
    is_creating_order: ReadSignal<bool>,
    create_new_order: Action<(), ()>,
) -> impl IntoView {
    view! {
        <Card variant=CardVariant::Default>
            <div class="p-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Md 
                    weight=FontWeight::Semibold
                    class="mb-4"
                >
                    {category.name}
                </Text>
                
                <div class="grid grid-cols-2 gap-3">
                    {items.into_iter().map(|item| {
                        let item_id = item.id.clone();
                        let item_name = item.name.clone();
                        let item_price = item.price;
                        
                        view! {
                            <button
                                class=move || {
                                    let base_class = "h-20 flex flex-col items-center justify-center text-center p-3 border rounded transition-colors";
                                    let is_creating = is_creating_order.get();
                                    let has_pending = pending_item.get().is_some();
                                    
                                    if is_creating || has_pending {
                                        format!("{} bg-gray-100 border-gray-300 cursor-wait opacity-75 dark:bg-slate-700 dark:border-slate-600", base_class)
                                    } else {
                                        format!("{} bg-white hover:bg-green-50 dark:bg-slate-800 dark:hover:bg-green-900/20 border-gray-300 dark:border-slate-600 hover:border-green-300 dark:hover:border-green-600 cursor-pointer", base_class)
                                    }
                                }
                                on:click=move |_| {
                                    if !is_creating_order.get_untracked() && pending_item.get_untracked().is_none() {
                                        // Auto-create order if none exists
                                        if current_order.get_untracked().is_none() {
                                            // Store pending item and create order
                                            pending_item.set(Some((item_id.clone(), 1)));
                                            create_new_order.dispatch(());
                                        } else {
                                            // Order exists, add item directly
                                            on_item_click.run((item_id.clone(), 1));
                                        }
                                    }
                                }
                                disabled=move || is_creating_order.get() || pending_item.get().is_some()
                            >
                                <Text 
                                    variant=TextVariant::Body 
                                    size=Size::Sm 
                                    weight=FontWeight::Semibold
                                    class="mb-1"
                                >
                                    {item_name.clone()}
                                </Text>
                                <Text 
                                    variant=TextVariant::Caption 
                                    size=Size::Xs
                                    intent=Intent::Success
                                >
                                    "$" {format!("{:.2}", item_price)}
                                </Text>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </Card>
    }
}

#[component]
fn CartSidebar(
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
