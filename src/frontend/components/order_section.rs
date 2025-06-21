use leptos::prelude::*;
use crate::common::types::Order;
use crate::frontend::components::order_list::OrderList;
use crate::frontend::design_system::{
    Button, Text,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn OrderSection<F1, F2>(
    orders: ReadSignal<Vec<Order>>,
    on_create: F1,
    on_delete: F2,
) -> impl IntoView
where
    F1: Fn(()) + 'static + Clone + Send + Sync,
    F2: Fn(String) + 'static + Clone + Send + Sync,
{
    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Xl 
                    weight=FontWeight::Semibold
                >
                    "Order Management"
                </Text>
                <Button
                    size=Size::Md
                    intent=Intent::Primary
                    on_click=Callback::new(move |_| on_create(()))
                >
                    "Create New Order"
                </Button>
            </div>
            <OrderList orders=orders on_delete=on_delete />
        </div>
    }
}