use leptos::prelude::*;
use crate::common::types::Order;
use crate::frontend::components::order_list::OrderList;
use crate::frontend::state::theme::{text_primary, button_primary};

#[component]
pub fn OrderSection<F1, F2>(
    orders: ReadSignal<Vec<Order>>,
    on_create: F1,
    on_delete: F2,
) -> impl IntoView
where
    F1: Fn(()) + 'static + Clone,
    F2: Fn(String) + 'static + Clone + Send,
{
    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h2 class=format!("text-xl font-semibold {}", text_primary())>"Order Management"</h2>
                <button
                    class=button_primary()
                    on:click=move |_| on_create(())
                >
                    "Create New Order"
                </button>
            </div>
            <OrderList orders=orders on_delete=on_delete />
        </div>
    }
}