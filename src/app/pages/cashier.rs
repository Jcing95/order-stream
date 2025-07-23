use leptos::prelude::*;

use crate::app::components::cashier::{products::CashierProducts, order::Order, order_info::{OrderInfoComponent, provide as provide_order_info}};

#[component]
pub fn Cashier() -> impl IntoView {
    // Provide order info state
    provide_order_info();
    
    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                // Page Header
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-text mb-2">"Cashier"</h1>
                    <p class="text-text-muted">"Select products to add to the order"</p>
                </div>

                // Two-column layout: Products and Order
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-6">
                    // Products Grid (takes 2/3 of the space on small screens and up)
                    <div class="sm:col-span-2">
                        <CashierProducts />
                    </div>
                    
                    // Order sidebar (takes 1/3 of the space on small screens and up)
                    <div class="sm:col-span-1 min-w-80 mr-0 sm:mr-4 lg:mr-6">
                        <OrderInfoComponent />
                        <Order />
                    </div>
                </div>
            </div>
        </div>
    }
}