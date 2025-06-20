use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::frontend::components::{
    tab_nav::TabNav,
    category_section::CategorySection,
    item_section::ItemSection,
    order_section::OrderSection,
};
use crate::frontend::state::admin::AdminState;

#[component]
pub fn AdminPage() -> impl IntoView {
    let state = AdminState::new();
    let (active_tab, set_active_tab) = signal("categories".to_string());

    // Load data when component mounts
    Effect::new({
        let state = state.clone();
        move |_| {
            let state = state.clone();
            spawn_local(async move {
                state.load_all().await;
            });
        }
    });


    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-6">"Admin Panel"</h1>
            
            {move || state.error.get().map(|err| view! {
                <div class="mb-4 text-red-600 bg-red-50 p-3 rounded">
                    {err}
                </div>
            })}
            
            {move || {
                if state.loading.get() {
                    view! {
                        <div class="mb-4 text-blue-600 bg-blue-50 p-3 rounded">
                            "Loading..."
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            
            <TabNav 
                active_tab=active_tab.into()
                set_active_tab=set_active_tab
                tabs=vec![
                    ("categories", "Categories"),
                    ("items", "Items"),
                    ("orders", "Orders"),
                ]
            />
            
            // Tab content
            {move || {
                let state = state.clone();
                match active_tab.get().as_str() {
                    "categories" => {
                        view! {
                            <CategorySection 
                                categories=state.categories.read_only() 
                                on_submit=move |request| state.create_category(request) 
                                on_delete=move |category_id| state.delete_category(category_id) 
                            />
                        }.into_any()
                    },
                    "items" => {
                        view! {
                            <ItemSection 
                                categories=state.categories.read_only() 
                                items=state.items.read_only()
                                on_submit=move |request| state.create_item(request) 
                            />
                        }.into_any()
                    },
                    "orders" => {
                        view! {
                            <OrderSection 
                                orders=state.orders.read_only() 
                                on_create=move |_| state.create_order() 
                                on_delete=move |order_id| state.delete_order(order_id) 
                            />
                        }.into_any()
                    },
                    _ => view! {}.into_any()
                }
            }}
        </div>
    }
}