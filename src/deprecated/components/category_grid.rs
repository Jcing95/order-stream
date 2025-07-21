use leptos::prelude::*;
use crate::common::types::{Category, Item, Order};
use crate::frontend::design_system::{
    atoms::TextVariant,
    theme::Intent,
    Text,
};
use crate::frontend::components::category_pane::CategoryPane;

#[component]
pub fn CategoryGrid(
    categories: Vec<Category>,
    items: Vec<Item>,
    current_order: ReadSignal<Option<Order>>,
    is_creating_order: ReadSignal<bool>,
    on_item_click: Callback<(String, u32)>,
) -> impl IntoView {
    view! {
        <div class="flex-1 p-4 overflow-y-auto">
            {
                if categories.is_empty() {
                    view! {
                        <div class="text-center p-8">
                            <Text variant=TextVariant::Body intent=Intent::Secondary>
                                "No categories available"
                            </Text>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
                            {categories.into_iter().map(|category| {
                                let category_items: Vec<Item> = items.iter()
                                    .filter(|item| item.category_id == category.id)
                                    .cloned()
                                    .collect();
                                
                                view! {
                                    <CategoryPane 
                                        category=category
                                        items=category_items
                                        on_item_click=on_item_click
                                        _current_order=current_order
                                        is_creating_order=is_creating_order
                                    />
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }
        </div>
    }
}