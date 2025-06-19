use leptos::prelude::*;
use crate::common::types::Item;

#[component]
pub fn ItemList(items: ReadSignal<Vec<Item>>) -> impl IntoView {
    view! {
        <div class="space-y-4 p-4 border rounded-lg">
            <h3 class="text-lg font-semibold">"Items"</h3>
            
            <div class="space-y-2">
                {move || {
                    let items_list = items.get();
                    if items_list.is_empty() {
                        view! {
                            <div class="text-gray-500 text-center py-4">
                                "No items yet. Add some items to get started!"
                            </div>
                        }.into_any()
                    } else {
                        items_list.into_iter().map(|item| {
                            view! {
                                <div class="flex justify-between items-center p-3 bg-gray-50 rounded">
                                    <div>
                                        <div class="font-medium">{item.name}</div>
                                        <div class="text-sm text-gray-600">{item.category}</div>
                                    </div>
                                    <div class="text-right">
                                        <div class="font-semibold">"$"{format!("{:.2}", item.price)}</div>
                                        <div class={
                                            if item.active { 
                                                "text-xs text-green-600" 
                                            } else { 
                                                "text-xs text-red-600" 
                                            }
                                        }>
                                            {if item.active { "Active" } else { "Inactive" }}
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