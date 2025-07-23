use leptos::prelude::*;

use crate::app::states::{category, product, order};

#[component]
fn ProductButton(product: crate::common::types::Product) -> impl IntoView {
    let order_state = order::get();
    let name = product.name.clone();
    let price = product.price;
    let active = product.active;
    let product_for_click = product.clone();

    view! {
        <button
            class=format!(
                "w-full h-24 p-4 rounded-lg border-2 transition-all duration-200 flex flex-col items-center justify-center text-center {}",
                if active {
                    "bg-surface hover:bg-surface-elevated border-border hover:border-primary hover:shadow-lg hover:scale-105 text-text"
                } else {
                    "bg-surface-disabled border-border-disabled text-text-disabled cursor-not-allowed opacity-50"
                }
            )
            disabled=!active
            on:click=move |_| {
                if active {
                    order_state.add_product(product_for_click.clone());
                }
            }
        >
            <div class="font-semibold text-lg leading-tight mb-1">{name}</div>
            <div class="text-sm font-medium text-primary">{format!("€{:.2}", price)}</div>
        </button>
    }
}

#[component]
fn CategorySection(
    category: crate::common::types::Category,
    products: Vec<crate::common::types::Product>,
) -> impl IntoView {
    view! {
        <div class="mb-8">
            <h2 class="text-2xl font-bold text-text mb-4 border-b border-border pb-2">
                {category.name}
            </h2>
            <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-4 gap-3 sm:gap-4">
                <For
                    each=move || products.clone()
                    key=|product| product.id.clone()
                    children=move |product| {
                        view! {
                            <ProductButton product=product />
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn CashierProducts() -> impl IntoView {
    let product_state = product::get();
    let category_state = category::get();
    let products = product_state.get_products();
    let categories = category_state.get_categories();

    let products_by_category = move || {
        let all_products = products.get();
        let all_categories = categories.get();
        
        all_categories
            .into_iter()
            .map(|category| {
                let category_products: Vec<_> = all_products
                    .iter()
                    .filter(|product| product.category_id == category.id)
                    .cloned()
                    .collect();
                (category, category_products)
            })
            .filter(|(_, products)| !products.is_empty())
            .collect::<Vec<_>>()
    };

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <Show
                when=move || !products_by_category().is_empty()
                fallback=|| view! {
                    <div class="text-center py-12">
                        <div class="text-6xl mb-4 text-text-muted">"🍽️"</div>
                        <h3 class="text-xl font-semibold text-text mb-2">"No products available"</h3>
                        <p class="text-text-muted">"Products will appear here once they are added to the system"</p>
                    </div>
                }
            >
                <div class="space-y-8">
                    <For
                        each=products_by_category
                        key=|(category, _)| category.id.clone()
                        children=move |(category, products)| {
                            view! {
                                <CategorySection category=category products=products />
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}