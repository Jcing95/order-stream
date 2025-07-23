use leptos::prelude::*;
use std::collections::HashMap;

use crate::app::states::{product, websocket};
use crate::backend::item::{get_items_by_station, update_item, update_items_by_order};
use crate::backend::station::get_station;
use crate::common::{requests, types, german_names};

#[component]
fn ItemCard(
    item: types::Item, 
    product: types::Product,
    station_output_status: types::OrderStatus,
    on_update: WriteSignal<u32>,
) -> impl IntoView {
    let item_id = item.id.clone();
    let item_id_for_update = item_id.clone();
    
    let update_item_action = Action::new(move |_: &()| {
        let item_id = item_id_for_update.clone();
        let new_status = station_output_status;
        async move {
            let update_request = requests::item::Update {
                product_id: None,
                quantity: None,
                status: Some(new_status),
            };
            let _ = update_item(item_id, update_request).await;
        }
    });

    // Trigger parent refresh when action completes
    Effect::new(move |_| {
        if update_item_action.value().get().is_some() {
            on_update.update(|n| *n += 1);
        }
    });

    view! {
        <div class="p-3 bg-surface-elevated rounded-lg border border-border">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <div class="flex items-center justify-between">
                        <h4 class="text-text font-medium">{item.quantity}{" x "}{product.name}</h4>
                    </div>
                </div>
                
                <div class="ml-4">
                    <button
                        class="bg-primary text-white hover:bg-primary/90 px-3 py-1 rounded text-sm transition-colors"
                        on:click=move |_| {
                            update_item_action.dispatch(());
                        }
                        disabled=move || update_item_action.pending().get()
                    >
                        {move || if update_item_action.pending().get() {
                            "..."
                        } else {
                            "☑"
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn OrderGroup(
    order_id: String,
    items: Vec<types::Item>,
    products: HashMap<String, types::Product>,
    station_id: String,
    station_output_status: types::OrderStatus,
    on_update: WriteSignal<u32>,
) -> impl IntoView {
    let order_id_for_bulk = order_id.clone();
    let items_count = items.len();
    
    let update_order_action = Action::new(move |_: &()| {
        let order_id = order_id_for_bulk.clone();
        let station_id_clone = station_id.clone();
        let new_status = station_output_status;
        async move {
            let _ = update_items_by_order(order_id, station_id_clone, new_status).await;
        }
    });

    // Trigger parent refresh when action completes
    Effect::new(move |_| {
        if update_order_action.value().get().is_some() {
            on_update.update(|n| *n += 1);
        }
    });

    view! {
        <div class="bg-surface rounded-lg border border-border p-4">
            <div class="flex items-center justify-between mb-4">
                <div>
                    <h3 class="text-lg font-semibold text-text" data-order-id=order_id.clone()>{"Bestellung '"}{german_names::generate_german_name(&order_id.clone())}{"'"}</h3>
                    <p class="text-sm text-text-muted">{format!("{} items", items_count)}</p>
                </div>
                <button
                    class="bg-green-600 text-white hover:bg-green-700 px-4 py-2 rounded text-sm transition-colors"
                    on:click=move |_| {
                        update_order_action.dispatch(());
                    }
                    disabled=move || update_order_action.pending().get()
                >
                    {move || if update_order_action.pending().get() {
                        "..."
                    } else {
                        "☑"
                    }}
                </button>
            </div>
            
            <div class="space-y-2">
                <For
                    each=move || items.clone()
                    key=|item| item.id.clone()
                    children=move |item| {
                        let product = products.get(&item.product_id).cloned().unwrap_or_else(|| {
                            types::Product {
                                id: item.product_id.clone(),
                                name: "Unknown Product".to_string(),
                                category_id: "".to_string(),
                                price: item.price,
                                active: false,
                            }
                        });
                        
                        view! {
                            <ItemCard 
                                item=item
                                product=product
                                station_output_status=station_output_status
                                on_update=on_update
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn StationView(station_id: String) -> impl IntoView {
    let (refresh_trigger, set_refresh_trigger) = signal::<u32>(0);
    
    // Listen to WebSocket updates for items and orders to trigger refresh
    let websocket_state = websocket::get();
    Effect::new({
        let set_refresh_trigger = set_refresh_trigger;
        let websocket_state = websocket_state.clone();
        move |_| {
            if let Some(_message) = websocket_state.items.get() {
                // Any item change should refresh all station views
                set_refresh_trigger.update(|n| *n += 1);
                websocket_state.items.set(None);
            }
        }
    });
    
    // Also listen for order changes (new orders create items)
    Effect::new({
        let set_refresh_trigger = set_refresh_trigger;
        let websocket_state = websocket_state.clone();
        move |_| {
            if let Some(_message) = websocket_state.orders.get() {
                // New orders might have items for this station
                set_refresh_trigger.update(|n| *n += 1);
                websocket_state.orders.set(None);
            }
        }
    });
    
    let station_id_mv = station_id.clone();
    // Resource to fetch station details
    let station_resource = Resource::new(
        move || station_id_mv.clone(),
        |id| async move {
            get_station(id).await.ok()
        },
    );
    let station_id_mv = station_id.clone();

    // Resource to fetch items for this station
    let items_resource = Resource::new(
        move || (station_id_mv.clone(), refresh_trigger.get()),
        |(id, _)| async move {
            get_items_by_station(id).await.unwrap_or_default()
        },
    );
    
    // Resource to fetch all products (to display item details)
    let product_state = product::get();
    let products = product_state.get_products();
    
    view! {
        <div class="space-y-6">
            <Suspense fallback=move || view! {
                <div class="flex items-center justify-center py-12">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
                </div>
            }>
                {move || {
                    let station_opt = station_resource.get();
                    let items = items_resource.get().unwrap_or_default();
                    let products_list = products.get();
                    
                    if let Some(Some(station)) = station_opt {
                        // Create a product lookup map
                        let product_map: HashMap<String, types::Product> = products_list
                            .into_iter()
                            .map(|p| (p.id.clone(), p))
                            .collect();
                        
                        // Group items by order_id
                        let mut orders: HashMap<String, Vec<types::Item>> = HashMap::new();
                        for item in items {
                            if let Some(order_id) = &item.order_id {
                                orders.entry(order_id.clone()).or_insert_with(Vec::new).push(item);
                            }
                        }
                        
                        if orders.is_empty() {
                            view! {
                                <div class="text-center py-12">
                                    <svg class="mx-auto h-12 w-12 text-text-muted mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path>
                                    </svg>
                                    <p class="text-text-muted text-lg">"No items for this station"</p>
                                    <p class="text-text-muted text-sm mt-2">"Items will appear here when they match this station's criteria"</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">    
                                    <For
                                        each=move || orders.clone()
                                        key=|(order_id, _)| order_id.clone()
                                        children=move |(order_id, order_items)| {
                                            view! {
                                                <OrderGroup 
                                                    order_id=order_id
                                                    items=order_items
                                                    products=product_map.clone()
                                                    station_id=station.id.clone()
                                                    station_output_status=station.output_status
                                                    on_update=set_refresh_trigger
                                                />
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    } else {
                        view! {
                            <div class="text-center py-12">
                                <p class="text-text-muted">"Loading station details..."</p>
                            </div>
                        }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}