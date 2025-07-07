use leptos::prelude::*;
use crate::common::types::{Category, Item, CreateItemRequest};
use crate::frontend::components::{item_form::ItemForm, item_list::ItemList};

#[component]
pub fn ItemSection<F>(
    categories: ReadSignal<Vec<Category>>,
    items: ReadSignal<Vec<Item>>,
    on_submit: F,
) -> impl IntoView
where
    F: Fn(CreateItemRequest) + 'static + Clone + Send + Sync,
{
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div>
                <ItemForm categories=categories.into() on_submit=on_submit />
            </div>
            <div>
                <ItemList items=items />
            </div>
        </div>
    }
}