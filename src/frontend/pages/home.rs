use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use crate::frontend::design_system::{
    Card, CardVariant, Text,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Order Stream Demo"/>
        <main>
            <div class="flex flex-col min-h-screen">
                <div class="flex flex-col items-center justify-center flex-1 space-y-8">
                    <Text 
                        variant=TextVariant::Heading 
                        size=Size::Xl 
                        weight=FontWeight::Bold
                        as_element="h1"
                    >
                        "Order Stream"
                    </Text>
                    <Text 
                        variant=TextVariant::Body 
                        size=Size::Lg 
                        intent=Intent::Secondary
                        class="text-center max-w-md"
                    >
                        "Streamlining food and drink logistics at small events"
                    </Text>
                    
                    <div class="grid grid-cols-1 gap-4 w-full max-w-4xl">
                        // Admin and Management
                        <A href="/admin/">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🔧"</div>
                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold>
                                    "Admin Panel"
                                </Text>
                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                    "Manage items & settings"
                                </Text>
                            </Card>
                        </A>

                        // Cashier Station
                        <A href="/cashier/">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"💰"</div>
                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold>
                                    "Cashier"
                                </Text>
                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                    "Take orders & payments"
                                </Text>
                            </Card>
                        </A>

                        // All Items Station
                        <A href="/stations/">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"📋"</div>
                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold>
                                    "Select Station"
                                </Text>
                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                    "View all order types"
                                </Text>
                            </Card>
                        </A>

                        // Design System
                        <A href="/design-system/">
                            <Card variant=CardVariant::Default>
                                <div class="text-lg">"🎨"</div>
                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold>
                                    "Design System"
                                </Text>
                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                    "UI components & theming"
                                </Text>
                            </Card>
                        </A>
                    </div>
                </div>
            </div>
        </main>
    }
}