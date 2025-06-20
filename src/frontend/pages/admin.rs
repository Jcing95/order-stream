use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::frontend::components::{
    tab_nav::TabNav,
    category_section::CategorySection,
    item_section::ItemSection,
    order_section::OrderSection,
    theme_toggle::ThemeToggle,
};
use crate::frontend::state::{admin::AdminState, theme::ThemeState};

#[component]
pub fn AdminPage() -> impl IntoView {
    let state = AdminState::new();
    let (active_tab, set_active_tab) = signal("categories".to_string());
    let theme_state = expect_context::<ThemeState>();

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
        <div class="container mx-auto p-6">
            <div class="flex justify-between items-center mb-8">
                <h1 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">"Admin Panel"</h1>
                <ThemeToggle theme_state />
            </div>
            
            {move || state.error.get().map(|err| view! {
                <div class="mb-6 text-red-700 dark:text-red-300 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-4 rounded-xl shadow-sm">
                    <div class="flex items-center space-x-2">
                        <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                        </svg>
                        <span class="font-medium">{err}</span>
                    </div>
                </div>
            })}
            
            {move || {
                if state.loading.get() {
                    view! {
                        <div class="mb-6 text-blue-700 dark:text-blue-300 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 p-4 rounded-xl shadow-sm">
                            <div class="flex items-center space-x-3">
                                <div class="animate-spin rounded-full h-5 w-5 border-2 border-blue-200 border-t-blue-600 dark:border-blue-600 dark:border-t-blue-400"></div>
                                <span class="font-medium">"Loading admin data..."</span>
                            </div>
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