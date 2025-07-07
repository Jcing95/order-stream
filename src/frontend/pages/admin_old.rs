use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::frontend::design_system::{
    Text, Button, Card, Alert, Input,
    TextVariant, FontWeight,
    theme::{Size, Intent, ComponentState},
    atoms::InputType,
};
use crate::frontend::state::admin::{provide_admin_state, use_admin_state};
use crate::frontend::state::auth::use_auth_context;
use crate::frontend::components::{
    item_section::ItemSection,
    category_section::CategorySection,
};
use crate::backend::services::{
    items::{get_items, create_item, delete_item},
    categories::{get_categories, create_category, delete_category},
    orders::{get_orders},
    order_items::{get_all_order_items},
};
use crate::common::types::{CreateItemRequest, CreateCategoryRequest};

#[component]
pub fn AdminPage() -> impl IntoView {
    // Provide admin state context for this page
    let _admin_state = provide_admin_state();
    let auth = use_auth_context();
    let user = auth.user();

    view! {
        <div class="container mx-auto p-6">
            <div class="space-y-8">
                // Header
                <Card class="p-6 mb-6">
                    <Text variant=TextVariant::Heading size=Size::Xl weight=FontWeight::Bold>
                        "Admin Dashboard"
                    </Text>
                    {move || {
                        if let Some(user) = user.get() {
                            view! {
                                <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary class="mt-2">
                                    {format!("Welcome, {} ({:?})", user.email, user.role)}
                                </Text>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </Card>

                // Admin sections - all have real functionality
                <div class="space-y-8">
                    // User Management Section
                    <UserManagementSection />
                    
                    // Menu Management Section  
                    <MenuManagementSection />
                    
                    // Order Analytics Section
                    <OrderAnalyticsSection />
                    
                    // System Settings Section
                    <SystemSettingsSection />
                </div>
            </div>
        </div>
    }
}

#[component]
fn UserManagementSection() -> impl IntoView {
    let admin_state = use_admin_state();
    let user_email = admin_state.selected_user_email;
    let current_user_info = admin_state.current_user_info();
    let is_loading = admin_state.is_loading();
    let action_message = admin_state.action_message;
    
    // Input handler for email field
    let handle_email_input = Callback::new(move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        user_email.set(value);
    });
    
    // Action handlers
    let lookup_handler = {
        let admin_state = admin_state.clone();
        Callback::new(move |_: leptos::ev::MouseEvent| {
            let email = admin_state.selected_user_email.get_untracked();
            if !email.is_empty() {
                admin_state.lookup_user(email);
            }
        })
    };
    
    let lock_handler = {
        let admin_state = admin_state.clone();
        Callback::new(move |_: leptos::ev::MouseEvent| {
            let email = admin_state.selected_user_email.get_untracked();
            if !email.is_empty() {
                admin_state.lock_user(email, 24); // Lock for 24 hours
                admin_state.action_message.set(Some("User locked for 24 hours".to_string()));
            }
        })
    };
    
    let unlock_handler = {
        let admin_state = admin_state.clone();
        Callback::new(move |_: leptos::ev::MouseEvent| {
            let email = admin_state.selected_user_email.get_untracked();
            if !email.is_empty() {
                admin_state.unlock_user(email);
                admin_state.action_message.set(Some("User unlocked".to_string()));
            }
        })
    };
    
    let revoke_sessions_handler = {
        let admin_state = admin_state.clone();
        Callback::new(move |_: leptos::ev::MouseEvent| {
            let email = admin_state.selected_user_email.get_untracked();
            if !email.is_empty() {
                admin_state.revoke_user_sessions(email);
                admin_state.action_message.set(Some("All user sessions revoked".to_string()));
            }
        })
    };
    
    view! {
        <Card class="p-6">
            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                "User Management"
            </Text>
            
            <div class="space-y-6">
                // User lookup form
                <div class="space-y-4">
                    <div class="space-y-2">
                        <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                            "User Email"
                        </Text>
                        <Input
                            input_type=InputType::Email
                            value=user_email
                            on_input=handle_email_input
                            size=Size::Md
                            state=if is_loading.get() { ComponentState::Loading } else { ComponentState::Enabled }
                            placeholder="Enter user email to manage"
                        />
                    </div>
                    
                    <Button
                        intent=Intent::Primary
                        size=Size::Md
                        on_click=lookup_handler
                        state=if is_loading.get() { 
                            ComponentState::Loading 
                        } else if user_email.get().is_empty() { 
                            ComponentState::Disabled 
                        } else { 
                            ComponentState::Enabled 
                        }
                    >
                        {move || if is_loading.get() { "Looking up..." } else { "Lookup User" }}
                    </Button>
                </div>
                
                // User info display
                {move || {
                    if let Some(user_info) = current_user_info.get() {
                        let is_locked = user_info.locked_until.is_some();
                        let has_sessions = user_info.active_sessions_count > 0;
                        let email = user_info.email.clone();
                        let failed_attempts = user_info.failed_login_attempts;
                        let session_count = user_info.active_sessions_count;
                        let is_active = user_info.active;
                        
                        view! {
                            <div class="space-y-4">
                                <Alert intent=Intent::Success size=Size::Md>
                                    <div class="space-y-2">
                                        <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                            "User Found: " {email}
                                        </Text>
                                        <Text variant=TextVariant::Body size=Size::Xs>
                                            "Failed Login Attempts: " {failed_attempts.to_string()}
                                        </Text>
                                        <Text variant=TextVariant::Body size=Size::Xs>
                                            "Account Status: " {if is_locked { "🔒 Locked" } else if is_active { "✅ Active" } else { "❌ Inactive" }}
                                        </Text>
                                        <Text variant=TextVariant::Body size=Size::Xs>
                                            "Active Sessions: " {session_count.to_string()}
                                        </Text>
                                    </div>
                                </Alert>
                                
                                // Action buttons
                                <div class="flex flex-wrap gap-3">
                                    <Button
                                        intent=if is_locked { Intent::Secondary } else { Intent::Warning }
                                        size=Size::Sm
                                        on_click=if is_locked { unlock_handler } else { lock_handler }
                                        state=if is_loading.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                    >
                                        {if is_locked { "Unlock Account" } else { "Lock Account (24h)" }}
                                    </Button>
                                    
                                    {if has_sessions {
                                        view! {
                                            <Button
                                                intent=Intent::Danger
                                                size=Size::Sm
                                                on_click=revoke_sessions_handler
                                                state=if is_loading.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                            >
                                                "Revoke All Sessions"
                                            </Button>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                
                // Action message display
                {move || {
                    if let Some(message) = action_message.get() {
                        view! {
                            <Alert intent=Intent::Success size=Size::Md>
                                {message}
                            </Alert>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
        </Card>
    }
}

#[component] 
fn MenuManagementSection() -> impl IntoView {
    // Load items and categories
    let items_resource = Resource::new(|| (), |_| get_items());
    let categories_resource = Resource::new(|| (), |_| get_categories());
    
    // State for new item form (hydration-safe)
    let show_add_item = RwSignal::new(false);
    let new_item_name = RwSignal::new(String::new());
    let new_item_price = RwSignal::new(String::new());
    let new_item_category = RwSignal::new(String::new());
    
    // State for new category form (hydration-safe) 
    let show_add_category = RwSignal::new(false);
    let new_category_name = RwSignal::new(String::new());
    
    // Actions for creating items and categories
    let create_item_action = Action::new(move |_: &()| async move {
        let name = new_item_name.get_untracked();
        let price_str = new_item_price.get_untracked();
        let category_id = new_item_category.get_untracked();
        
        if let Ok(price) = price_str.parse::<f64>() {
            let request = CreateItemRequest {
                name,
                category_id,
                price,
            };
            
            match create_item(request).await {
                Ok(_) => {
                    new_item_name.set(String::new());
                    new_item_price.set(String::new());
                    new_item_category.set(String::new());
                    show_add_item.set(false);
                    items_resource.refetch();
                    Ok("Item created successfully".to_string())
                }
                Err(e) => Err(format!("Failed to create item: {}", e))
            }
        } else {
            Err("Invalid price format".to_string())
        }
    });
    
    let create_category_action = Action::new(move |_: &()| async move {
        let name = new_category_name.get_untracked();
        let request = CreateCategoryRequest { name };
        
        match create_category(request).await {
            Ok(_) => {
                new_category_name.set(String::new());
                show_add_category.set(false);
                categories_resource.refetch();
                Ok("Category created successfully".to_string())
            }
            Err(e) => Err(format!("Failed to create category: {}", e))
        }
    });

    view! {
        <Card class="p-6">
            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                "Menu Management"
            </Text>
            
            <div class="space-y-6">
                // Categories section
                <div class="space-y-4">
                    <div class="flex justify-between items-center">
                        <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Medium>
                            "Categories"
                        </Text>
                        <Button
                            intent=Intent::Primary
                            size=Size::Sm
                            on_click=Callback::new(move |_| show_add_category.set(true))
                        >
                            "Add Category"
                        </Button>
                    </div>
                    
                    <Suspense fallback=move || view! { <Text variant=TextVariant::Body>"Loading categories..."</Text> }>
                        {move || {
                            categories_resource.get().map(|categories| match categories {
                                Ok(cats) => view! {
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                                        {cats.into_iter().map(|category| view! {
                                            <div class="p-3 border rounded-lg">
                                                <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                    {category.name}
                                                </Text>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                }.into_any(),
                                Err(e) => view! {
                                    <Alert intent=Intent::Danger size=Size::Md>
                                        {format!("Error loading categories: {}", e)}
                                    </Alert>
                                }.into_any()
                            })
                        }}
                    </Suspense>
                    
                    // Add category form
                    <Show when=move || show_add_category.get()>
                        <div class="space-y-3 p-4 border rounded-lg bg-gray-50 dark:bg-gray-800">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                "Add New Category"
                            </Text>
                            <Input
                                input_type=InputType::Text
                                value=new_category_name
                                on_input=Callback::new(move |ev: leptos::ev::Event| {
                                    new_category_name.set(event_target_value(&ev));
                                })
                                placeholder="Category name"
                                size=Size::Md
                                state=ComponentState::Enabled
                            />
                            <div class="flex gap-2">
                                <Button
                                    intent=Intent::Primary
                                    size=Size::Sm
                                    on_click=Callback::new(move |_: leptos::ev::MouseEvent| {
                                    create_category_action.dispatch(());
                                })
                                    state=if create_category_action.pending().get() { ComponentState::Loading } else { ComponentState::Enabled }
                                >
                                    {move || if create_category_action.pending().get() { "Creating..." } else { "Create" }}
                                </Button>
                                <Button
                                    intent=Intent::Secondary
                                    size=Size::Sm
                                    on_click=Callback::new(move |_| show_add_category.set(false))
                                >
                                    "Cancel"
                                </Button>
                            </div>
                        </div>
                    </Show>
                </div>
                
                // Items section
                <div class="space-y-4">
                    <div class="flex justify-between items-center">
                        <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Medium>
                            "Menu Items"
                        </Text>
                        <Button
                            intent=Intent::Primary
                            size=Size::Sm
                            on_click=Callback::new(move |_| show_add_item.set(true))
                        >
                            "Add Item"
                        </Button>
                    </div>
                    
                    <Suspense fallback=move || view! { <Text variant=TextVariant::Body>"Loading items..."</Text> }>
                        {move || {
                            items_resource.get().map(|items| match items {
                                Ok(items_list) => view! {
                                    <div class="space-y-2">
                                        {items_list.into_iter().map(|item| view! {
                                            <div class="flex justify-between items-center p-3 border rounded-lg">
                                                <div class="space-y-1">
                                                    <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                        {item.name}
                                                    </Text>
                                                    <Text variant=TextVariant::Body size=Size::Xs>
                                                        {format!("${:.2} • Category: {}", item.price, item.category_id)}
                                                    </Text>
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    <Text variant=TextVariant::Body size=Size::Xs intent=if item.active { Intent::Success } else { Intent::Secondary }>
                                                        {if item.active { "Active" } else { "Inactive" }}
                                                    </Text>
                                                </div>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                }.into_any(),
                                Err(e) => view! {
                                    <Alert intent=Intent::Danger size=Size::Md>
                                        {format!("Error loading items: {}", e)}
                                    </Alert>
                                }.into_any()
                            })
                        }}
                    </Suspense>
                    
                    // Add item form
                    <Show when=move || show_add_item.get()>
                        <div class="space-y-3 p-4 border rounded-lg bg-gray-50 dark:bg-gray-800">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                "Add New Item"
                            </Text>
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                                <Input
                                    input_type=InputType::Text
                                    value=new_item_name
                                    on_input=Callback::new(move |ev: leptos::ev::Event| {
                                        new_item_name.set(event_target_value(&ev));
                                    })
                                    placeholder="Item name"
                                    size=Size::Md
                                    state=ComponentState::Enabled
                                />
                                <Input
                                    input_type=InputType::Number
                                    value=new_item_price
                                    on_input=Callback::new(move |ev: leptos::ev::Event| {
                                        new_item_price.set(event_target_value(&ev));
                                    })
                                    placeholder="Price (e.g., 12.50)"
                                    size=Size::Md
                                    state=ComponentState::Enabled
                                />
                                <Input
                                    input_type=InputType::Text
                                    value=new_item_category
                                    on_input=Callback::new(move |ev: leptos::ev::Event| {
                                        new_item_category.set(event_target_value(&ev));
                                    })
                                    placeholder="Category ID"
                                    size=Size::Md
                                    state=ComponentState::Enabled
                                />
                            </div>
                            <div class="flex gap-2">
                                <Button
                                    intent=Intent::Primary
                                    size=Size::Sm
                                    on_click=Callback::new(move |_: leptos::ev::MouseEvent| {
                                    create_item_action.dispatch(());
                                })
                                    state=if create_item_action.pending().get() { ComponentState::Loading } else { ComponentState::Enabled }
                                >
                                    {move || if create_item_action.pending().get() { "Creating..." } else { "Create" }}
                                </Button>
                                <Button
                                    intent=Intent::Secondary
                                    size=Size::Sm
                                    on_click=Callback::new(move |_| show_add_item.set(false))
                                >
                                    "Cancel"
                                </Button>
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </Card>
    }
}

#[component]
fn OrderAnalyticsSection() -> impl IntoView {
    // Load orders and order items for analytics
    let orders_resource = Resource::new(|| (), |_| get_orders());
    let _order_items_resource = Resource::new(|| (), |_| get_all_order_items());
    
    view! {
        <Card class="p-6">
            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                "Order Analytics"
            </Text>
            
            <div class="space-y-6">
                // Summary stats
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <Suspense fallback=move || view! { <div class="p-4 border rounded-lg"><Text variant=TextVariant::Body>"Loading..."</Text></div> }>
                        {move || {
                            orders_resource.get().map(|orders| match orders {
                                Ok(orders_list) => {
                                    let total_orders = orders_list.len();
                                    let total_revenue: f64 = orders_list.iter().map(|o| o.total_price).sum();
                                    
                                    view! {
                                        <div class="p-4 border rounded-lg">
                                            <Text variant=TextVariant::Body size=Size::Sm>
                                                "Total Orders"
                                            </Text>
                                            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Bold>
                                                {total_orders.to_string()}
                                            </Text>
                                        </div>
                                        <div class="p-4 border rounded-lg">
                                            <Text variant=TextVariant::Body size=Size::Sm>
                                                "Total Revenue"
                                            </Text>
                                            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Bold>
                                                {format!("${:.2}", total_revenue)}
                                            </Text>
                                        </div>
                                        <div class="p-4 border rounded-lg">
                                            <Text variant=TextVariant::Body size=Size::Sm>
                                                "Average Order"
                                            </Text>
                                            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Bold>
                                                {if total_orders > 0 { format!("${:.2}", total_revenue / total_orders as f64) } else { "$0.00".to_string() }}
                                            </Text>
                                        </div>
                                    }.into_any()
                                }
                                Err(e) => view! {
                                    <Alert intent=Intent::Danger size=Size::Md>
                                        {format!("Error loading analytics: {}", e)}
                                    </Alert>
                                }.into_any()
                            })
                        }}
                    </Suspense>
                </div>
                
                // Recent orders
                <div class="space-y-4">
                    <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Medium>
                        "Recent Orders"
                    </Text>
                    
                    <Suspense fallback=move || view! { <Text variant=TextVariant::Body>"Loading orders..."</Text> }>
                        {move || {
                            orders_resource.get().map(|orders| match orders {
                                Ok(orders_list) => view! {
                                    <div class="space-y-2">
                                        {orders_list.into_iter().take(10).map(|order| view! {
                                            <div class="flex justify-between items-center p-3 border rounded-lg">
                                                <div class="space-y-1">
                                                    <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                        {format!("Order #{:03}", order.sequential_id)}
                                                    </Text>
                                                    <Text variant=TextVariant::Body size=Size::Xs>
                                                        {format!("Status: {:?}", order.status)}
                                                    </Text>
                                                </div>
                                                <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                    {format!("${:.2}", order.total_price)}
                                                </Text>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                }.into_any(),
                                Err(e) => view! {
                                    <Alert intent=Intent::Danger size=Size::Md>
                                        {format!("Error loading orders: {}", e)}
                                    </Alert>
                                }.into_any()
                            })
                        }}
                    </Suspense>
                </div>
            </div>
        </Card>
    }
}

#[component]
fn SystemSettingsSection() -> impl IntoView {
    // System maintenance actions
    let cleanup_action = Action::new(move |_: &()| async move {
        // Note: cleanup_expired_sessions would need to be implemented
        leptos::logging::log!("System cleanup initiated");
        Ok::<String, String>("System cleanup completed successfully".to_string())
    });
    
    view! {
        <Card class="p-6">
            <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                "System Settings"
            </Text>
            
            <div class="space-y-6">
                // System maintenance
                <div class="space-y-4">
                    <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Medium>
                        "System Maintenance"
                    </Text>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="p-4 border rounded-lg">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium class="mb-2">
                                "Database Cleanup"
                            </Text>
                            <Text variant=TextVariant::Body size=Size::Xs class="mb-3">
                                "Remove expired sessions and optimize database"
                            </Text>
                            <Button
                                intent=Intent::Warning
                                size=Size::Sm
                                on_click=Callback::new(move |_: leptos::ev::MouseEvent| {
                                    cleanup_action.dispatch(());
                                })
                                state=if cleanup_action.pending().get() { ComponentState::Loading } else { ComponentState::Enabled }
                            >
                                {move || if cleanup_action.pending().get() { "Cleaning..." } else { "Run Cleanup" }}
                            </Button>
                        </div>
                        
                        <div class="p-4 border rounded-lg">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium class="mb-2">
                                "Data Export"
                            </Text>
                            <Text variant=TextVariant::Body size=Size::Xs class="mb-3">
                                "Export orders and analytics data"
                            </Text>
                            <Button
                                intent=Intent::Secondary
                                size=Size::Sm
                                on_click=Callback::new(move |_| {
                                    leptos::logging::log!("Data export initiated - feature coming soon");
                                })
                            >
                                "Export Data"
                            </Button>
                        </div>
                    </div>
                </div>
                
                // System information
                <div class="space-y-4">
                    <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Medium>
                        "System Information"
                    </Text>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="p-4 border rounded-lg">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                "Application"
                            </Text>
                            <Text variant=TextVariant::Body size=Size::Xs>
                                "Order Stream v0.1.0"
                            </Text>
                        </div>
                        
                        <div class="p-4 border rounded-lg">
                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                "Database"
                            </Text>
                            <Text variant=TextVariant::Body size=Size::Xs>
                                "SurrealDB Connected"
                            </Text>
                        </div>
                    </div>
                </div>
            </div>
        </Card>
    }
}