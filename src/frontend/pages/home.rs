use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use crate::frontend::design_system::{
    Card, CardVariant, Text, Button,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};
use crate::frontend::state::auth::use_auth_context;

#[component]
pub fn Home() -> impl IntoView {
    let auth = use_auth_context();
    let user = auth.user();
    let is_authenticated = auth.is_authenticated();

    view! {
        <Title text="Order Stream"/>
        <main>
            <div class="flex flex-col min-h-screen">
                <div class="flex flex-col items-center justify-center flex-1 space-y-8 p-6">
                    <div class="text-center space-y-4">
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
                    </div>
                    
                    {move || {
                        if is_authenticated.get() {
                            if let Some(user) = user.get() {
                                view! {
                                    <div class="space-y-6 w-full max-w-2xl">
                                        <div class="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg border border-green-200 dark:border-green-800">
                                            <Text variant=TextVariant::Body weight=FontWeight::Medium class="mb-2">
                                                {format!("Welcome back, {}!", user.email)}
                                            </Text>
                                            <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary>
                                                {format!("Logged in as: {:?}", user.role)}
                                            </Text>
                                        </div>
                                        
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                            {move || {
                                                match user.role {
                                                    crate::common::types::UserRole::Admin => view! {
                                                        <A href="/admin">
                                                            <Card variant=CardVariant::Default>
                                                                <div class="text-2xl mb-2">"🔧"</div>
                                                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold class="mb-1">
                                                                    "Admin Panel"
                                                                </Text>
                                                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                                                    "Manage system and users"
                                                                </Text>
                                                            </Card>
                                                        </A>
                                                        <A href="/cashier">
                                                            <Card variant=CardVariant::Default>
                                                                <div class="text-2xl mb-2">"💰"</div>
                                                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold class="mb-1">
                                                                    "Cashier"
                                                                </Text>
                                                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                                                    "Take orders & payments"
                                                                </Text>
                                                            </Card>
                                                        </A>
                                                        <A href="/stations">
                                                            <Card variant=CardVariant::Default>
                                                                <div class="text-2xl mb-2">"🍽️"</div>
                                                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold class="mb-1">
                                                                    "Stations"
                                                                </Text>
                                                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                                                    "View preparation stations"
                                                                </Text>
                                                            </Card>
                                                        </A>
                                                    }.into_any(),
                                                    crate::common::types::UserRole::Cashier => view! {
                                                        <A href="/cashier">
                                                            <Card variant=CardVariant::Default>
                                                                <div class="text-2xl mb-2">"💰"</div>
                                                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold class="mb-1">
                                                                    "Cashier"
                                                                </Text>
                                                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                                                    "Take orders & payments"
                                                                </Text>
                                                            </Card>
                                                        </A>
                                                        <A href="/stations">
                                                            <Card variant=CardVariant::Default>
                                                                <div class="text-2xl mb-2">"🍽️"</div>
                                                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold class="mb-1">
                                                                    "Stations"
                                                                </Text>
                                                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                                                    "View preparation stations"
                                                                </Text>
                                                            </Card>
                                                        </A>
                                                    }.into_any(),
                                                    crate::common::types::UserRole::Staff => view! {
                                                        <A href="/stations">
                                                            <Card variant=CardVariant::Default>
                                                                <div class="text-2xl mb-2">"🍽️"</div>
                                                                <Text variant=TextVariant::Body size=Size::Md weight=FontWeight::Semibold class="mb-1">
                                                                    "Stations"
                                                                </Text>
                                                                <Text variant=TextVariant::Caption size=Size::Sm intent=Intent::Secondary>
                                                                    "View preparation stations"
                                                                </Text>
                                                            </Card>
                                                        </A>
                                                    }.into_any(),
                                                }
                                            }}
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        } else {
                            view! {
                                <div class="space-y-6 w-full max-w-md text-center">
                                    <div class="bg-blue-50 dark:bg-blue-900/20 p-6 rounded-lg border border-blue-200 dark:border-blue-800">
                                        <Text variant=TextVariant::Body class="mb-4">
                                            "Welcome to Order Stream! Please sign in to access the system."
                                        </Text>
                                        <div class="w-full">
                                            <A href="/signin">
                                                <Button intent=Intent::Primary size=Size::Lg>
                                                    "Sign In"
                                                </Button>
                                            </A>
                                        </div>
                                    </div>
                                    
                                    <div class="space-y-3">
                                        <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Semibold>
                                            "Features"
                                        </Text>
                                        <div class="space-y-2 text-left">
                                            <div class="flex items-center space-x-2">
                                                <div class="text-lg">"💰"</div>
                                                <Text variant=TextVariant::Body size=Size::Sm>
                                                    "Cashier station for taking orders"
                                                </Text>
                                            </div>
                                            <div class="flex items-center space-x-2">
                                                <div class="text-lg">"🍽️"</div>
                                                <Text variant=TextVariant::Body size=Size::Sm>
                                                    "Kitchen stations for order preparation"
                                                </Text>
                                            </div>
                                            <div class="flex items-center space-x-2">
                                                <div class="text-lg">"🔧"</div>
                                                <Text variant=TextVariant::Body size=Size::Sm>
                                                    "Admin panel for system management"
                                                </Text>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </main>
    }
}