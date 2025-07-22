use leptos::prelude::*;

use crate::{
    app::{
        components::atoms::icons,
        states::{category, product},
    },
    backend::product::{delete_product, UpdateProduct},
};

#[component]
fn ProductDisplayItem(
    product: crate::common::types::Product,
    on_edit: WriteSignal<Option<String>>,
) -> impl IntoView {
    let category_state = category::get();
    let categories = category_state.get_categories();
    
    let delete_action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            let _ = delete_product(input.clone()).await;
        }
    });

    let id = product.id.clone();
    let name = product.name.clone();
    let category_id = product.category_id.clone();
    let price = product.price;
    let active = product.active;
    
    let status_text = if active { "Active" } else { "Inactive" };
    let status_class = if active { "text-green-600" } else { "text-red-600" };
    
    // Get category name reactively
    let category_name = move || {
        categories.get()
            .iter()
            .find(|c| c.id == category_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Unknown Category".to_string())
    };

    view! {
        <div class="p-3 bg-surface-elevated rounded-md border border-border">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <div class="flex items-center justify-between">
                        <span class="text-text font-medium">
                            {name}
                            <i class="text-text-muted ml-1">
                                {move || format!("({})", category_name())}
                            </i>
                        </span>
                        <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                    </div>
                    <div class="flex items-center justify-between mt-1 text-sm text-text-muted">
                        <span class="font-medium">{format!("€{:.2}", price)}</span>
                    </div>
                    <div class="mt-1">
                        <span class={format!("text-xs font-medium {}", status_class)}>{status_text}</span>
                    </div>
                </div>
                
                <div class="flex items-center space-x-2 ml-4">
                    <button
                        class="bg-border/80 text-blue-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click={
                            let id = id.clone();
                            move |_| {
                                on_edit.set(Some(id.clone()));
                            }
                        }
                    >
                        <icons::Edit />
                    </button>
                    
                    <button
                        class="bg-border/80 text-red-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click={
                            let id = id.clone();
                            move |_| {
                                delete_action.dispatch(id.clone());
                            }
                        }
                    >
                        <icons::Trash />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProductEditItem(
    product: crate::common::types::Product,
    on_cancel: WriteSignal<Option<String>>,
) -> impl IntoView {
    let category_state = category::get();
    let categories = category_state.get_categories();
    
    let (edit_name, set_edit_name) = signal(product.name.clone());
    let (edit_category_id, set_edit_category_id) = signal(product.category_id.clone());
    let (edit_price, set_edit_price) = signal(product.price);
    let (edit_active, set_edit_active) = signal(product.active);
    
    let update_action = ServerAction::<UpdateProduct>::new();
    
    
    let id = product.id.clone();
    let original_name = product.name.clone();
    let original_category_id = product.category_id.clone();
    let original_price = product.price;
    let original_active = product.active;

    view! {
        <div class="p-3 bg-surface-elevated rounded-md border border-border">
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Name"</label>
                        <input
                            type="text"
                            prop:value=move || edit_name.get()
                            on:input=move |ev| {
                                set_edit_name.set(event_target_value(&ev));
                            }
                            class="w-full px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Category"</label>
                        <select
                            prop:value=move || edit_category_id.get()
                            on:change=move |ev| {
                                set_edit_category_id.set(event_target_value(&ev));
                            }
                            class="w-full px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                        >
                            <For
                                each=move || categories.get()
                                key=|cat| cat.id.clone()
                                children=move |cat| {
                                    let cat_id = cat.id.clone();
                                    view! {
                                        <option value={cat.id.clone()} selected=move || edit_category_id.get() == cat_id>
                                            {cat.name}
                                        </option>
                                    }
                                }
                            />
                        </select>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Price (€)"</label>
                        <input
                            type="number"
                            step="0.01"
                            min="0"
                            prop:value=move || edit_price.get().to_string()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                    set_edit_price.set(val);
                                }
                            }
                            class="w-full px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                        />
                    </div>
                    
                    <div class="flex items-center">
                        <input
                            type="checkbox"
                            prop:checked=move || edit_active.get()
                            on:change=move |ev| {
                                set_edit_active.set(event_target_checked(&ev));
                            }
                            class="h-4 w-4 text-primary focus:ring-primary border-border rounded mr-2"
                        />
                        <label class="text-sm text-text">"Active"</label>
                    </div>
                </div>
                
                <div class="flex justify-end space-x-2">
                    <ActionForm 
                        action=update_action
                        on:submit=move |_| {
                            on_cancel.set(None);
                        }
                    >
                        <input type="hidden" name="id" value={id.clone()} />
                        <input type="hidden" name="update[name]" value=move || edit_name.get() />
                        <input type="hidden" name="update[category_id]" value=move || edit_category_id.get() />
                        <input type="hidden" name="update[price]" value=move || edit_price.get() />
                        <input type="hidden" name="update[active]" value=move || edit_active.get().to_string() />
                        <button
                            type="submit"
                            class="bg-border/80 text-green-600 hover:bg-border hover:scale-105 p-2 rounded"
                        >
                            <icons::Accept />
                        </button>
                    </ActionForm>
                    
                    <button
                        class="bg-border/80 text-gray-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click=move |_| {
                            set_edit_name.set(original_name.clone());
                            set_edit_category_id.set(original_category_id.clone());
                            set_edit_price.set(original_price);
                            set_edit_active.set(original_active);
                            on_cancel.set(None);
                        }
                    >
                        <icons::Cancel />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Products() -> impl IntoView {
    let product_state = product::get();
    let products = product_state.get_products();
    let (editing_id, set_editing_id) = signal::<Option<String>>(None);

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <h2 class="text-xl font-semibold text-text mb-4">"Products"</h2>

            <Show
                when=move || !products.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-8">
                        <p class="text-text-muted">"No products found"</p>
                    </div>
                }
            >
                <div class="space-y-2">
                    <For
                        each=move || {
                            let prods = products.get();
                            leptos::logging::log!("Products component re-rendering with {} products", prods.len());
                            prods
                        }
                        key=|product| product.id.clone()
                        children=move |product| {
                            let product_id = product.id.clone();
                            let product_id_for_editing = product_id.clone();
                            let product_id_for_display = product_id.clone();
                            let product_id_for_edit = product_id.clone();
                            let product_fallback = product.clone();
                            let product_edit = product.clone();
                            
                            let is_editing = move || editing_id.get() == Some(product_id_for_editing.clone());
                            
                            view! {
                                <Show
                                    when=is_editing
                                    fallback=move || {
                                        let current_product = products.get()
                                            .iter()
                                            .find(|p| p.id == product_id_for_display)
                                            .cloned()
                                            .unwrap_or_else(|| product_fallback.clone());
                                        leptos::logging::log!("Display product for {}: {:?}", product_id_for_display, current_product);
                                        view! {
                                            <ProductDisplayItem 
                                                product=current_product
                                                on_edit=set_editing_id
                                            />
                                        }
                                    }
                                >
                                    {
                                        let product_id_for_edit_clone = product_id_for_edit.clone();
                                        let product_edit_clone = product_edit.clone();
                                        move || {
                                            let current_product = products.get()
                                                .iter()
                                                .find(|p| p.id == product_id_for_edit_clone)
                                                .cloned()
                                                .unwrap_or_else(|| product_edit_clone.clone());
                                            leptos::logging::log!("Edit product for {}: {:?}", product_id_for_edit_clone, current_product);
                                            view! {
                                                <ProductEditItem 
                                                    product=current_product
                                                    on_cancel=set_editing_id
                                                />
                                            }
                                        }
                                    }
                                </Show>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}