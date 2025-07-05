use leptos::prelude::*;
use crate::frontend::design_system::{
    Text, Button,
    TextVariant, FontWeight,
    theme::{Size, Intent},
};
use crate::frontend::state::{admin::AdminState, theme::{ThemeState}};
use crate::frontend::state::auth::use_auth_context;

#[component]
pub fn AdminPage() -> impl IntoView {
    let _state = AdminState::new();
    let _theme_state = expect_context::<ThemeState>();
    let auth = use_auth_context();
    let user = auth.user();

    // Since we're now protected by route guards, we can assume the user is authenticated and has admin access
    view! {
        <div class="container mx-auto p-6">
            <div class="space-y-8">
                // Header
                <div class="border-b border-gray-200 dark:border-gray-700 pb-6">
                    <Text variant=TextVariant::Heading size=Size::Xl weight=FontWeight::Bold>
                        "Admin Dashboard"
                    </Text>
                    {move || {
                        if let Some(user) = user.get() {
                            view! {
                                <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary class="mt-2">
                                    {format!("Welcome, {} ({})", user.email, format!("{:?}", user.role))}
                                </Text>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>

                // Quick Actions
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "Menu Management"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Manage items, categories, and pricing"
                        </Text>
                        <Button intent=Intent::Primary size=Size::Sm>
                            "Manage Menu"
                        </Button>
                    </div>

                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "User Management"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Manage staff accounts and permissions"
                        </Text>
                        <Button intent=Intent::Secondary size=Size::Sm>
                            "Manage Users"
                        </Button>
                    </div>

                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "Station Setup"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Configure stations and workflows"
                        </Text>
                        <Button intent=Intent::Secondary size=Size::Sm>
                            "Setup Stations"
                        </Button>
                    </div>

                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "Order Analytics"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "View sales reports and analytics"
                        </Text>
                        <Button intent=Intent::Secondary size=Size::Sm>
                            "View Reports"
                        </Button>
                    </div>

                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg border border-gray-200 dark:border-gray-700">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "System Settings"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Configure system preferences"
                        </Text>
                        <Button intent=Intent::Secondary size=Size::Sm>
                            "Settings"
                        </Button>
                    </div>
                </div>

                // Status Overview
                <div class="bg-blue-50 dark:bg-blue-900/20 p-6 rounded-lg border border-blue-200 dark:border-blue-800">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-2">
                        "System Status"
                    </Text>
                    <Text variant=TextVariant::Body intent=Intent::Secondary>
                        "All systems operational. Authentication working correctly."
                    </Text>
                </div>
            </div>
        </div>
    }
}