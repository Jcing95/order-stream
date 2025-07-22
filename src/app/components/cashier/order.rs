use leptos::prelude::*;

use crate::app::{
    components::atoms::icons,
    states::order::{self, OrderItem},
};

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

                    <button class="w-full bg-primary hover:bg-primary/90 text-white font-semibold py-4 px-6 rounded-lg transition-colors duration-200 text-lg shadow-lg hover:shadow-xl">
                        "Paid"
                    </button>
                </div>
            </Show>
        </div>
    }
}
