use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use crate::frontend::design_system::{Button, ThemeContext, Theme, Size, Intent};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Order Stream Demo"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-col items-center justify-center flex-1 space-y-8">
                    <h1 class="text-4xl font-bold">"Order Stream"</h1>
                    <p class="text-xl text-center max-w-md">
                        "Streamlining food and drink logistics at small events"
                    </p>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-full max-w-4xl">
                        // Admin and Management
                        <A href="/admin">
                            <div class="bg-white text-blue-800 px-6 py-4 rounded-lg font-semibold hover:bg-gray-100 transition-colors text-center">
                                <div class="text-lg">"🔧"</div>
                                <div>"Admin Panel"</div>
                                <div class="text-sm opacity-70">"Manage items & settings"</div>
                            </div>
                        </A>

                        // Cashier Station
                        <A href="/cashier">
                            <div class="bg-green-500 text-white px-6 py-4 rounded-lg font-semibold hover:bg-green-600 transition-colors text-center">
                                <div class="text-lg">"💰"</div>
                                <div>"Cashier Station"</div>
                                <div class="text-sm opacity-90">"Take orders & payments"</div>
                            </div>
                        </A>

                        // Bar Station
                        <A href="/bar">
                            <div class="bg-purple-500 text-white px-6 py-4 rounded-lg font-semibold hover:bg-purple-600 transition-colors text-center">
                                <div class="text-lg">"🍸"</div>
                                <div>"Bar Station"</div>
                                <div class="text-sm opacity-90">"Alcoholic drinks & cocktails"</div>
                            </div>
                        </A>

                        // Kitchen Station
                        <A href="/kitchen">
                            <div class="bg-orange-500 text-white px-6 py-4 rounded-lg font-semibold hover:bg-orange-600 transition-colors text-center">
                                <div class="text-lg">"🍳"</div>
                                <div>"Kitchen Station"</div>
                                <div class="text-sm opacity-90">"Hot food & prepared meals"</div>
                            </div>
                        </A>

                        // Drinks Station
                        <A href="/drinks">
                            <div class="bg-cyan-500 text-white px-6 py-4 rounded-lg font-semibold hover:bg-cyan-600 transition-colors text-center">
                                <div class="text-lg">"🥤"</div>
                                <div>"Drinks Station"</div>
                                <div class="text-sm opacity-90">"Beverages & soft drinks"</div>
                            </div>
                        </A>

                        // Food Station
                        <A href="/food">
                            <div class="bg-yellow-500 text-white px-6 py-4 rounded-lg font-semibold hover:bg-yellow-600 transition-colors text-center">
                                <div class="text-lg">"🍿"</div>
                                <div>"Food Station"</div>
                                <div class="text-sm opacity-90">"Snacks & cold food"</div>
                            </div>
                        </A>

                        // All Items Station
                        <A href="/station">
                            <div class="bg-gray-600 text-white px-6 py-4 rounded-lg font-semibold hover:bg-gray-700 transition-colors text-center">
                                <div class="text-lg">"📋"</div>
                                <div>"All Orders"</div>
                                <div class="text-sm opacity-90">"View all order types"</div>
                            </div>
                        </A>

                        // Design System
                        <A href="/design-system">
                            <div class="bg-indigo-500 text-white px-6 py-4 rounded-lg font-semibold hover:bg-indigo-600 transition-colors text-center">
                                <div class="text-lg">"🎨"</div>
                                <div>"Design System"</div>
                                <div class="text-sm opacity-90">"UI components & theming"</div>
                            </div>
                        </A>
                    </div>
                    
                    // Theme test section
                    <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 space-y-4">
                            <h3 class="text-lg font-semibold">"Design System Test"</h3>
                            
                            <div class="flex space-x-2">
                                <Button 
                                    size=Size::Md
                                    intent=Intent::Primary
                                    on_click=Callback::new(move |_| {
                                        ThemeContext::set_theme(Theme::light());
                                    })
                                >
                                    "Light Theme"
                                </Button>
                                
                                <Button 
                                    size=Size::Md
                                    intent=Intent::Secondary
                                    on_click=Callback::new(move |_| {
                                        ThemeContext::set_theme(Theme::dark());
                                    })
                                >
                                    "Dark Theme"
                                </Button>
                            </div>
                            
                            <div class="space-y-4">
                                <div class="flex space-x-2 items-center">
                                    <Button size=Size::Xs intent=Intent::Secondary>
                                        "XS"
                                    </Button>
                                    <Button size=Size::Sm intent=Intent::Secondary>
                                        "Small"
                                    </Button>
                                    <Button size=Size::Md intent=Intent::Success>
                                        "Medium"
                                    </Button>
                                    <Button size=Size::Lg intent=Intent::Danger>
                                        "Large"
                                    </Button>
                                    <Button size=Size::Xl intent=Intent::Warning>
                                        "XL"
                                    </Button>
                                </div>
                                <div class="text-sm text-white/70">
                                    "Button size progression test - check if sizes look proportional"
                                </div>
                            </div>
                        </div>
                </div>
            </div>
        </main>
    }
}