use leptos::prelude::*;
use crate::common::types::{Order, OrderStatus};
use crate::frontend::state::theme::{card_surface, text_primary, text_secondary, text_muted, button_danger, button_small};

#[component]
pub fn OrderList<F>(
    orders: ReadSignal<Vec<Order>>,
    on_delete: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Clone + Send,
{
    let on_delete_clone = on_delete.clone();

    let status_badge_class = |status: &OrderStatus| -> &'static str {
        match status {
            OrderStatus::Draft => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300",
            OrderStatus::Ordered => "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300", 
            OrderStatus::Ready => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300",
            OrderStatus::Completed => "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300",
            OrderStatus::Cancelled => "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300",
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
            <h3 class=format!("text-lg font-semibold {}", text_primary())>"Orders"</h3>
            
            {move || {
                let orders_list = orders.get();
                if orders_list.is_empty() {
                    view! {
                        <div class=format!("italic p-4 {} {}", text_muted(), card_surface())>
                            "No orders yet. Create one to get started."
                        </div>
                    }.into_any()
                } else {
                    orders_list.into_iter().map(|order| {
                        let order_id = order.id.clone();
                        let on_delete_inner = on_delete_clone.clone();
                        let status_class = status_badge_class(&order.status);
                        let status_label = status_text(&order.status);
                        
                        view! {
                            <div class=format!("flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600")>
                                <div class="flex-1">
                                    <div class="flex items-center gap-3">
                                        <h4 class=format!("font-medium {}", text_primary())>
                                            "Order #" {move || format!("{:03}", order.sequential_id)}
                                        </h4>
                                        <span class={move || format!("px-2 py-1 text-xs font-medium rounded-full {}", status_class)}>
                                            {move || status_label}
                                        </span>
                                    </div>
                                    <div class=format!("text-sm mt-1 {}", text_secondary())>
                                        <p>"Total: $" {move || format!("{:.2}", order.total_price)}</p>
                                        <p class=format!("text-xs {}", text_muted())>"ID: " {move || order.id.clone()}</p>
                                    </div>
                                </div>
                                <button
                                    class=format!("{} {}", button_danger(), button_small())
                                    on:click=move |_| on_delete_inner(order_id.clone())
                                >
                                    "Delete"
                                </button>
                            </div>
                        }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}