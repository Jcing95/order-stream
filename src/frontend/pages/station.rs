use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{Order, OrderStatus};
use crate::frontend::components::order_card::OrderCard;
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};
use crate::backend::services::{
    orders::{get_orders, update_order_status},
    order_items::get_all_order_items,
    items::get_items,
    categories::get_categories,
};

#[derive(Clone, PartialEq, Default)]
pub enum StationType {
    #[default]
    Bar,
    Kitchen,
    Drinks,
    Food,
    All,
}

impl StationType {
    pub fn display_name(&self) -> &'static str {
        match self {
            StationType::Bar => "Bar Station",
            StationType::Kitchen => "Kitchen Station", 
            StationType::Drinks => "Drinks Station",
            StationType::Food => "Food Station",
            StationType::All => "All Items Station",
        }
    }

    pub fn category_filter(&self) -> Option<Vec<String>> {
        match self {
            StationType::Bar => Some(vec!["Alcoholic Drinks".to_string(), "Cocktails".to_string()]),
            StationType::Kitchen => Some(vec!["Hot Food".to_string(), "Prepared Food".to_string()]),
            StationType::Drinks => Some(vec!["Beverages".to_string(), "Soft Drinks".to_string(), "Coffee".to_string()]),
            StationType::Food => Some(vec!["Snacks".to_string(), "Cold Food".to_string()]),
            StationType::All => None,
        }
    }
}

