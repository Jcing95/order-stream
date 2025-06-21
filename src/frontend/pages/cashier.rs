use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{Order, OrderStatus, Item, CreateOrderItemRequest};
use crate::frontend::components::order_card::OrderCard;
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert, Input,
    theme::{Size, Intent, ComponentState},
    atoms::{TextVariant, FontWeight, InputType},
};
use crate::backend::services::{
    orders::{create_order, get_orders, update_order_status},
    order_items::{create_order_item, get_all_order_items},
    items::get_items,
    categories::get_categories,
};

#[component]
pub fn CashierPage() -> impl IntoView {
    // State for current draft order
    let current_order = RwSignal::new(None::<Order>);
    let cart_items = RwSignal::new(Vec::<(Item, u32)>::new());
    let selected_item_id = RwSignal::new(String::new());
    let quantity_input = RwSignal::new(1u32);
    let quantity_string = RwSignal::new("1".to_string());
    
    // Data signals
    let orders = Resource::new(|| (), |_| get_orders());
    let order_items = Resource::new(|| (), |_| get_all_order_items());
    let items = Resource::new(|| (), |_| get_items());
    let categories = Resource::new(|| (), |_| get_categories());
    
    // Error and loading state
    let error_message = RwSignal::new(None::<String>);
    let is_creating_order = RwSignal::new(false);

    // Create new draft order
    let create_new_order = Action::new(move |_: &()| async move {
        is_creating_order.set(true);
        error_message.set(None);
        
        match create_order().await {
            Ok(order) => {
                current_order.set(Some(order));
                cart_items.set(Vec::new());
                error_message.set(None);
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to create order: {}", e)));
            }
        }
        
        is_creating_order.set(false);
    });

    // Add item to cart
    let add_to_cart = Action::new(move |_: &()| async move {
        let item_id = selected_item_id.get();
        let quantity = quantity_input.get();
        
        if item_id.is_empty() || quantity == 0 {
            error_message.set(Some("Please select an item and quantity".to_string()));
            return;
        }
        
        if let Some(items_list) = items.get().and_then(|r| r.ok()) {
            if let Some(item) = items_list.iter().find(|i| i.id == item_id) {
                let mut cart = cart_items.get();
                
                // Check if item already in cart
                if let Some(existing) = cart.iter_mut().find(|(i, _)| i.id == item_id) {
                    existing.1 += quantity;
                } else {
                    cart.push((item.clone(), quantity));
                }
                
                cart_items.set(cart);
                selected_item_id.set(String::new());
                quantity_input.set(1);
                quantity_string.set("1".to_string());
                error_message.set(None);
            }
        }
    });

    // Process payment and finalize order
    let process_payment = Action::new(move |_: &()| async move {
        if let Some(order) = current_order.get() {
            is_creating_order.set(true);
            error_message.set(None);
            
            // Add all cart items to the order
            let cart = cart_items.get();
            let mut _total_price = 0.0;
            
            for (item, quantity) in cart.iter() {
                let request = CreateOrderItemRequest {
                    order_id: order.id.clone(),
                    item_id: item.id.clone(),
                    quantity: *quantity,
                };
                
                match create_order_item(request).await {
                    Ok(_) => {
                        _total_price += item.price * (*quantity as f64);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to add item: {}", e)));
                        is_creating_order.set(false);
                        return;
                    }
                }
            }
            
            // Update order status to Ordered (payment processed)
            match update_order_status(order.id.clone(), OrderStatus::Ordered).await {
                Ok(_) => {
                    current_order.set(None);
                    cart_items.set(Vec::new());
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

    // Remove item from cart
    let remove_from_cart = move |item_id: String| {
        let mut cart = cart_items.get();
        cart.retain(|(item, _)| item.id != item_id);
        cart_items.set(cart);
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

    // Calculate cart total
    let cart_total = Signal::derive(move || {
        cart_items.get().iter().map(|(item, qty)| item.price * (*qty as f64)).sum::<f64>()
    });

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
                    <Card variant=CardVariant::Default>
                        <div class="space-y-4">
                            <Text 
                                variant=TextVariant::Heading 
                                size=Size::Lg 
                                weight=FontWeight::Semibold
                            >
                                "New Order"
                            </Text>

                            // Create order or show current order
                            {move || {
                                if let Some(order) = current_order.get() {
                                    view! {
                                        <div class="space-y-4">
                                            <Alert intent=Intent::Info size=Size::Md>
                                                "Creating Order #" {format!("{:03}", order.sequential_id)}
                                            </Alert>

                                            // Item selection
                                            <div class="space-y-4">
                                                {move || {
                                                    if let Some(Ok(items_list)) = items.get() {
                                                        if let Some(Ok(categories_list)) = categories.get() {
                                                            view! {
                                                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                                                    <div class="space-y-2">
                                                                        <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                                            "Item"
                                                                        </Text>
                                                                        <select 
                                                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                                            on:change=move |ev| selected_item_id.set(event_target_value(&ev))
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
                                                                            on_input=Callback::new(move |ev| {
                                                                                let val = event_target_value(&ev);
                                                                                quantity_string.set(val.clone());
                                                                                if let Ok(qty) = val.parse::<u32>() {
                                                                                    quantity_input.set(qty.max(1));
                                                                                }
                                                                            })
                                                                            size=Size::Md
                                                                            state=ComponentState::Enabled
                                                                        />
                                                                    </div>
                                                                </div>

                                                                <Button
                                                                    size=Size::Md
                                                                    intent=Intent::Secondary
                                                                    on_click=Callback::new(move |_| { add_to_cart.dispatch(()); })
                                                                >
                                                                    "Add to Cart"
                                                                </Button>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <Alert intent=Intent::Warning size=Size::Md>
                                                                    "Loading categories..."
                                                                </Alert>
                                                            }.into_any()
                                                        }
                                                    } else {
                                                        view! {
                                                            <Alert intent=Intent::Warning size=Size::Md>
                                                                "Loading items..."
                                                            </Alert>
                                                        }.into_any()
                                                    }
                                                }}
                                            </div>

                                            // Cart display
                                            <div class="space-y-2">
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Md 
                                                    weight=FontWeight::Medium
                                                >
                                                    "Cart Items:"
                                                </Text>
                                                
                                                {move || {
                                                    let cart = cart_items.get();
                                                    if cart.is_empty() {
                                                        view! {
                                                            <Alert intent=Intent::Info size=Size::Sm>
                                                                "Cart is empty"
                                                            </Alert>
                                                        }.into_any()
                                                    } else {
                                                        cart.into_iter().map(|(item, quantity)| {
                                                            let item_id = item.id.clone();
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
                                                                            on_click=Callback::new(move |_| remove_from_cart(item_id.clone()))
                                                                        >
                                                                            "×"
                                                                        </Button>
                                                                    </div>
                                                                </div>
                                                            }
                                                        }).collect_view().into_any()
                                                    }
                                                }}
                                            </div>

                                            // Cart total and payment
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
                                                        on_click=Callback::new(move |_| { process_payment.dispatch(()); })
                                                    >
                                                        {move || if is_creating_order.get() { "Processing..." } else { "Process Payment" }}
                                                    </Button>
                                                    
                                                    <Button
                                                        size=Size::Md
                                                        intent=Intent::Danger
                                                        on_click=Callback::new(move |_| {
                                                            current_order.set(None);
                                                            cart_items.set(Vec::new());
                                                        })
                                                    >
                                                        "Cancel Order"
                                                    </Button>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <Button
                                            size=Size::Lg
                                            intent=Intent::Primary
                                            on_click=Callback::new(move |_| { create_new_order.dispatch(()); })
                                        >
                                            {move || if is_creating_order.get() { "Creating..." } else { "Start New Order" }}
                                        </Button>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </Card>
                </div>

                // Recent Orders Panel
                <div class="space-y-4">
                    <Text 
                        variant=TextVariant::Heading 
                        size=Size::Lg 
                        weight=FontWeight::Semibold
                    >
                        "Recent Orders"
                    </Text>
                    
                    {move || {
                        match (orders.get(), order_items.get(), items.get()) {
                            (Some(Ok(orders_list)), Some(Ok(order_items_list)), Some(Ok(items_list))) => {
                                // Show only recent orders (last 10)
                                let mut recent_orders = orders_list;
                                recent_orders.sort_by(|a, b| b.sequential_id.cmp(&a.sequential_id));
                                recent_orders.truncate(10);
                                
                                if recent_orders.is_empty() {
                                    view! {
                                        <Alert intent=Intent::Info size=Size::Md>
                                            "No orders yet"
                                        </Alert>
                                    }.into_any()
                                } else {
                                    recent_orders.into_iter().map(|order| {
                                        let order_items_data = order_items_list.clone();
                                        let items_data = items_list.clone();
                                        view! {
                                            <OrderCard
                                                order=order
                                                order_items=Signal::derive(move || order_items_data.clone())
                                                items=Signal::derive(move || items_data.clone())
                                                on_status_update=update_status
                                                show_status_controls=true
                                                allowed_statuses=vec![OrderStatus::Cancelled]
                                            />
                                        }
                                    }).collect_view().into_any()
                                }
                            }
                            _ => {
                                view! {
                                    <Alert intent=Intent::Info size=Size::Md>
                                        "Loading orders..."
                                    </Alert>
                                }.into_any()
                            }
                        }
                    }}
                </div>
            </div>
        </div>
    }
}