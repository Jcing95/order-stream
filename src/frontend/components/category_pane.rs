use leptos::prelude::*;
use crate::common::types::{Category, Item, Order};
use crate::frontend::design_system::{
    atoms::{FontWeight, TextVariant},
    theme::{Intent, Size},
    Text, Card, CardVariant,
};

#[component]
pub fn CategoryPane(
    category: Category,
    items: Vec<Item>,
    on_item_click: Callback<(String, u32)>,
    current_order: ReadSignal<Option<Order>>,
    pending_item: RwSignal<Option<(String, u32)>>,
    is_creating_order: ReadSignal<bool>,
    create_new_order: Action<(), ()>,
) -> impl IntoView {
    view! {
        <Card variant=CardVariant::Default>
            <div class="p-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Md 
                    weight=FontWeight::Semibold
                    class="mb-4"
                >
                    {category.name}
                </Text>
                
                <div class="grid grid-cols-2 gap-3">
                    {items.into_iter().map(|item| {
                        let item_id = item.id.clone();
                        let item_name = item.name.clone();
                        let item_price = item.price;
                        
                        view! {
                            <button
                                class=move || {
                                    let base_class = "h-20 flex flex-col items-center justify-center text-center p-3 border rounded transition-colors";
                                    let is_creating = is_creating_order.get();
                                    let has_pending = pending_item.get().is_some();
                                    
                                    if is_creating || has_pending {
                                        format!("{} bg-gray-100 border-gray-300 cursor-wait opacity-75 dark:bg-slate-700 dark:border-slate-600", base_class)
                                    } else {
                                        format!("{} bg-white hover:bg-green-50 dark:bg-slate-800 dark:hover:bg-green-900/20 border-gray-300 dark:border-slate-600 hover:border-green-300 dark:hover:border-green-600 cursor-pointer", base_class)
                                    }
                                }
                                on:click=move |_| {
                                    if !is_creating_order.get_untracked() && pending_item.get_untracked().is_none() {
                                        // Auto-create order if none exists
                                        if current_order.get_untracked().is_none() {
                                            // Store pending item and create order
                                            pending_item.set(Some((item_id.clone(), 1)));
                                            create_new_order.dispatch(());
                                        } else {
                                            // Order exists, add item directly
                                            on_item_click.run((item_id.clone(), 1));
                                        }
                                    }
                                }
                                disabled=move || is_creating_order.get() || pending_item.get().is_some()
                            >
                                <Text 
                                    variant=TextVariant::Body 
                                    size=Size::Sm 
                                    weight=FontWeight::Semibold
                                    class="mb-1"
                                >
                                    {item_name.clone()}
                                </Text>
                                <Text 
                                    variant=TextVariant::Caption 
                                    size=Size::Xs
                                    intent=Intent::Success
                                >
                                    "$" {format!("{:.2}", item_price)}
                                </Text>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </Card>
    }
}