#[component]
pub fn StationPage(
    #[prop(optional)] station_type: StationType,
) -> impl IntoView {
    let station_type = StoredValue::new(station_type);
    
    // Data signals
    let orders = Resource::new(|| (), |_| get_orders());
    let order_items = Resource::new(|| (), |_| get_all_order_items());
    let items = Resource::new(|| (), |_| get_items());
    let categories = Resource::new(|| (), |_| get_categories());
    
    // UI state
    let selected_category_filter = RwSignal::new(String::new());
    let error_message = RwSignal::new(None::<String>);
    let status_filter = RwSignal::new(vec![OrderStatus::Ordered, OrderStatus::Ready]);

    // Update order status
    let update_status = move |order_id: String, new_status: OrderStatus| {
        spawn_local(async move {
            match update_order_status(order_id, new_status).await {
                Ok(_) => {
                    orders.refetch();
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to update order: {}", e)));
                }
            }
        });
    };

    // Filter orders based on station type and status
    let filtered_orders = Signal::derive(move || {
        let station = station_type.with_value(|s| s.clone());
        match (orders.get(), order_items.get(), items.get(), categories.get()) {
            (Some(Ok(orders_list)), Some(Ok(order_items_list)), Some(Ok(items_list)), Some(Ok(categories_list))) => {
                let status_filters = status_filter.get();
                let category_filter = selected_category_filter.get();
                
                // Filter orders by status
                let mut filtered: Vec<Order> = orders_list
                    .into_iter()
                    .filter(|order| status_filters.contains(&order.status))
                    .collect();

                // Filter by station type if not All
                if let Some(station_categories) = station.category_filter() {
                    filtered.retain(|order| {
                        // Check if this order has any items that belong to this station's categories
                        let order_has_station_items = order_items_list
                            .iter()
                            .filter(|oi| oi.order_id == order.id)
                            .any(|oi| {
                                if let Some(item) = items_list.iter().find(|i| i.id == oi.item_id) {
                                    if let Some(category) = categories_list.iter().find(|c| c.id == item.category_id) {
                                        station_categories.contains(&category.name)
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            });
                        order_has_station_items
                    });
                }

                // Filter by specific category if selected
                if !category_filter.is_empty() {
                    filtered.retain(|order| {
                        order_items_list
                            .iter()
                            .filter(|oi| oi.order_id == order.id)
                            .any(|oi| {
                                if let Some(item) = items_list.iter().find(|i| i.id == oi.item_id) {
                                    item.category_id == category_filter
                                } else {
                                    false
                                }
                            })
                    });
                }

                // Sort by order priority: Ordered first, then Ready
                filtered.sort_by(|a, b| {
                    match (&a.status, &b.status) {
                        (OrderStatus::Ordered, OrderStatus::Ready) => std::cmp::Ordering::Less,
                        (OrderStatus::Ready, OrderStatus::Ordered) => std::cmp::Ordering::Greater,
                        _ => a.sequential_id.cmp(&b.sequential_id),
                    }
                });

                filtered
            }
            _ => Vec::new()
        }
    });

    // Get available categories for this station
    let available_categories = Signal::derive(move || {
        let station = station_type.with_value(|s| s.clone());
        if let Some(Ok(categories_list)) = categories.get() {
            if let Some(station_categories) = station.category_filter() {
                categories_list
                    .into_iter()
                    .filter(|cat| station_categories.contains(&cat.name))
                    .collect::<Vec<_>>()
            } else {
                categories_list
            }
        } else {
            Vec::new()
        }
    });

    view! {
        <div class="container mx-auto p-6 space-y-6">
            <div class="flex justify-between items-center">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Xl 
                    weight=FontWeight::Bold
                >
                    {station_type.with_value(|s| s.display_name())}
                </Text>
                
                // Auto-refresh indicator
                <div class="flex items-center gap-2">
                    <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                    <Text variant=TextVariant::Caption size=Size::Sm>
                        "Live updates"
                    </Text>
                </div>
            </div>

            // Error display
            {move || {
                error_message.get().map(|msg| {
                    view! {
                        <Alert intent=Intent::Danger size=Size::Md>
                            {msg}
                        </Alert>
                    }
                })
            }}

            // Filters
            <Card variant=CardVariant::Default>
                <div class="space-y-4">
                    <Text 
                        variant=TextVariant::Body 
                        size=Size::Md 
                        weight=FontWeight::Medium
                    >
                        "Filters"
                    </Text>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        // Status filter
                        <div class="space-y-2">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                "Show Orders"
                            </Text>
                            <div class="space-y-2">
                                <label class="flex items-center gap-2">
                                    <input 
                                        type="checkbox" 
                                        checked=move || status_filter.get().contains(&OrderStatus::Ordered)
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            let mut filters = status_filter.get();
                                            if checked {
                                                if !filters.contains(&OrderStatus::Ordered) {
                                                    filters.push(OrderStatus::Ordered);
                                                }
                                            } else {
                                                filters.retain(|s| *s != OrderStatus::Ordered);
                                            }
                                            status_filter.set(filters);
                                        }
                                    />
                                    <Text variant=TextVariant::Body size=Size::Sm>
                                        "New Orders"
                                    </Text>
                                </label>
                                <label class="flex items-center gap-2">
                                    <input 
                                        type="checkbox" 
                                        checked=move || status_filter.get().contains(&OrderStatus::Ready)
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            let mut filters = status_filter.get();
                                            if checked {
                                                if !filters.contains(&OrderStatus::Ready) {
                                                    filters.push(OrderStatus::Ready);
                                                }
                                            } else {
                                                filters.retain(|s| *s != OrderStatus::Ready);
                                            }
                                            status_filter.set(filters);
                                        }
                                    />
                                    <Text variant=TextVariant::Body size=Size::Sm>
                                        "Ready Orders"
                                    </Text>
                                </label>
                            </div>
                        </div>

                        // Category filter (if applicable)
                        {move || {
                            let categories = available_categories.get();
                            if categories.len() > 1 {
                                view! {
                                    <div class="space-y-2">
                                        <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                            "Category"
                                        </Text>
                                        <select 
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                            on:change=move |ev| selected_category_filter.set(event_target_value(&ev))
                                        >
                                            <option value="">"All Categories"</option>
                                            {categories.into_iter().map(|category| {
                                                view! {
                                                    <option value=&category.id>
                                                        {category.name}
                                                    </option>
                                                }
                                            }).collect_view()}
                                        </select>
                                    </div>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }
                        }}

                        // Quick refresh button
                        <div class="flex items-end">
                            <Button
                                size=Size::Md
                                intent=Intent::Secondary
                                on_click=Callback::new(move |_| {
                                    orders.refetch();
                                    order_items.refetch();
                                })
                            >
                                "Refresh"
                            </Button>
                        </div>
                    </div>
                </div>
            </Card>

            // Orders display
            <div class="space-y-4">
                {move || {
                    let filtered = filtered_orders.get();
                    if filtered.is_empty() {
                        view! {
                            <Alert intent=Intent::Info size=Size::Lg>
                                "No orders to display. Check your filters or wait for new orders."
                            </Alert>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-2">
                                <Text 
                                    variant=TextVariant::Body 
                                    size=Size::Md 
                                    weight=FontWeight::Medium
                                >
                                    {format!("{} orders", filtered.len())}
                                </Text>
                                
                                <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
                                    {filtered.into_iter().map(|order| {
                                        // Determine allowed status transitions for this station
                                        let allowed_statuses = match order.status {
                                            OrderStatus::Ordered => vec![OrderStatus::Ready, OrderStatus::Cancelled],
                                            OrderStatus::Ready => vec![OrderStatus::Completed, OrderStatus::Cancelled],
                                            _ => vec![],
                                        };
                                        
                                        view! {
                                            <OrderCard
                                                order=order
                                                order_items=Signal::derive(move || {
                                                    order_items.get().unwrap_or_else(|| Ok(Vec::new())).unwrap_or_default()
                                                })
                                                items=Signal::derive(move || {
                                                    items.get().unwrap_or_else(|| Ok(Vec::new())).unwrap_or_default()
                                                })
                                                on_status_update=update_status
                                                show_status_controls=true
                                                allowed_statuses=allowed_statuses
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}