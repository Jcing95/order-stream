use leptos::prelude::*;
use crate::common::types::{Order, OrderStatus, OrderItem, Item, Category};
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn FilteredOrderCard(
    order: Order,
    order_items: Signal<Vec<OrderItem>>,
    items: Signal<Vec<Item>>,
    categories: Signal<Vec<Category>>,
    
    // Station filtering
    filter_category_ids: Vec<String>,
    filter_statuses: Vec<OrderStatus>,
    
    // Actions
    on_item_update: Callback<(String, OrderStatus)>,
    
    #[prop(optional)] 
    show_all_context: bool,  // Show full order context even if some items filtered out
    
    #[prop(optional)]
    expandable: bool,  // Allow expanding to see full order
) -> impl IntoView {
    let expanded = RwSignal::new(false);
    
    // Optimized toggle handler
    let toggle_expanded = Callback::new(move |_: leptos::ev::MouseEvent| {
        expanded.update(|e| *e = !*e);
    });
    
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

    let get_item_details = move |item_id: &str| -> Option<Item> {
        items.get().into_iter().find(|item| item.id == item_id)
    };
    
    let get_category_name = move |category_id: &str| -> String {
        categories.get()
            .into_iter()
            .find(|cat| cat.id == category_id)
            .map(|cat| cat.name)
            .unwrap_or_else(|| "Unknown".to_string())
    };

    // Get filtered order items that match station criteria
    let order_id = order.id.clone();
    let filtered_items = Signal::derive(move || {
        order_items.get()
            .into_iter()
            .filter(|oi| oi.order_id == order_id)
            .filter(|oi| {
                // Filter by status
                if !filter_statuses.contains(&oi.status) {
                    return false;
                }
                
                // Filter by category if specified
                if !filter_category_ids.is_empty() {
                    if let Some(item) = get_item_details(&oi.item_id) {
                        return filter_category_ids.contains(&item.category_id);
                    }
                    return false;
                }
                
                true
            })
            .collect::<Vec<_>>()
    });
    
    // Get all order items for context view
    let order_id_2 = order.id.clone();
    let all_items = Signal::derive(move || {
        order_items.get()
            .into_iter()
            .filter(|oi| oi.order_id == order_id_2)
            .collect::<Vec<_>>()
    });

    view! {
        <Show when=move || !filtered_items.get().is_empty()>
            {move || {
                let filtered = filtered_items.get();
                
                view! {
                    <Card variant=CardVariant::Default>
                        <div class="space-y-4">
                            // Order Header with expand button if applicable
                            <div class="flex items-start justify-between">
                                <div class="space-y-2 flex-1">
                                    <div class="flex items-center gap-3">
                                        <Text 
                                            variant=TextVariant::Heading 
                                            size=Size::Lg 
                                            weight=FontWeight::Semibold
                                        >
                                            "Order #" {format!("{:03}", order.sequential_id)}
                                        </Text>
                                        
                                        {
                                            if show_all_context {
                                                let all_count = all_items.get().len();
                                                let filtered_count = filtered.len();
                                                
                                                if all_count > filtered_count {
                                                    view! {
                                                        <Text 
                                                            variant=TextVariant::Caption 
                                                            size=Size::Xs 
                                                            weight=FontWeight::Medium
                                                            class="px-2 py-1 rounded bg-gray-100 dark:bg-gray-800"
                                                        >
                                                            {filtered_count.to_string()} "/" {all_count.to_string()} " items"
                                                        </Text>
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }
                                    </div>
                                    
                                    <Text 
                                        variant=TextVariant::Body 
                                        size=Size::Md 
                                        weight=FontWeight::Medium
                                    >
                                        "Total: $" {format!("{:.2}", order.total_price)}
                                    </Text>
                                </div>

                                // Expand button for context
                                {
                                    if expandable && show_all_context {
                                        view! {
                                            <Button
                                                size=Size::Sm
                                                intent=Intent::Secondary
                                                on_click=toggle_expanded
                                            >
                                                {move || if expanded.get() { "Collapse" } else { "View All" }}
                                            </Button>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                            </div>

                            // Filtered Items (always shown)
                            <div class="space-y-2">
                                {filtered.into_iter().map(|order_item| {
                                    let item_details = get_item_details(&order_item.item_id);
                                    let item_name = item_details.as_ref().map(|i| i.name.clone()).unwrap_or_else(|| format!("Item {}", order_item.item_id));
                                    let category_name = item_details.as_ref().map(|i| get_category_name(&i.category_id)).unwrap_or_default();
                                    let item_id = order_item.id.clone();
                                    let current_status = order_item.status;
                                    
                                    view! {
                                        <div class="flex justify-between items-center py-3 border border-gray-200 dark:border-gray-700 rounded-lg px-4 bg-gray-50 dark:bg-gray-800">
                                            <div class="space-y-1 flex-1">
                                                <div class="flex items-center gap-2">
                                                    <Text 
                                                        variant=TextVariant::Body 
                                                        size=Size::Sm 
                                                        weight=FontWeight::Medium
                                                    >
                                                        {item_name.clone()}
                                                    </Text>
                                                    
                                                    <Text 
                                                        variant=TextVariant::Caption 
                                                        size=Size::Xs 
                                                        weight=FontWeight::Medium
                                                        intent=status_intent(&current_status)
                                                        class="px-2 py-1 rounded-full bg-opacity-20"
                                                    >
                                                        {status_text(&current_status)}
                                                    </Text>
                                                </div>
                                                
                                                <div class="flex items-center gap-4">
                                                    <Text 
                                                        variant=TextVariant::Caption 
                                                        size=Size::Xs 
                                                        weight=FontWeight::Normal
                                                    >
                                                        "Qty: " {order_item.quantity.to_string()}
                                                    </Text>
                                                    
                                                    {if !category_name.is_empty() {
                                                        view! {
                                                            <Text 
                                                                variant=TextVariant::Caption 
                                                                size=Size::Xs 
                                                                weight=FontWeight::Normal
                                                                class="text-gray-500"
                                                            >
                                                                {category_name.clone()}
                                                            </Text>
                                                        }.into_any()
                                                    } else {
                                                        view! {}.into_any()
                                                    }}
                                                </div>
                                            </div>
                                            
                                            // Quick action buttons
                                            <div class="flex gap-2 ml-4">
                                                {
                                                    let next_status = match current_status {
                                                        OrderStatus::Ordered => Some(OrderStatus::Ready),
                                                        OrderStatus::Ready => Some(OrderStatus::Completed),
                                                        _ => None,
                                                    };
                                                    
                                                    if let Some(next) = next_status {
                                                        let item_id_clone = item_id.clone();
                                                        
                                                        view! {
                                                            <Button
                                                                size=Size::Sm
                                                                intent=match next {
                                                                    OrderStatus::Ready => Intent::Warning,
                                                                    OrderStatus::Completed => Intent::Success,
                                                                    _ => Intent::Primary,
                                                                }
                                                                on_click=Callback::new({
                                                                    let on_item_update = on_item_update;
                                                                    move |_| {
                                                                        on_item_update.run((item_id_clone.clone(), next));
                                                                    }
                                                                })
                                                            >
                                                                {match next {
                                                                    OrderStatus::Ready => "Ready",
                                                                    OrderStatus::Completed => "Done",
                                                                    _ => "Next",
                                                                }}
                                                            </Button>
                                                        }.into_any()
                                                    } else {
                                                        view! {}.into_any()
                                                    }
                                                }
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                            
                            // Expanded context view (all items)
                            {move || {
                                if expandable && expanded.get() && show_all_context {
                                    let all = all_items.get();
                                    
                                    view! {
                                        <div class="border-t pt-4 space-y-2">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Medium
                                                class="text-gray-600 dark:text-gray-400"
                                            >
                                                "Full Order Context:"
                                            </Text>
                                            
                                            {all.into_iter().map(|order_item| {
                                                let item_details = get_item_details(&order_item.item_id);
                                                let item_name = item_details.as_ref().map(|i| i.name.clone()).unwrap_or_else(|| format!("Item {}", order_item.item_id));
                                                let category_name = item_details.as_ref().map(|i| get_category_name(&i.category_id)).unwrap_or_default();
                                                
                                                view! {
                                                    <div class="flex justify-between items-center py-2 px-3 rounded bg-gray-100 dark:bg-gray-900">
                                                        <div class="space-y-1">
                                                            <Text 
                                                                variant=TextVariant::Body 
                                                                size=Size::Xs 
                                                                weight=FontWeight::Medium
                                                            >
                                                                {item_name.clone()}
                                                            </Text>
                                                            <div class="flex items-center gap-3">
                                                                <Text 
                                                                    variant=TextVariant::Caption 
                                                                    size=Size::Xs 
                                                                    weight=FontWeight::Normal
                                                                >
                                                                    "Qty: " {order_item.quantity.to_string()}
                                                                </Text>
                                                                
                                                                <Text 
                                                                    variant=TextVariant::Caption 
                                                                    size=Size::Xs 
                                                                    weight=FontWeight::Medium
                                                                    intent=status_intent(&order_item.status)
                                                                >
                                                                    {status_text(&order_item.status)}
                                                                </Text>
                                                                
                                                                {if !category_name.is_empty() {
                                                                    view! {
                                                                        <Text 
                                                                            variant=TextVariant::Caption 
                                                                            size=Size::Xs 
                                                                            weight=FontWeight::Normal
                                                                            class="text-gray-500"
                                                                        >
                                                                            {category_name.clone()}
                                                                        </Text>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {}.into_any()
                                                                }}
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }}
                        </div>
                    </Card>
                }
            }}
        </Show>
    }
}