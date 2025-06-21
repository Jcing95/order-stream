use leptos::prelude::*;
use crate::common::types::{Order, OrderStatus, OrderItem, Item};
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn OrderCard<F>(
    order: Order,
    order_items: Signal<Vec<OrderItem>>,
    items: Signal<Vec<Item>>,
    on_status_update: F,
    #[prop(optional)] show_status_controls: bool,
    #[prop(optional)] allowed_statuses: Vec<OrderStatus>,
) -> impl IntoView 
where
    F: Fn(String, OrderStatus) + 'static + Clone + Send + Sync,
{
    let show_status_controls = show_status_controls;
    let allowed_statuses = if allowed_statuses.is_empty() {
        vec![OrderStatus::Ordered, OrderStatus::Ready, OrderStatus::Completed, OrderStatus::Cancelled]
    } else {
        allowed_statuses
    };
    let allowed_statuses_clone = allowed_statuses.clone();

    let status_intent = |status: &OrderStatus| -> Intent {
        match status {
            OrderStatus::Draft => Intent::Secondary,
            OrderStatus::Ordered => Intent::Primary,
            OrderStatus::Ready => Intent::Warning,
            OrderStatus::Completed => Intent::Success,
            OrderStatus::Cancelled => Intent::Danger,
        }
    };

    let status_text = |status: &OrderStatus| -> &'static str {
        match status {
            OrderStatus::Draft => "Draft",
            OrderStatus::Ordered => "Ordered",
            OrderStatus::Ready => "Ready", 
            OrderStatus::Completed => "Completed",
            OrderStatus::Cancelled => "Cancelled",
        }
    };

    let next_status = |current: &OrderStatus| -> Option<OrderStatus> {
        match current {
            OrderStatus::Draft => Some(OrderStatus::Ordered),
            OrderStatus::Ordered => Some(OrderStatus::Ready),
            OrderStatus::Ready => Some(OrderStatus::Completed),
            OrderStatus::Completed => None,
            OrderStatus::Cancelled => None,
        }
    };

    let can_cancel = |status: &OrderStatus| -> bool {
        matches!(status, OrderStatus::Draft | OrderStatus::Ordered | OrderStatus::Ready)
    };

    let get_item_details = move |item_id: &str| -> Option<Item> {
        items.get().into_iter().find(|item| item.id == item_id)
    };

    let order_id_clone = order.id.clone();
    let on_status_update_clone = on_status_update.clone();

    view! {
        <Card variant=CardVariant::Default>
            <div class="space-y-4">
                // Order Header
                <div class="flex items-start justify-between">
                    <div class="space-y-2">
                        <div class="flex items-center gap-3">
                            <Text 
                                variant=TextVariant::Heading 
                                size=Size::Lg 
                                weight=FontWeight::Semibold
                            >
                                "Order #" {format!("{:03}", order.sequential_id)}
                            </Text>
                            <Text 
                                variant=TextVariant::Caption 
                                size=Size::Sm 
                                weight=FontWeight::Medium
                                intent=status_intent(&order.status)
                                class="px-3 py-1 rounded-full bg-opacity-20"
                            >
                                {status_text(&order.status)}
                            </Text>
                        </div>
                        <Text 
                            variant=TextVariant::Body 
                            size=Size::Md 
                            weight=FontWeight::Medium
                        >
                            "Total: $" {format!("{:.2}", order.total_price)}
                        </Text>
                    </div>

                    // Status Control Buttons
                    {
                        if show_status_controls {
                            let order_id = order_id_clone.clone();
                            let on_update = on_status_update_clone.clone();
                            let current_status = order.status.clone();
                            let allowed_clone = allowed_statuses_clone.clone();
                            let allowed_clone2 = allowed_statuses_clone.clone();
                            
                            // Clone for each closure
                            let order_id_next = order_id.clone();
                            let on_update_next = on_update.clone();
                            let current_status_next = current_status.clone();
                            
                            let order_id_cancel = order_id.clone();
                            let on_update_cancel = on_update.clone();
                            let current_status_cancel = current_status.clone();
                            
                            view! {
                                <div class="flex gap-2">
                                    // Next Status Button
                                    {move || {
                                        if let Some(next) = next_status(&current_status_next) {
                                            if allowed_clone.contains(&next) {
                                                let next_clone = next.clone();
                                                let order_id_next_inner = order_id_next.clone();
                                                let on_update_next_inner = on_update_next.clone();
                                                
                                                view! {
                                                    <Button
                                                        size=Size::Sm
                                                        intent=Intent::Primary
                                                        on_click=Callback::new(move |_| {
                                                            on_update_next_inner(order_id_next_inner.clone(), next_clone.clone())
                                                        })
                                                    >
                                                        {match next {
                                                            OrderStatus::Ordered => "Mark Ordered",
                                                            OrderStatus::Ready => "Mark Ready",
                                                            OrderStatus::Completed => "Complete",
                                                            _ => "Update",
                                                        }}
                                                    </Button>
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}

                                    // Cancel Button
                                    {move || {
                                        if can_cancel(&current_status_cancel) && allowed_clone2.contains(&OrderStatus::Cancelled) {
                                            let order_id_cancel_inner = order_id_cancel.clone();
                                            let on_update_cancel_inner = on_update_cancel.clone();
                                            
                                            view! {
                                                <Button
                                                    size=Size::Sm
                                                    intent=Intent::Danger
                                                    on_click=Callback::new(move |_| {
                                                        on_update_cancel_inner(order_id_cancel_inner.clone(), OrderStatus::Cancelled)
                                                    })
                                                >
                                                    "Cancel"
                                                </Button>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                </div>

                // Order Items
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Body 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                    >
                        "Items:"
                    </Text>
                    
                    {move || {
                        let current_order_items: Vec<OrderItem> = order_items.get()
                            .into_iter()
                            .filter(|oi| oi.order_id == order.id)
                            .collect();
                            
                        if current_order_items.is_empty() {
                            view! {
                                <Alert intent=Intent::Info size=Size::Sm>
                                    "No items in this order"
                                </Alert>
                            }.into_any()
                        } else {
                            current_order_items.into_iter().map(|order_item| {
                                let item_details = get_item_details(&order_item.item_id);
                                
                                view! {
                                    <div class="flex justify-between items-center py-2 border-b border-opacity-20">
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Medium
                                            >
                                                {match item_details {
                                                    Some(item) => item.name,
                                                    None => format!("Item {}", order_item.item_id),
                                                }}
                                            </Text>
                                            <Text 
                                                variant=TextVariant::Caption 
                                                size=Size::Xs 
                                                weight=FontWeight::Normal
                                            >
                                                "Qty: " {order_item.quantity.to_string()} 
                                                " × $" {format!("{:.2}", order_item.price)}
                                            </Text>
                                        </div>
                                        <Text 
                                            variant=TextVariant::Body 
                                            size=Size::Sm 
                                            weight=FontWeight::Medium
                                        >
                                            "$" {format!("{:.2}", order_item.price * order_item.quantity as f64)}
                                        </Text>
                                    </div>
                                }
                            }).collect_view().into_any()
                        }
                    }}
                </div>
            </div>
        </Card>
    }
}