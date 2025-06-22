use leptos::prelude::*;
use crate::common::types::Order;
use crate::frontend::design_system::{
    atoms::{FontWeight, TextVariant},
    theme::{Intent, Size},
    Text, Alert,
};

#[component]
pub fn CashierHeader(
    current_order: ReadSignal<Option<Order>>,
    is_creating_order: ReadSignal<bool>,
    pending_item: ReadSignal<Option<(String, u32)>>,
    error_message: ReadSignal<Option<String>>,
    total: f64,
    on_create_order: Callback<leptos::ev::MouseEvent>,
) -> impl IntoView {
    view! {
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
                                <Text variant=TextVariant::Body size=Size::Lg weight=FontWeight::Semibold>
                                    "Total: $" {format!("{:.2}", total)}
                                </Text>
                            </div>
                        }.into_any()
                    } else if is_creating_order.get() || pending_item.get().is_some() {
                        view! {
                            <div class="text-right">
                                <Text variant=TextVariant::Body size=Size::Md intent=Intent::Secondary>
                                    "Creating order..."
                                </Text>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-right">
                                <Text variant=TextVariant::Body size=Size::Md intent=Intent::Secondary>
                                    "No active order"
                                </Text>
                            </div>
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
    }
}