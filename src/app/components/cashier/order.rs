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
        <div class="p-4 bg-surface-elevated rounded-xl border border-border shadow-sm space-y-3">
            // Top Row: Product Info
            <div class="flex items-center justify-between">
                <div class="flex-1 min-w-0 mr-4">
                    <h3 class="text-lg sm:text-xl font-semibold text-text truncate">{move || item.get().name}</h3>
                    <span class="text-sm text-text-muted">{move || format!("€{:.2} each", item.get().price)}</span>
                </div>
                <span class="text-lg font-bold text-primary">{move || format!("€{:.2}", item.get().total())}</span>
            </div>

            // Bottom Row: Controls
            <div class="flex items-center justify-between">
                // Quantity Controls
                <div class="flex items-center bg-surface border-2 border-border rounded-xl p-1 shadow-sm">
                    <button
                        class="flex items-center justify-center w-14 h-14 text-error/75 hover:bg-error/10 hover:scale-105 rounded-lg transition-all duration-200 touch-manipulation"
                        on:click={
                            move |_| {
                                on_decrease.set(item.get().product_id.clone());
                            }
                        }
                    >
                        <icons::Minus attr:class="w-8 h-8"/>
                    </button>

                    <span class="min-w-12 text-center text-lg font-bold text-text px-2">{move || item.get().quantity}</span>

                    <button
                        class="flex items-center justify-center w-14 h-14 text-success/75 hover:bg-success/10 hover:scale-105 rounded-lg transition-all duration-200 touch-manipulation"
                        on:click={
                            move |_| {
                                on_increase.set(item.get().product_id.clone());
                            }
                        }
                    >
                        <icons::Plus attr:class="w-8 h-8"/>
                    </button>
                </div>

                // Delete Button (smaller, less prominent)
                <button
                    class="w-10 h-10 flex items-center justify-center text-text-muted hover:text-error hover:bg-error/10 hover:scale-105 rounded-lg transition-all duration-200 touch-manipulation"
                    on:click={
                        move |_| {
                            on_remove.set(item.get().product_id.clone());
                        }
                    }
                >
                    <icons::Trash attr:class="w-5 h-5"/>
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
        <div class="bg-surface rounded-xl border border-border p-4 sm:p-6 sticky top-6 shadow-lg">
            // Active Event Indicator
            <div class="mb-6 p-4 bg-surface-elevated rounded-xl border border-border shadow-sm">
                <div class="text-sm font-semibold text-text-muted mb-2">"Active Event"</div>
                <Show
                    when=move || {
                        settings.get().and_then(|s| s.active_event_id).is_some()
                    }
                    fallback=|| view! {
                        <div class="text-sm text-error font-semibold">"No active event set"</div>
                        <div class="text-xs text-error opacity-80">"Ask an admin to set an active event"</div>
                    }
                >
                    <div class="text-sm text-success font-semibold">
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
                    <div class="text-center py-8 sm:py-12">
                        <div class="text-5xl sm:text-6xl mb-4 text-text-muted">"🛒"</div>
                        <h3 class="text-lg sm:text-xl font-semibold text-text mb-2">"Your order is empty"</h3>
                        <p class="text-text-muted text-sm sm:text-base">"Add products from the menu to get started"</p>
                    </div>
                }
            >
                <div class="space-y-4 mb-6 max-h-96 overflow-y-auto pr-1">
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

                <div class="border-t-2 border-border pt-6 space-y-6">
                    <div class="flex items-center justify-between p-4 bg-surface-elevated rounded-xl border border-border">
                        <span class="text-xl sm:text-2xl font-bold text-text">"Total"</span>
                        <span class="text-xl sm:text-2xl font-bold text-primary">{move || format!("€{:.2}", total_price())}</span>
                    </div>

                    <Show when=move || order_error.get().is_some()>
                        <div class="p-4 bg-error/10 border border-error/20 rounded-xl">
                            <p class="text-sm text-error font-medium">{move || order_error.get().unwrap_or_default()}</p>
                        </div>
                    </Show>

                    <button 
                        class=move || format!(
                            "w-full font-bold py-5 px-6 rounded-xl transition-all duration-200 text-xl shadow-lg hover:shadow-xl touch-manipulation {}",
                            if is_creating_order.get() {
                                "bg-text-muted/30 text-text-muted cursor-not-allowed"
                            } else {
                                "bg-primary hover:bg-primary/90 hover:scale-105 text-white"
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
                        {move || if is_creating_order.get() { "Creating Order..." } else { "Alles Bezahlt!" }}
                    </button>
                </div>
            </Show>
        </div>
    }
}
