use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{Station, OrderStatus, UpdateOrderItemRequest};
use crate::backend::services::{
    orders::get_orders,
    order_items::{get_all_order_items, update_order_item},
    items::get_items,
    categories::get_categories,
};
use crate::frontend::components::{
    filtered_order_card::FilteredOrderCard, 
    order_item_row::OrderItemRow,
    loading::LoadingSpinner
};
use crate::frontend::design_system::{
    Card, CardVariant, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn StationView(
    station: Station,
    
    #[prop(default = StationViewMode::OrderCards)]
    view_mode: StationViewMode,
) -> impl IntoView {
    
    // Data signals
    let orders = Resource::new(|| (), |_| get_orders());
    let order_items = Resource::new(|| (), |_| get_all_order_items());
    let items = Resource::new(|| (), |_| get_items());
    let categories = Resource::new(|| (), |_| get_categories());
    
    // Clone station data for use in closures
    let station_category_ids = station.category_ids.clone();
    let station_category_ids_2 = station.category_ids.clone();
    let station_input_statuses_2 = station.input_statuses.clone();
    let station_input_statuses_3 = station.input_statuses.clone();
    
    // Handle order item status updates using station's configured output status
    let station_output_status = station.output_status;
    let on_item_update = {
        let order_items = order_items;
        let orders = orders;
        
        Callback::new(move |(item_id, _new_status): (String, OrderStatus)| {
            let order_items = order_items;
            let orders = orders;
            
            spawn_local(async move {
                // Always use station's configured output status, ignoring the passed status
                let update_req = UpdateOrderItemRequest {
                    item_id: None,
                    quantity: None,
                    status: Some(station_output_status),
                };
                
                match update_order_item(item_id, update_req).await {
                    Ok(_) => {
                        // Refresh data
                        order_items.refetch();
                        orders.refetch();
                    }
                    Err(e) => {
                        leptos::logging::error!("Failed to update item status: {}", e);
                    }
                }
            });
        })
    };

    view! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
            <div class="max-w-7xl mx-auto p-4 space-y-6">
                // Header
                <div class="flex items-center justify-between">
                    <div class="space-y-1">
                        <Text 
                            variant=TextVariant::Heading 
                            size=Size::Xl 
                            weight=FontWeight::Bold
                        >
                            {station.name.clone()}
                        </Text>
                        <div class="mt-2 flex flex-wrap gap-2 text-sm">
                            <Text 
                                variant=TextVariant::Caption 
                                size=Size::Xs 
                                weight=FontWeight::Medium
                                class="px-2 py-1 rounded bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200"
                            >
                                {format!("Shows: {}", station.input_statuses.iter()
                                    .map(|s| match s {
                                        OrderStatus::Draft => "Draft",
                                        OrderStatus::Ordered => "Ordered", 
                                        OrderStatus::Ready => "Ready",
                                        OrderStatus::Completed => "Completed",
                                        OrderStatus::Cancelled => "Cancelled",
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ")
                                )}
                            </Text>
                            <Text 
                                variant=TextVariant::Caption 
                                size=Size::Xs 
                                weight=FontWeight::Medium
                                class="px-2 py-1 rounded bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200"
                            >
                                {format!("Updates to: {}", match station.output_status {
                                    OrderStatus::Draft => "Draft",
                                    OrderStatus::Ordered => "Ordered",
                                    OrderStatus::Ready => "Ready", 
                                    OrderStatus::Completed => "Completed",
                                    OrderStatus::Cancelled => "Cancelled",
                                })}
                            </Text>
                        </div>
                    </div>
                </div>
                
                // Main content based on view mode
                {match view_mode {
                    StationViewMode::OrderCards => view! {
                        <div class="space-y-4">
                            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                                {move || {
                                    let orders_data = orders.get();
                                    let items_data = items.get();
                                    let order_items_data = order_items.get();
                                    let categories_data = categories.get();
                                    
                                    match (orders_data, items_data, order_items_data, categories_data) {
                                        (Some(Ok(orders_vec)), Some(Ok(items_vec)), Some(Ok(order_items_vec)), Some(Ok(categories_vec))) => {
                                            let filtered_orders = orders_vec.into_iter()
                                                .filter(|order| {
                                                    // Only show orders that have items matching our station criteria
                                                    order_items_vec.iter().any(|oi| {
                                                        oi.order_id == order.id &&
                                                        station_input_statuses_2.contains(&oi.status) &&
                                                        (station_category_ids.is_empty() || {
                                                            items_vec.iter()
                                                                .find(|item| item.id == oi.item_id)
                                                                .map(|item| station_category_ids.contains(&item.category_id))
                                                                .unwrap_or(false)
                                                        })
                                                    })
                                                })
                                                .collect::<Vec<_>>();
                                            
                                            if filtered_orders.is_empty() {
                                                view! {
                                                    <Card variant=CardVariant::Default>
                                                        <div class="text-center py-8">
                                                            <Text 
                                                                variant=TextVariant::Body 
                                                                size=Size::Lg 
                                                                weight=FontWeight::Medium
                                                                class="text-gray-500"
                                                            >
                                                                "No orders found"
                                                            </Text>
                                                            <Text 
                                                                variant=TextVariant::Body 
                                                                size=Size::Sm 
                                                                weight=FontWeight::Normal
                                                                class="text-gray-400 mt-2"
                                                            >
                                                                "No orders match this station's configuration"
                                                            </Text>
                                                        </div>
                                                    </Card>
                                                }.into_any()
                                            } else {
                                                filtered_orders.into_iter().map(|order| {
                                                    let order_items_clone = order_items_vec.clone();
                                                    let items_clone = items_vec.clone();
                                                    let categories_clone = categories_vec.clone();
                                                    
                                                    view! {
                                                        <FilteredOrderCard
                                                            order=order
                                                            order_items=Signal::derive(move || order_items_clone.clone())
                                                            items=Signal::derive(move || items_clone.clone())
                                                            categories=Signal::derive(move || categories_clone.clone())
                                                            filter_category_ids=station_category_ids_2.clone()
                                                            filter_statuses=station_input_statuses_2.clone()
                                                            on_item_update=on_item_update
                                                            show_all_context=true
                                                            expandable=true
                                                        />
                                                    }
                                                }).collect_view().into_any()
                                            }
                                        }
                                        _ => view! {
                                            <Alert intent=Intent::Danger size=Size::Md>
                                                "Failed to load data"
                                            </Alert>
                                        }.into_any()
                                    }
                                }}
                            </Suspense>
                        </div>
                    }.into_any(),
                    
                    StationViewMode::ItemList => view! {
                        <div class="space-y-2">
                            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                                {move || {
                                    let orders_data = orders.get();
                                    let items_data = items.get();
                                    let order_items_data = order_items.get();
                                    let categories_data = categories.get();
                                    
                                    match (orders_data, items_data, order_items_data, categories_data) {
                                        (Some(Ok(orders_vec)), Some(Ok(items_vec)), Some(Ok(order_items_vec)), Some(Ok(categories_vec))) => {
                                            let filtered_items = order_items_vec.into_iter()
                                                .filter(|oi| {
                                                    // Filter by status
                                                    if !station_input_statuses_3.contains(&oi.status) {
                                                        return false;
                                                    }
                                                    
                                                    // Filter by category if specified
                                                    if !station_category_ids.is_empty() {
                                                        if let Some(item) = items_vec.iter().find(|item| item.id == oi.item_id) {
                                                            return station_category_ids.contains(&item.category_id);
                                                        }
                                                        return false;
                                                    }
                                                    
                                                    true
                                                })
                                                .collect::<Vec<_>>();
                                                
                                            if filtered_items.is_empty() {
                                                view! {
                                                    <Card variant=CardVariant::Default>
                                                        <div class="text-center py-8">
                                                            <Text 
                                                                variant=TextVariant::Body 
                                                                size=Size::Lg 
                                                                weight=FontWeight::Medium
                                                                class="text-gray-500"
                                                            >
                                                                "No items found"
                                                            </Text>
                                                        </div>
                                                    </Card>
                                                }.into_any()
                                            } else {
                                                filtered_items.into_iter().map(|order_item| {
                                                    let item_details = items_vec.iter()
                                                        .find(|item| item.id == order_item.item_id)
                                                        .cloned();
                                                    
                                                    let category_name = item_details.as_ref()
                                                        .and_then(|item| {
                                                            categories_vec.iter()
                                                                .find(|cat| cat.id == item.category_id)
                                                                .map(|cat| cat.name.clone())
                                                        });
                                                        
                                                    let order_seq_id = orders_vec.iter()
                                                        .find(|order| order.id == order_item.order_id)
                                                        .map(|order| order.sequential_id);
                                                    
                                                    view! {
                                                        <OrderItemRow
                                                            order_item=order_item
                                                            item_details=item_details
                                                            category_name=category_name
                                                            on_status_update=on_item_update
                                                            show_order_context=true
                                                            order_sequential_id=order_seq_id.unwrap_or(0)
                                                            compact=true
                                                        />
                                                    }
                                                }).collect_view().into_any()
                                            }
                                        }
                                        _ => view! {
                                            <Alert intent=Intent::Danger size=Size::Md>
                                                "Failed to load data"
                                            </Alert>
                                        }.into_any()
                                    }
                                }}
                            </Suspense>
                        </div>
                    }.into_any(),
                }}
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StationViewMode {
    #[default]
    OrderCards,  // Group by orders (default)
    ItemList,    // Flat list of items
}