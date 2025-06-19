use leptos::prelude::*;
use crate::common::types::{Order, OrderStatus};

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
            OrderStatus::Draft => "bg-gray-100 text-gray-800",
            OrderStatus::Ordered => "bg-blue-100 text-blue-800", 
            OrderStatus::Ready => "bg-yellow-100 text-yellow-800",
            OrderStatus::Completed => "bg-green-100 text-green-800",
            OrderStatus::Cancelled => "bg-red-100 text-red-800",
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
            <h3 class="text-lg font-semibold">"Orders"</h3>
            
            {move || {
                let orders_list = orders.get();
                if orders_list.is_empty() {
                    view! {
                        <div class="text-gray-500 italic p-4 border rounded-lg">
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
                            <div class="flex items-center justify-between p-4 border rounded-lg bg-gray-50">
                                <div class="flex-1">
                                    <div class="flex items-center gap-3">
                                        <h4 class="font-medium">
                                            "Order #" {move || format!("{:03}", order.sequential_id)}
                                        </h4>
                                        <span class={move || format!("px-2 py-1 text-xs font-medium rounded-full {}", status_class)}>
                                            {move || status_label}
                                        </span>
                                    </div>
                                    <div class="text-sm text-gray-600 mt-1">
                                        <p>"Total: $" {move || format!("{:.2}", order.total_price)}</p>
                                        <p class="text-xs text-gray-500">"ID: " {move || order.id.clone()}</p>
                                    </div>
                                </div>
                                <button
                                    class="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
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