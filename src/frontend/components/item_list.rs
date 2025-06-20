use leptos::prelude::*;
use crate::common::types::Item;
use crate::frontend::state::theme::{card_surface, text_primary, text_secondary, text_muted, text_success, text_error};

#[component]
pub fn ItemList(items: ReadSignal<Vec<Item>>) -> impl IntoView {
    view! {
        <div class=format!("space-y-4 p-4 {}", card_surface())>
            <h3 class=format!("text-lg font-semibold {}", text_primary())>"Items"</h3>
            
            <div class="space-y-2">
                {move || {
                    let items_list = items.get();
                    if items_list.is_empty() {
                        view! {
                            <div class=format!("text-center py-4 {}", text_muted())>
                                "No items yet. Add some items to get started!"
                            </div>
                        }.into_any()
                    } else {
                        items_list.into_iter().map(|item| {
                            view! {
                                <div class="flex justify-between items-center p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                    <div>
                                        <div class=format!("font-medium {}", text_primary())>{move || item.name.clone()}</div>
                                        <div class=format!("text-sm {}", text_secondary())>{move || item.category_id.clone()}</div>
                                    </div>
                                    <div class="text-right">
                                        <div class=format!("font-semibold {}", text_primary())>"$"{move || format!("{:.2}", item.price)}</div>
                                        <div class={move || 
                                            if item.active { 
                                                format!("text-xs {}", text_success())
                                            } else { 
                                                format!("text-xs {}", text_error())
                                            }
                                        }>
                                            {move || if item.active { "Active" } else { "Inactive" }}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}