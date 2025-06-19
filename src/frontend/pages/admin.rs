use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{CreateItemRequest, Item, CreateCategoryRequest, Category, Order};
use crate::frontend::components::{
    item_form::ItemForm, 
    item_list::ItemList,
    category_form::CategoryForm,
    category_list::CategoryList,
    order_list::OrderList,
};
use crate::backend::services::items::{get_items, create_item};
use crate::backend::services::categories::{get_categories, create_category, delete_category};
use crate::backend::services::orders::{get_orders, create_order, delete_order};

#[component]
pub fn AdminPage() -> impl IntoView {
    let (items, set_items) = signal(Vec::<Item>::new());
    let (categories, set_categories) = signal(Vec::<Category>::new());
    let (orders, set_orders) = signal(Vec::<Order>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (active_tab, set_active_tab) = signal("categories".to_string());

    // Load all data on component mount
    let load_all_data = Action::new(move |_: &()| async move {
        set_loading.set(true);
        set_error.set(None);
        
        // Load categories first (needed for items)
        match get_categories().await {
            Ok(fetched_categories) => {
                set_categories.set(fetched_categories);
            }
            Err(err) => {
                set_error.set(Some(format!("Failed to load categories: {}", err)));
                set_loading.set(false);
                return;
            }
        }
        
        // Load items
        match get_items().await {
            Ok(fetched_items) => {
                set_items.set(fetched_items);
            }
            Err(err) => {
                set_error.set(Some(format!("Failed to load items: {}", err)));
            }
        }
        
        // Load orders
        match get_orders().await {
            Ok(fetched_orders) => {
                set_orders.set(fetched_orders);
            }
            Err(err) => {
                set_error.set(Some(format!("Failed to load orders: {}", err)));
            }
        }
        
        set_loading.set(false);
    });

    // Load data when component mounts
    Effect::new(move |_| {
        load_all_data.dispatch(());
    });

    // Handle category form submission
    let handle_category_submit = move |request: CreateCategoryRequest| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_category(request).await {
                Ok(new_category) => {
                    set_categories.update(|categories| categories.push(new_category));
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to create category: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };

    // Handle category deletion
    let handle_category_delete = move |category_id: String| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match delete_category(category_id.clone()).await {
                Ok(_) => {
                    set_categories.update(|categories| {
                        categories.retain(|c| c.id != category_id);
                    });
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to delete category: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };

    // Handle item form submission
    let handle_item_submit = move |request: CreateItemRequest| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_item(request).await {
                Ok(new_item) => {
                    set_items.update(|items| items.push(new_item));
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to create item: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };

    // Handle order creation
    let handle_create_order = move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_order().await {
                Ok(new_order) => {
                    set_orders.update(|orders| orders.push(new_order));
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to create order: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };

    // Handle order deletion
    let handle_order_delete = move |order_id: String| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match delete_order(order_id.clone()).await {
                Ok(_) => {
                    set_orders.update(|orders| {
                        orders.retain(|o| o.id != order_id);
                    });
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to delete order: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };

    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-6">"Admin Panel"</h1>
            
            {move || error.get().map(|err| view! {
                <div class="mb-4 text-red-600 bg-red-50 p-3 rounded">
                    {err}
                </div>
            })}
            
            {move || {
                if loading.get() {
                    view! {
                        <div class="mb-4 text-blue-600 bg-blue-50 p-3 rounded">
                            "Loading..."
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            
            // Tab navigation
            <div class="mb-6">
                <div class="border-b border-gray-200">
                    <nav class="-mb-px flex space-x-8">
                        <button
                            class=move || if active_tab.get() == "categories" { 
                                "border-indigo-500 text-indigo-600 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm" 
                            } else { 
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm" 
                            }
                            on:click=move |_| set_active_tab.set("categories".to_string())
                        >
                            "Categories"
                        </button>
                        <button
                            class=move || if active_tab.get() == "items" { 
                                "border-indigo-500 text-indigo-600 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm" 
                            } else { 
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm" 
                            }
                            on:click=move |_| set_active_tab.set("items".to_string())
                        >
                            "Items"
                        </button>
                        <button
                            class=move || if active_tab.get() == "orders" { 
                                "border-indigo-500 text-indigo-600 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm" 
                            } else { 
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm" 
                            }
                            on:click=move |_| set_active_tab.set("orders".to_string())
                        >
                            "Orders"
                        </button>
                    </nav>
                </div>
            </div>
            
            // Tab content
            {move || {
                match active_tab.get().as_str() {
                    "categories" => view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            <div>
                                <CategoryForm on_submit=handle_category_submit />
                            </div>
                            <div>
                                <CategoryList categories=categories.into() on_delete=handle_category_delete />
                            </div>
                        </div>
                    }.into_any(),
                    "items" => view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            <div>
                                <ItemForm categories=categories.into() on_submit=handle_item_submit />
                            </div>
                            <div>
                                <ItemList items=items.into() />
                            </div>
                        </div>
                    }.into_any(),
                    "orders" => view! {
                        <div class="space-y-6">
                            <div class="flex justify-between items-center">
                                <h2 class="text-xl font-semibold">"Order Management"</h2>
                                <button
                                    class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    on:click=handle_create_order
                                >
                                    "Create New Order"
                                </button>
                            </div>
                            <OrderList orders=orders.into() on_delete=handle_order_delete />
                        </div>
                    }.into_any(),
                    _ => view! {}.into_any()
                }
            }}
        </div>
    }
}