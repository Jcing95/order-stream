use leptos::prelude::*;

use crate::app::components::admin::{
    categories::Categories, create_category::CreateCategory, create_event::CreateEvent,
    create_product::CreateProduct, create_station::CreateStation, events::Events,
    products::Products, stations::Stations, users::Users,
};
use crate::app::components::atoms::icons;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AdminTab {
    Products,
    Categories,
    Events,
    Stations,
    Users,
}

#[component]
pub fn Admin() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(AdminTab::Products);

    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                // Page Header
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-text mb-2">"Admin Dashboard"</h1>
                    <p class="text-text-muted">"Manage categories, products, and users for your restaurant"</p>
                </div>

                // Tab Navigation
                <div class="mb-8">
                    <nav class="flex space-x-1 bg-surface rounded-lg p-1 border border-border">
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors duration-200 flex items-center justify-center {}",
                                if active_tab.get() == AdminTab::Products {
                                    "bg-primary text-white shadow-sm"
                                } else {
                                    "text-text hover:bg-background hover:text-text"
                                }
                            )
                            on:click=move |_| set_active_tab.set(AdminTab::Products)
                        >
                            <icons::Box attr:class="w-4 h-4 mr-2"/>
                            "Products"
                        </button>
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors duration-200 flex items-center justify-center {}",
                                if active_tab.get() == AdminTab::Categories {
                                    "bg-primary text-white shadow-sm"
                                } else {
                                    "text-text hover:bg-background hover:text-text"
                                }
                            )
                            on:click=move |_| set_active_tab.set(AdminTab::Categories)
                        >
                            <icons::Tag attr:class="w-4 h-4 mr-2"/>
                            "Categories"
                        </button>
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors duration-200 flex items-center justify-center {}",
                                if active_tab.get() == AdminTab::Events {
                                    "bg-primary text-white shadow-sm"
                                } else {
                                    "text-text hover:bg-background hover:text-text"
                                }
                            )
                            on:click=move |_| set_active_tab.set(AdminTab::Events)
                        >
                            <icons::Calendar attr:class="w-4 h-4 mr-2"/>
                            "Events"
                        </button>
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors duration-200 flex items-center justify-center {}",
                                if active_tab.get() == AdminTab::Stations {
                                    "bg-primary text-white shadow-sm"
                                } else {
                                    "text-text hover:bg-background hover:text-text"
                                }
                            )
                            on:click=move |_| set_active_tab.set(AdminTab::Stations)
                        >
                            <icons::Building attr:class="w-4 h-4 mr-2"/>
                            "Stations"
                        </button>
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors duration-200 flex items-center justify-center {}",
                                if active_tab.get() == AdminTab::Users {
                                    "bg-primary text-white shadow-sm"
                                } else {
                                    "text-text hover:bg-background hover:text-text"
                                }
                            )
                            on:click=move |_| set_active_tab.set(AdminTab::Users)
                        >
                            <icons::Users attr:class="w-4 h-4 mr-2"/>
                            "Users"
                        </button>
                    </nav>
                </div>

                // Tab Content
                <div class="space-y-6">
                    {move || match active_tab.get() {
                        AdminTab::Products => view! {
                            <div class="space-y-6">
                                <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                                    <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                        <icons::Box attr:class="w-5 h-5 mr-2 text-primary"/>
                                        "Product Management"
                                    </h2>
                                    <CreateProduct />
                                </div>
                                <Products />
                            </div>
                        }.into_any(),
                        AdminTab::Categories => view! {
                            <div class="space-y-6">
                                <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                                    <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                        <icons::Tag attr:class="w-5 h-5 mr-2 text-primary"/>
                                        "Category Management"
                                    </h2>
                                    <CreateCategory />
                                </div>
                                <Categories />
                            </div>
                        }.into_any(),
                        AdminTab::Events => view! {
                            <div class="space-y-6">
                                <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                                    <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                        <icons::Calendar attr:class="w-5 h-5 mr-2 text-primary"/>
                                        "Event Management"
                                    </h2>
                                    <CreateEvent />
                                </div>
                                <Events />
                            </div>
                        }.into_any(),
                        AdminTab::Stations => view! {
                            <div class="space-y-6">
                                <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                                    <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                        <icons::Building attr:class="w-5 h-5 mr-2 text-primary"/>
                                        "Station Management"
                                    </h2>
                                    <CreateStation />
                                </div>
                                <Stations />
                            </div>
                        }.into_any(),
                        AdminTab::Users => view! {
                            <div class="space-y-6">
                                <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                                    <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                        <icons::Users attr:class="w-5 h-5 mr-2 text-primary"/>
                                        "User Management"
                                    </h2>
                                </div>
                                <Users />
                            </div>
                        }.into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}
