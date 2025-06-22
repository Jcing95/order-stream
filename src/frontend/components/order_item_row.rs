use leptos::prelude::*;
use crate::common::types::{OrderItem, OrderStatus, Item};
use crate::frontend::design_system::{
    Button, Text,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn OrderItemRow(
    order_item: OrderItem,
    item_details: Option<Item>,
    category_name: Option<String>,
    
    // Actions
    on_status_update: Callback<(String, OrderStatus)>,
    
    #[prop(optional)]
    show_order_context: bool,
    
    #[prop(optional)]
    order_sequential_id: u32,
    
    #[prop(optional)]
    compact: bool,  // More compact display for mobile
) -> impl IntoView {
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
    
    let get_next_status = |current: &OrderStatus| -> Option<OrderStatus> {
        match current {
            OrderStatus::Ordered => Some(OrderStatus::Ready),
            OrderStatus::Ready => Some(OrderStatus::Completed),
            _ => None,
        }
    };
    
    let get_action_text = |next: &OrderStatus| -> &'static str {
        match next {
            OrderStatus::Ready => "Ready",
            OrderStatus::Completed => "Done",
            _ => "Next",
        }
    };

    let item_name = item_details
        .as_ref()
        .map(|item| item.name.clone())
        .unwrap_or_else(|| format!("Item {}", order_item.item_id));

    view! {
        <div class=move || if compact {
            "flex items-center justify-between py-3 px-4 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 shadow-sm"
        } else {
            "flex items-start justify-between p-4 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 shadow-sm space-y-2"
        }>
            <div class="flex-1 min-w-0">
                // Order context (if shown)
                {
                    if show_order_context && order_sequential_id > 0 {
                        view! {
                            <Text 
                                variant=TextVariant::Caption 
                                size=Size::Xs 
                                weight=FontWeight::Medium
                                class="text-gray-500 mb-1"
                            >
                                "Order #" {format!("{:03}", order_sequential_id)}
                            </Text>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
                
                // Item name and status
                <div class=move || if compact {
                    "flex items-center gap-2 mb-1"
                } else {
                    "flex items-start gap-2 mb-2"
                }>
                    <Text 
                        variant=TextVariant::Body 
                        size=if compact { Size::Sm } else { Size::Md }
                        weight=FontWeight::Medium
                        class="truncate flex-1"
                    >
                        {item_name}
                    </Text>
                    
                    <Text 
                        variant=TextVariant::Caption 
                        size=Size::Xs 
                        weight=FontWeight::Medium
                        intent=status_intent(&order_item.status)
                        class="px-2 py-1 rounded-full bg-opacity-20 whitespace-nowrap"
                    >
                        {status_text(&order_item.status)}
                    </Text>
                </div>
                
                // Quantity and category
                <div class="flex items-center gap-4">
                    <Text 
                        variant=TextVariant::Caption 
                        size=Size::Xs 
                        weight=FontWeight::Normal
                        class="text-gray-600 dark:text-gray-400"
                    >
                        "Qty: " {order_item.quantity.to_string()}
                    </Text>
                    
                    {
                        if let Some(cat_name) = category_name {
                            view! {
                                <Text 
                                    variant=TextVariant::Caption 
                                    size=Size::Xs 
                                    weight=FontWeight::Normal
                                    class="text-gray-500 dark:text-gray-500"
                                >
                                    {cat_name}
                                </Text>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                </div>
            </div>
            
            // Action buttons - mobile-optimized
            <div class=move || if compact {
                "flex gap-2 ml-3"
            } else {
                "flex flex-col gap-2 ml-4"
            }>
                {
                    let next_status = get_next_status(&order_item.status);
                    
                    if let Some(next) = next_status {
                        let item_id = order_item.id.clone();
                        let on_update = on_status_update.clone();
                        
                        view! {
                            <Button
                                size=if compact { Size::Sm } else { Size::Md }
                                intent=match next {
                                    OrderStatus::Ready => Intent::Warning,
                                    OrderStatus::Completed => Intent::Success,
                                    _ => Intent::Primary,
                                }
                                on_click=Callback::new(move |_| {
                                    on_update.run((item_id.clone(), next));
                                })
                            >
                                {get_action_text(&next)}
                            </Button>
                        }.into_any()
                    } else {
                        view! {
                            <div class=move || if compact {
                                "w-16 flex items-center justify-center"
                            } else {
                                "w-20 flex items-center justify-center"
                            }>
                                <Text 
                                    variant=TextVariant::Caption 
                                    size=Size::Xs 
                                    weight=FontWeight::Medium
                                    class="text-gray-400"
                                >
                                    "✓"
                                </Text>
                            </div>
                        }.into_any()
                    }
                }
                
                // Bulk action hint for swipe (mobile)
                {
                    if compact && get_next_status(&order_item.status).is_some() {
                        view! {
                            <div class="hidden touch:block">
                                <Text 
                                    variant=TextVariant::Caption 
                                    size=Size::Xs 
                                    weight=FontWeight::Normal
                                    class="text-gray-400 ml-2"
                                >
                                    "→"
                                </Text>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            </div>
        </div>
    }
}