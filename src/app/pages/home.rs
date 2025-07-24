use leptos::prelude::*;
use crate::app::components::atoms::icons::OrderStream;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-back">
            // Hero Section
            <section class="bg-surface-elevated border-b border-border">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16 lg:py-20">
                    <div class="text-center">
                        <div class="flex justify-center mb-8">
                            <div class="p-4 bg-primary/10 rounded-2xl">
                                <OrderStream attr:class="size-16 text-primary"/>
                            </div>
                        </div>
                        <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-text mb-6">
                            "Welcome to " <span class="text-primary">"OrderStream"</span>
                        </h1>
                        <p class="text-xl sm:text-2xl text-text-muted mb-8 max-w-3xl mx-auto">
                            "Real-time ordering and kitchen management system designed specifically for small events"
                        </p>
                        <div class="flex flex-col sm:flex-row gap-4 justify-center">
                            <a 
                                href="/signin" 
                                class="px-8 py-4 bg-primary text-white font-semibold rounded-xl hover:bg-primary/90 transition-all duration-200 hover:scale-105 shadow-lg hover:shadow-xl"
                            >
                                "Get Started"
                            </a>
                            <a 
                                href="#features" 
                                class="px-8 py-4 bg-surface border border-border text-text font-semibold rounded-xl hover:bg-surface-elevated transition-all duration-200 hover:scale-105"
                            >
                                "Learn More"
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // What is OrderStream Section
            <section class="py-16 sm:py-20 bg-back" id="about">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl sm:text-4xl font-bold text-text mb-6">
                            "What is OrderStream?"
                        </h2>
                        <p class="text-lg text-text-muted max-w-3xl mx-auto">
                            "OrderStream is perfect for small event organizers who need real-time coordination for temporary kitchen setups, volunteer-friendly interfaces, and flexible station management."
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 gap-12 items-center">
                        <div class="space-y-6">
                            <div class="bg-surface rounded-xl p-6 border border-border">
                                <div class="flex items-center mb-4">
                                    <div class="p-2 bg-success/10 rounded-lg mr-4">
                                        <span class="text-2xl">"🎪"</span>
                                    </div>
                                    <h3 class="text-xl font-semibold text-text">"Perfect for Small Events"</h3>
                                </div>
                                <p class="text-text-muted">
                                    "Community gatherings, private parties, festivals, and pop-up dining experiences. Get running in under 10 minutes!"
                                </p>
                            </div>

                            <div class="bg-surface rounded-xl p-6 border border-border">
                                <div class="flex items-center mb-4">
                                    <div class="p-2 bg-primary/10 rounded-lg mr-4">
                                        <span class="text-2xl">"👥"</span>
                                    </div>
                                    <h3 class="text-xl font-semibold text-text">"Volunteer-Friendly"</h3>
                                </div>
                                <p class="text-text-muted">
                                    "Intuitive interface requiring minimal training. Perfect for volunteer kitchen staff and event workers."
                                </p>
                            </div>

                            <div class="bg-surface rounded-xl p-6 border border-border">
                                <div class="flex items-center mb-4">
                                    <div class="p-2 bg-error/10 rounded-lg mr-4">
                                        <span class="text-2xl">"🔄"</span>
                                    </div>
                                    <h3 class="text-xl font-semibold text-text">"Real-Time Updates"</h3>
                                </div>
                                <p class="text-text-muted">
                                    "WebSocket-powered live updates keep your entire team synchronized. No confusion, no missed orders!"
                                </p>
                            </div>
                        </div>

                        <div class="bg-surface rounded-2xl p-8 border border-border shadow-lg">
                            <h3 class="text-2xl font-bold text-text mb-6 text-center">"Perfect Use Cases"</h3>
                            <div class="space-y-4">
                                <div class="flex items-center p-3 bg-surface-elevated rounded-lg">
                                    <span class="text-2xl mr-3">"⛪"</span>
                                    <span class="text-text">"Church dinners & fundraisers"</span>
                                </div>
                                <div class="flex items-center p-3 bg-surface-elevated rounded-lg">
                                    <span class="text-2xl mr-3">"🎉"</span>
                                    <span class="text-text">"Wedding receptions & parties"</span>
                                </div>
                                <div class="flex items-center p-3 bg-surface-elevated rounded-lg">
                                    <span class="text-2xl mr-3">"🎵"</span>
                                    <span class="text-text">"Music festivals & craft fairs"</span>
                                </div>
                                <div class="flex items-center p-3 bg-surface-elevated rounded-lg">
                                    <span class="text-2xl mr-3">"🚚"</span>
                                    <span class="text-text">"Food trucks & pop-up dining"</span>
                                </div>
                                <div class="flex items-center p-3 bg-surface-elevated rounded-lg">
                                    <span class="text-2xl mr-3">"❤️"</span>
                                    <span class="text-text">"Charity events & disaster relief"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section class="py-16 sm:py-20 bg-surface-elevated" id="features">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl sm:text-4xl font-bold text-text mb-6">
                            "Key Features"
                        </h2>
                        <p class="text-lg text-text-muted max-w-2xl mx-auto">
                            "Everything you need to run smooth food operations at your next event"
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Cashier System
                        <div class="bg-surface rounded-xl p-6 border border-border hover:shadow-lg transition-all duration-200">
                            <div class="p-3 bg-primary/10 rounded-lg w-fit mb-4">
                                <span class="text-3xl">"🏪"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-text mb-3">"Event Cashier System"</h3>
                            <ul class="space-y-2 text-text-muted">
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Quick product selection by categories"
                                </li>
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Real-time cart building with pricing"
                                </li>
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Simple \"Alles Bezahlt!\" workflow"
                                </li>
                            </ul>
                        </div>

                        // Kitchen Stations
                        <div class="bg-surface rounded-xl p-6 border border-border hover:shadow-lg transition-all duration-200">
                            <div class="p-3 bg-success/10 rounded-lg w-fit mb-4">
                                <span class="text-3xl">"👨‍🍳"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-text mb-3">"Flexible Kitchen Stations"</h3>
                            <ul class="space-y-2 text-text-muted">
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Adaptable station views"
                                </li>
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Fun German order names"
                                </li>
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Simple status workflow"
                                </li>
                            </ul>
                        </div>

                        // Event Administration
                        <div class="bg-surface rounded-xl p-6 border border-border hover:shadow-lg transition-all duration-200">
                            <div class="p-3 bg-error/10 rounded-lg w-fit mb-4">
                                <span class="text-3xl">"🎪"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-text mb-3">"Event Administration"</h3>
                            <ul class="space-y-2 text-text-muted">
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Quick event setup & menus"
                                </li>
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "Temporary station assignments"
                                </li>
                                <li class="flex items-start">
                                    <span class="text-success mr-2 mt-1">"✓"</span>
                                    "User management for staff"
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>

            // Tutorial Section
            <section class="py-16 sm:py-20 bg-back" id="tutorial">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl sm:text-4xl font-bold text-text mb-6">
                            "How to Use OrderStream"
                        </h2>
                        <p class="text-lg text-text-muted max-w-2xl mx-auto">
                            "Get your event running smoothly in just a few simple steps"
                        </p>
                    </div>

                    <div class="space-y-12">
                        // Step 1
                        <div class="grid md:grid-cols-2 gap-8 items-center">
                            <div class="order-2 md:order-1">
                                <div class="flex items-center mb-4">
                                    <div class="w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg mr-4">
                                        "1"
                                    </div>
                                    <h3 class="text-2xl font-bold text-text">"Pre-Event Setup"</h3>
                                    <span class="ml-auto px-3 py-1 bg-success/10 text-success text-sm rounded-full">"5-10 min"</span>
                                </div>
                                <div class="pl-14 space-y-3">
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Create your event menu with categories and pricing"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Set up kitchen stations based on your setup"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Add event staff and assign roles"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Configure the active event period"
                                    </p>
                                </div>
                            </div>
                            <div class="order-1 md:order-2 bg-surface rounded-xl p-6 border border-border">
                                <div class="text-center text-6xl mb-4">"⚙️"</div>
                                <p class="text-center text-text-muted italic">
                                    "Admin configures products, stations, and event settings through the intuitive admin panel"
                                </p>
                            </div>
                        </div>

                        // Step 2
                        <div class="grid md:grid-cols-2 gap-8 items-center">
                            <div class="bg-surface rounded-xl p-6 border border-border">
                                <div class="text-center text-6xl mb-4">"🛒"</div>
                                <p class="text-center text-text-muted italic">
                                    "Cashiers use the grid-based interface to quickly build and submit customer orders"
                                </p>
                            </div>
                            <div>
                                <div class="flex items-center mb-4">
                                    <div class="w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg mr-4">
                                        "2"
                                    </div>
                                    <h3 class="text-2xl font-bold text-text">"Event Order Taking"</h3>
                                </div>
                                <div class="pl-14 space-y-3">
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Select items from your event menu grid"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Build customer orders with quantities"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Process payments and submit orders"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Orders appear instantly at kitchen stations"
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Step 3
                        <div class="grid md:grid-cols-2 gap-8 items-center">
                            <div class="order-2 md:order-1">
                                <div class="flex items-center mb-4">
                                    <div class="w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg mr-4">
                                        "3"
                                    </div>
                                    <h3 class="text-2xl font-bold text-text">"Kitchen Operations"</h3>
                                </div>
                                <div class="pl-14 space-y-3">
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Kitchen staff see station-specific views"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Orders display with fun German names"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Simple status updates as food is prepared"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Real-time coordination with other stations"
                                    </p>
                                </div>
                            </div>
                            <div class="order-1 md:order-2 bg-surface rounded-xl p-6 border border-border">
                                <div class="text-center text-6xl mb-4">"👨‍🍳"</div>
                                <p class="text-center text-text-muted italic">
                                    "Kitchen volunteers see orders like \"Bestellung 'Günther'\" and update status as they cook"
                                </p>
                            </div>
                        </div>

                        // Step 4
                        <div class="grid md:grid-cols-2 gap-8 items-center">
                            <div class="bg-surface rounded-xl p-6 border border-border">
                                <div class="text-center text-6xl mb-4">"📊"</div>
                                <p class="text-center text-text-muted italic">
                                    "Real-time updates flow instantly across all devices, keeping everyone perfectly synchronized"
                                </p>
                            </div>
                            <div>
                                <div class="flex items-center mb-4">
                                    <div class="w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg mr-4">
                                        "4"
                                    </div>
                                    <h3 class="text-2xl font-bold text-text">"Live Event Coordination"</h3>
                                </div>
                                <div class="pl-14 space-y-3">
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "New orders appear instantly at stations"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Status updates flow in real-time"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "Event organizers monitor overall progress"
                                    </p>
                                    <p class="flex items-start text-text-muted">
                                        <span class="text-primary mr-2 mt-1">"▶"</span>
                                        "No confusion, no missed orders!"
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Why OrderStream Section
            <section class="py-16 sm:py-20 bg-surface-elevated">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl sm:text-4xl font-bold text-text mb-6">
                            "Why Choose OrderStream?"
                        </h2>
                        <p class="text-lg text-text-muted max-w-2xl mx-auto">
                            "Built specifically for the unique challenges of small event food service"
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <div class="bg-surface rounded-xl p-6 border border-border text-center hover:shadow-lg transition-all duration-200">
                            <div class="text-4xl mb-4">"⚡"</div>
                            <h3 class="text-lg font-semibold text-text mb-2">"Fast Setup"</h3>
                            <p class="text-text-muted text-sm">"From zero to serving in under 10 minutes"</p>
                        </div>

                        <div class="bg-surface rounded-xl p-6 border border-border text-center hover:shadow-lg transition-all duration-200">
                            <div class="text-4xl mb-4">"👥"</div>
                            <h3 class="text-lg font-semibold text-text mb-2">"Volunteer-Friendly"</h3>
                            <p class="text-text-muted text-sm">"Intuitive enough for untrained staff"</p>
                        </div>

                        <div class="bg-surface rounded-xl p-6 border border-border text-center hover:shadow-lg transition-all duration-200">
                            <div class="text-4xl mb-4">"📱"</div>
                            <h3 class="text-lg font-semibold text-text mb-2">"Mobile-Ready"</h3>
                            <p class="text-text-muted text-sm">"Works on phones, tablets, and computers"</p>
                        </div>

                        <div class="bg-surface rounded-xl p-6 border border-border text-center hover:shadow-lg transition-all duration-200">
                            <div class="text-4xl mb-4">"🔄"</div>
                            <h3 class="text-lg font-semibold text-text mb-2">"Real-Time"</h3>
                            <p class="text-text-muted text-sm">"Perfect for dynamic event environments"</p>
                        </div>

                        <div class="bg-surface rounded-xl p-6 border border-border text-center hover:shadow-lg transition-all duration-200">
                            <div class="text-4xl mb-4">"🎪"</div>
                            <h3 class="text-lg font-semibold text-text mb-2">"Event-Focused"</h3>
                            <p class="text-text-muted text-sm">"Built specifically for temporary operations"</p>
                        </div>

                        <div class="bg-surface rounded-xl p-6 border border-border text-center hover:shadow-lg transition-all duration-200">
                            <div class="text-4xl mb-4">"💝"</div>
                            <h3 class="text-lg font-semibold text-text mb-2">"German Fun"</h3>
                            <p class="text-text-muted text-sm">"Memorable order names add charm to your event"</p>
                        </div>
                    </div>
                </div>
            </section>

            // Call to Action
            <section class="py-16 sm:py-20 bg-primary text-white">
                <div class="max-w-4xl mx-auto text-center px-4 sm:px-6 lg:px-8">
                    <h2 class="text-3xl sm:text-4xl font-bold mb-6">
                        "Ready for Your Next Event?"
                    </h2>
                    <p class="text-xl mb-8 opacity-90">
                        "Join event organizers who are already using OrderStream to run smoother, more organized food service operations."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a 
                            href="/signin" 
                            class="px-8 py-4 bg-white text-primary font-semibold rounded-xl hover:bg-gray-100 transition-all duration-200 hover:scale-105 shadow-lg"
                        >
                            "Start Your Event"
                        </a>
                        <a 
                            href="/signup" 
                            class="px-8 py-4 bg-primary-600 border-2 border-white text-white font-semibold rounded-xl hover:bg-primary-700 transition-all duration-200 hover:scale-105"
                        >
                            "Create Account"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}