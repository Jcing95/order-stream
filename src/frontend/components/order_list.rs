use leptos::prelude::*;
use crate::common::types::{Order, OrderStatus};
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn OrderList<F>(
    orders: ReadSignal<Vec<Order>>,
    on_delete: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Clone + Send + Sync,
{
    let on_delete_clone = on_delete.clone();

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

    view! {
        <div class="space-y-4">
            <Text 
                variant=TextVariant::Heading 
                size=Size::Lg 
                weight=FontWeight::Semibold
            >
                "Orders"
            </Text>
            
            {move || {
                let orders_list = orders.get();
                if orders_list.is_empty() {
                    view! {
                        <Alert intent=Intent::Info size=Size::Md>
                            "No orders yet. Create one to get started."
                        </Alert>
                    }.into_any()
                } else {
                    orders_list.into_iter().map(|order| {
                        let order_id = order.id.clone();
                        let on_delete_inner = on_delete_clone.clone();
                        let status_intent_value = status_intent(&order.status);
                        let status_label = status_text(&order.status);
                        
                        view! {
                            <Card variant=CardVariant::Default>
                                <div class="flex items-center justify-between">
                                    <div class="flex-1 space-y-2">
                                        <div class="flex items-center gap-3">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Md 
                                                weight=FontWeight::Medium
                                            >
                                                "Order #" {move || format!("{:03}", order.sequential_id)}
                                            </Text>
                                            <Text 
                                                variant=TextVariant::Caption 
                                                size=Size::Xs 
                                                weight=FontWeight::Medium
                                                intent=status_intent_value
                                                class="px-2 py-1 rounded-full bg-opacity-20"
                                            >
                                                {move || status_label}
                                            </Text>
                                        </div>
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Normal
                                            >
                                                "Total: $" {move || format!("{:.2}", order.total_price)}
                                            </Text>
                                            <Text 
                                                variant=TextVariant::Caption 
                                                size=Size::Xs 
                                                weight=FontWeight::Normal
                                            >
                                                "ID: " {move || order.id.clone()}
                                            </Text>
                                        </div>
                                    </div>
                                    <Button
                                        size=Size::Sm
                                        intent=Intent::Danger
                                        on_click=Callback::new(move |_| on_delete_inner(order_id.clone()))
                                    >
                                        "Delete"
                                    </Button>
                                </div>
                            </Card>
                        }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}