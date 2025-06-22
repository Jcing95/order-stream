use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use crate::frontend::design_system::{Card, CardVariant};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Order Stream Demo"/>
        <main>
            <div class="flex flex-col min-h-screen">
                <div class="flex flex-col items-center justify-center flex-1 space-y-8">
                    <h1 class="text-4xl font-bold">"Order Stream"</h1>
                    <p class="text-xl text-center max-w-md">
                        "Streamlining food and drink logistics at small events"
                    </p>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-full max-w-4xl">
                        // Admin and Management
                        <A href="/admin">
                            <Card variant=CardVariant::Default>
                                    <div class="text-lg">"🔧"</div>
                                    <div>"Admin Panel"</div>
                                    <div class="text-sm opacity-70">"Manage items & settings"</div>
                            </Card>
                        </A>

                        // Cashier Station
                        <A href="/cashier">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"💰"</div>
                                <div>"Cashier Station"</div>
                                <div class="text-sm opacity-90">"Take orders & payments"</div>
                            </Card>
                        </A>

                        // Bar Station
                        <A href="/bar">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🍸"</div>
                                <div>"Bar Station"</div>
                                <div class="text-sm opacity-90">"Alcoholic drinks & cocktails"</div>
                            </Card>
                        </A>

                        // Kitchen Station
                        <A href="/kitchen">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🍳"</div>
                                <div>"Kitchen Station"</div>
                                <div class="text-sm opacity-90">"Hot food & prepared meals"</div>
                            </Card>
                        </A>

                        // Drinks Station
                        <A href="/drinks">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🥤"</div>
                                <div>"Drinks Station"</div>
                                <div class="text-sm opacity-90">"Beverages & soft drinks"</div>
                            </Card>
                        </A>

                        // Food Station
                        <A href="/food">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🍿"</div>
                                <div>"Food Station"</div>
                                <div class="text-sm opacity-90">"Snacks & cold food"</div>
                            </Card>
                        </A>

                        // All Items Station
                        <A href="/station">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"📋"</div>
                                <div>"All Orders"</div>
                                <div class="text-sm opacity-90">"View all order types"</div>
                            </Card>
                        </A>

                        // Design System
                        <A href="/design-system">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🎨"</div>
                                <div>"Design System"</div>
                                <div class="text-sm opacity-90">"UI components & theming"</div>
                            </Card>
                        </A>
                    </div>
                </div>
            </div>
        </main>
    }
}