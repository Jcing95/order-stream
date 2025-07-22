use leptos::prelude::*;

use crate::app::components::admin::{
    categories::Categories, create_category::CreateCategory, create_product::CreateProduct,
    products::Products, users::Users,
};

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                // Page Header
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-text mb-2">"Admin Dashboard"</h1>
                    <p class="text-text-muted">"Manage categories, products, and users for your restaurant"</p>
                </div>

                // Main Content Grid
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    // Products Section
                    <div class="space-y-6">
                        <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                            <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                <svg class="w-5 h-5 mr-2 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"></path>
                                </svg>
                                "Product Management"
                            </h2>
                            <CreateProduct />
                        </div>
                        <Products />
                    </div>
                    // Categories Section
                    <div class="space-y-6">
                        <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                            <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                <svg class="w-5 h-5 mr-2 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                </svg>
                                "Category Management"
                            </h2>
                            <CreateCategory />
                        </div>
                        <Categories />
                    </div>
                    
                    // Users Section
                    <div class="space-y-6">
                        <div class="bg-surface rounded-lg border border-border p-6 shadow-sm">
                            <h2 class="text-xl font-semibold text-text mb-4 flex items-center">
                                <svg class="w-5 h-5 mr-2 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5 0A4 4 0 0025 12.5v-.5"></path>
                                </svg>
                                "User Management"
                            </h2>
                        </div>
                        <Users />
                    </div>
                </div>
            </div>
        </div>
    }
}
