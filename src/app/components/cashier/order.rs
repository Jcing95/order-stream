use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::app::{
    components::atoms::icons,
    states::{order::{self, OrderItem}, settings, event},
};
use crate::backend::order::create_order;
use crate::common::{requests, types};

#[component]
fn OrderItemComponent(
    item: Signal<OrderItem>,
    on_increase: WriteSignal<String>,
    on_decrease: WriteSignal<String>,
    on_remove: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-3 bg-surface-elevated rounded-lg border border-border">
            <div class="flex-1 min-w-0">
                <h4 class="text-text font-medium truncate">{move || item.get().name}</h4>
                <div class="flex items-center justify-between mt-1">
                    <span class="text-sm text-text-muted">{move || format!("€{:.2} each", item.get().price)}</span>
                    <span class="text-sm font-semibold text-primary">{move || format!("€{:.2}", item.get().total())}</span>
                </div>
            </div>

            <div class="flex items-center space-x-2 ml-4">
                <div class="flex items-center space-x-1 bg-surface border border-border rounded-lg p-1">
                    <button
                        class="flex items-center justify-center text-red-600 hover:bg-red-50 hover:scale-110 rounded transition-all duration-150"
                        on:click={
                            move |_| {
                                on_decrease.set(item.get().product_id.clone());
                            }
                        }
                    >
                        <icons::Minus attr:class="w-12 h-12"/>
                    </button>

                    <span class="w-8 text-center text-sm font-medium text-text">{move || item.get().quantity}</span>

                    <button
                        class="flex items-center justify-center text-green-600 hover:bg-green-50 hover:scale-110 rounded transition-all duration-150"
                        on:click={
                            move |_| {
                                on_increase.set(item.get().product_id.clone());
                            }
                        }
                    >
                        <icons::Plus attr:class="w-12 h-12"/>
                    </button>
                </div>

                <button
                    class="w-8 h-8 flex items-center justify-center text-red-600 hover:bg-red-50 hover:scale-110 rounded transition-all duration-150"
                    on:click={
                        move |_| {
                            on_remove.set(item.get().product_id.clone());
                        }
                    }
                >
                    <icons::Trash />
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Order() -> impl IntoView {
    let order_state = order::get();
    let order_items = order_state.get_items();
    let settings_state = settings::get();
    let settings = settings_state.get_settings();
    let event_state = event::get();
    let events = event_state.get_events();
    
    // States for order creation
    let (is_creating_order, set_is_creating_order) = signal(false);
    let (order_error, set_order_error) = signal::<Option<String>>(None);

    let (increase_signal, set_increase_signal) = signal::<String>(String::new());
    let (decrease_signal, set_decrease_signal) = signal::<String>(String::new());
    let (remove_signal, set_remove_signal) = signal::<String>(String::new());

    Effect::new({
        let order_state = order_state.clone();
        move |_| {
            let item_id = increase_signal.get();
            if !item_id.is_empty() {
                order_state.increase_quantity(&item_id);
                set_increase_signal.set(String::new());
            }
        }
    });

    Effect::new({
        let order_state = order_state.clone();
        move |_| {
            let item_id = decrease_signal.get();
            if !item_id.is_empty() {
                order_state.decrease_quantity(&item_id);
                set_decrease_signal.set(String::new());
            }
        }
    });

    Effect::new({
        let order_state = order_state.clone();
        move |_| {
            let item_id = remove_signal.get();
            if !item_id.is_empty() {
                order_state.remove_item(&item_id);
                set_remove_signal.set(String::new());
            }
        }
    });

    let total_price = move || {
        order_items
            .get()
            .iter()
            .map(|item| item.total())
            .sum::<f64>()
    };

    let ids: Signal<Vec<String>> = Signal::derive({
        let order_items = order_items.clone();
        move || {
            order_items
                .get()
                .iter()
                .map(|item| item.product_id.clone())
                .collect()
        }
    });

    view! {
        <div class="bg-surface rounded-lg border border-border p-6 sticky top-6">
            // Active Event Indicator
            <div class="mb-4 p-3 bg-background rounded-lg border border-border">
                <div class="text-sm font-medium text-text-muted mb-1">"Active Event"</div>
                <Show
                    when=move || {
                        settings.get().and_then(|s| s.active_event_id).is_some()
                    }
                    fallback=|| view! {
                        <div class="text-sm text-red-600 font-medium">"No active event set"</div>
                        <div class="text-xs text-red-500">"Ask an admin to set an active event"</div>
                    }
                >
                    <div class="text-sm text-green-600 font-medium">
                        {move || {
                            let active_event_id = settings.get().and_then(|s| s.active_event_id);
                            if let Some(event_id) = active_event_id {
                                // Find the event name from the events list
                                events.get()
                                    .iter()
                                    .find(|e| e.id == event_id)
                                    .map(|e| e.name.clone())
                                    .unwrap_or_else(|| format!("Event: {}", event_id))
                            } else {
                                "Unknown".to_string()
                            }
                        }}
                    </div>
                </Show>
            </div>

            <Show
                when=move || !order_items.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-12">
                        <div class="text-6xl mb-4 text-text-muted">"🛒"</div>
                        <h3 class="text-lg font-medium text-text mb-2">"Your order is empty"</h3>
                        <p class="text-text-muted text-sm">"Add products from the menu to get started"</p>
                    </div>
                }
            >
                <div class="space-y-3 mb-6 max-h-96 overflow-y-auto">
                    <For
                        each=ids
                        key=|id| id.clone()
                        children=move |id| {
                            let item_signal = Signal::derive({
                                let order_items = order_items.clone();
                                let id = id.clone();
                                move || {
                                    order_items.get().iter().find(|i| i.product_id == id).cloned().unwrap()
                                }
                            });

                            view! {
                                <OrderItemComponent
                                    item=item_signal
                                    on_increase=set_increase_signal
                                    on_decrease=set_decrease_signal
                                    on_remove=set_remove_signal
                                />
                            }
                        }
                    />
                </div>

                <div class="border-t border-border pt-4 space-y-4">
                    <div class="flex items-center justify-between text-lg font-semibold">
                        <span class="text-text">"Total"</span>
                        <span class="text-primary">{move || format!("€{:.2}", total_price())}</span>
                    </div>

                    <Show when=move || order_error.get().is_some()>
                        <div class="p-3 bg-red-50 border border-red-200 rounded-lg">
                            <p class="text-sm text-red-600">{move || order_error.get().unwrap_or_default()}</p>
                        </div>
                    </Show>

                    <button 
                        class=move || format!(
                            "w-full font-semibold py-4 px-6 rounded-lg transition-colors duration-200 text-lg shadow-lg hover:shadow-xl {}",
                            if is_creating_order.get() {
                                "bg-gray-400 text-gray-200 cursor-not-allowed"
                            } else {
                                "bg-primary hover:bg-primary/90 text-white"
                            }
                        )
                        disabled=move || is_creating_order.get()
                        on:click={
                            let order_state = order_state.clone();
                            let order_items = order_items.clone();
                            let set_is_creating_order = set_is_creating_order.clone();
                            let set_order_error = set_order_error.clone();
                            let is_creating_order = is_creating_order.clone();
                            
                            move |_| {
                                if is_creating_order.get_untracked() {
                                    return; // Prevent double-submission
                                }
                                
                                let current_items = order_items.get_untracked();
                                if current_items.is_empty() {
                                    return; // Don't submit empty orders
                                }
                                
                                set_is_creating_order.set(true);
                                set_order_error.set(None);
                                
                                let items: Vec<types::Item> = current_items
                                    .iter()
                                    .enumerate()
                                    .map(|(index, item)| types::Item {
                                        id: format!("temp_{}", index), // Temporary ID, will be replaced by backend
                                        order_id: None, // Will be set by backend
                                        product_id: item.product_id.clone(),
                                        quantity: item.quantity,
                                        price: item.price,
                                        status: types::OrderStatus::Ordered,
                                    })
                                    .collect();
                                
                                // Get active event or show error if none set
                                let active_event_id = settings.get_untracked()
                                    .and_then(|s| s.active_event_id);
                                
                                let event_id = match active_event_id {
                                    Some(id) => id,
                                    None => {
                                        set_order_error.set(Some("No active event set. Please ask an admin to set an active event.".to_string()));
                                        set_is_creating_order.set(false);
                                        return;
                                    }
                                };
                                
                                let request = requests::order::Create {
                                    event: event_id,
                                    items,
                                };
                                
                                let order_state = order_state.clone();
                                let set_is_creating_order = set_is_creating_order.clone();
                                let set_order_error = set_order_error.clone();
                                
                                spawn_local(async move {
                                    match create_order(request).await {
                                        Ok(_) => {
                                            // Order created successfully, clear the cart
                                            order_state.clear();
                                            set_is_creating_order.set(false);
                                        }
                                        Err(e) => {
                                            set_order_error.set(Some(format!("Failed to create order: {}", e)));
                                            set_is_creating_order.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    >
                        {move || if is_creating_order.get() { "Creating Order..." } else { "Paid" }}
                    </button>
                </div>
            </Show>
        </div>
    }
}
