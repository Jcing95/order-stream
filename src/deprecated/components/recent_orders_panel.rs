use leptos::prelude::*;
use crate::common::types::{Order, OrderItem, Item, OrderStatus};
use crate::frontend::design_system::{
    Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight}
};
use crate::frontend::components::order_card::OrderCard;

#[component]
pub fn RecentOrdersPanel(
    orders: Signal<Option<Result<Vec<Order>, ServerFnError>>>,
    order_items: Signal<Option<Result<Vec<OrderItem>, ServerFnError>>>,
    items: Signal<Option<Result<Vec<Item>, ServerFnError>>>,
    on_status_update: impl Fn(String, OrderStatus) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    view! {
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
                                        on_status_update=on_status_update
                                        show_status_controls=true
                                        allowed_statuses=vec![OrderStatus::Cancelled]
                                    />
                                }
                            }).collect_view().into_any()
                        }
                    }
                    (Some(Err(error)), _, _) => {
                        view! {
                            <Alert intent=Intent::Danger size=Size::Md>
                                "Error loading orders: " {error.to_string()}
                            </Alert>
                        }.into_any()
                    }
                    (_, Some(Err(error)), _) => {
                        view! {
                            <Alert intent=Intent::Danger size=Size::Md>
                                "Error loading order items: " {error.to_string()}
                            </Alert>
                        }.into_any()
                    }
                    (_, _, Some(Err(error))) => {
                        view! {
                            <Alert intent=Intent::Danger size=Size::Md>
                                "Error loading items: " {error.to_string()}
                            </Alert>
                        }.into_any()
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
    }
}