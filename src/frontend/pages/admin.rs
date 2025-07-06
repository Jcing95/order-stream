use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::frontend::design_system::{
    Text, Button, Card, Input, Alert,
    TextVariant, FontWeight,
    theme::{Size, Intent, ComponentState},
    atoms::InputType,
};
use crate::frontend::state::{admin::AdminState, theme::{ThemeState}};
use crate::frontend::state::auth::use_auth_context;
use crate::common::types::UserSecurityInfo;
use crate::backend::services::auth::{
    get_user_security_info, admin_lock_user_account, unlock_user_account, revoke_user_sessions
};

#[component]
pub fn AdminPage() -> impl IntoView {
    let _state = AdminState::new();
    let _theme_state = expect_context::<ThemeState>();
    let auth = use_auth_context();
    let user = auth.user();
    
    // User management state
    let (show_user_management, set_show_user_management) = signal(false);
    let selected_user_email = RwSignal::new(String::new());
    let (user_info, set_user_info) = signal(Option::<UserSecurityInfo>::None);
    let (loading_user_info, set_loading_user_info) = signal(false);
    let (user_action_loading, set_user_action_loading) = signal(false);
    let (action_message, set_action_message) = signal(Option::<String>::None);

    // Since we're now protected by route guards, we can assume the user is authenticated and has admin access
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
                                    {format!("Welcome, {} ({})", user.email, format!("{:?}", user.role))}
                                </Text>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </Card>

                // Quick Actions
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card class="p-6">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "Menu Management"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Manage items, categories, and pricing"
                        </Text>
                        <Button 
                            intent=Intent::Primary 
                            size=Size::Sm
                            on_click=Callback::new(move |_| {
                                leptos::logging::log!("Navigate to menu management - not yet implemented");
                            })
                        >
                            "Manage Menu"
                        </Button>
                    </Card>

                    <Card class="p-6">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "User Management"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Manage staff accounts and permissions"
                        </Text>
                        <Button 
                            intent=Intent::Secondary 
                            size=Size::Sm
                            on_click=Callback::new(move |_| {
                                set_show_user_management.set(!show_user_management.get());
                                set_action_message.set(None);
                            })
                        >
                            {move || if show_user_management.get() { "Close User Management" } else { "Manage Users" }}
                        </Button>
                    </Card>

                    <Card class="p-6">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "Station Setup"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Configure stations and workflows"
                        </Text>
                        <Button 
                            intent=Intent::Secondary 
                            size=Size::Sm
                            on_click=Callback::new(move |_| {
                                leptos::logging::log!("Navigate to station setup - not yet implemented");
                            })
                        >
                            "Setup Stations"
                        </Button>
                    </Card>

                    <Card class="p-6">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "Order Analytics"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "View sales reports and analytics"
                        </Text>
                        <Button 
                            intent=Intent::Secondary 
                            size=Size::Sm
                            on_click=Callback::new(move |_| {
                                leptos::logging::log!("Navigate to analytics - not yet implemented");
                            })
                        >
                            "View Reports"
                        </Button>
                    </Card>

                    <Card class="p-6">
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                            "System Settings"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary class="mb-4">
                            "Configure system preferences"
                        </Text>
                        <Button 
                            intent=Intent::Secondary 
                            size=Size::Sm
                            on_click=Callback::new(move |_| {
                                leptos::logging::log!("Navigate to system settings - not yet implemented");
                            })
                        >
                            "Settings"
                        </Button>
                    </Card>
                </div>

                // User Management Interface (conditionally shown)
                {move || {
                    if show_user_management.get() {
                        view! {
                            <Card class="p-6">
                                <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-4">
                                    "User Management"
                                </Text>
                                
                                // User lookup section
                                <div class="space-y-4 mb-6">
                                    <div class="space-y-2">
                                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium class="block">
                                            "User Email"
                                            <span class="text-red-500 ml-1">"*"</span>
                                        </Text>
                                        <Text variant=TextVariant::Caption size=Size::Xs intent=Intent::Secondary>
                                            "Enter the email address of the user to manage"
                                        </Text>
                                        <Input
                                            input_type=InputType::Email
                                            value=selected_user_email
                                            placeholder="user@example.com"
                                            size=Size::Md
                                            intent=Intent::Primary
                                            state=if loading_user_info.get() || user_action_loading.get() { ComponentState::Disabled } else { ComponentState::Enabled }
                                            required=true
                                        />
                                    </div>
                                    
                                    <Button 
                                        intent=Intent::Primary 
                                        size=Size::Md
                                        state=if loading_user_info.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                        on_click=Callback::new(move |_| {
                                            let email = selected_user_email.get().trim().to_string();
                                            if !email.is_empty() {
                                                set_loading_user_info.set(true);
                                                set_action_message.set(None);
                                                spawn_local(async move {
                                                    match get_user_security_info(email).await {
                                                        Ok(info) => {
                                                            set_user_info.set(Some(info));
                                                        },
                                                        Err(e) => {
                                                            set_action_message.set(Some(format!("Error: {}", e)));
                                                        }
                                                    }
                                                    set_loading_user_info.set(false);
                                                });
                                            }
                                        })
                                    >
                                        {move || if loading_user_info.get() { "Searching..." } else { "Lookup User" }}
                                    </Button>
                                </div>
                                
                                // Action message display
                                {move || {
                                    action_message.get().map(|msg| {
                                        let is_error = msg.starts_with("Error:");
                                        let intent = if is_error { Intent::Danger } else { Intent::Success };
                                        view! {
                                            <Alert intent=intent size=Size::Md class="mb-4">
                                                {msg}
                                            </Alert>
                                        }
                                    })
                                }}
                                
                                // User info display and controls
                                {move || {
                                    user_info.get().map(|info| {
                                        let email = info.email.clone();
                                        let email_unlock = info.email.clone();
                                        let email_lock = info.email.clone();
                                        let email_revoke = info.email.clone();
                                        let is_locked = info.locked_until.is_some();
                                        let is_active = info.active;
                                        let sessions_count = info.active_sessions_count;
                                        let failed_attempts = info.recent_failed_attempts_count;
                                        
                                        view! {
                                            <div class="space-y-6">
                                                // User information
                                                <Card class="p-4">
                                                    <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Semibold class="mb-3">
                                                        "User Information"
                                                    </Text>
                                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                                        <div>
                                                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                                "Email"
                                                            </Text>
                                                            <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary>
                                                                {email}
                                                            </Text>
                                                        </div>
                                                        <div>
                                                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                                "Account Status"
                                                            </Text>
                                                            <Text variant=TextVariant::Body size=Size::Sm intent=if is_active { Intent::Success } else { Intent::Danger }>
                                                                {if is_active { "Active" } else { "Inactive" }}
                                                            </Text>
                                                        </div>
                                                        <div>
                                                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                                "Active Sessions"
                                                            </Text>
                                                            <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary>
                                                                {sessions_count.to_string()}
                                                            </Text>
                                                        </div>
                                                        <div>
                                                            <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                                                                "Recent Failed Attempts (24h)"
                                                            </Text>
                                                            <Text variant=TextVariant::Body size=Size::Sm intent=Intent::Secondary>
                                                                {failed_attempts.to_string()}
                                                            </Text>
                                                        </div>
                                                    </div>
                                                    {move || {
                                                        if is_locked {
                                                            view! {
                                                                <Alert intent=Intent::Danger size=Size::Sm class="mt-3">
                                                                    "⚠️ Account is currently locked"
                                                                </Alert>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }
                                                    }}
                                                </Card>
                                                
                                                // Admin actions
                                                <Card class="p-4">
                                                    <Alert intent=Intent::Warning size=Size::Sm class="mb-4">
                                                        "⚠️ Use these actions carefully. Account locks last 24 hours and session revocation will force immediate logout."
                                                    </Alert>
                                                    <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Semibold class="mb-3">
                                                        "Admin Actions"
                                                    </Text>
                                                    <div class="flex flex-wrap gap-3">
                                                        // Lock/Unlock account
                                                        {move || {
                                                            if is_locked {
                                                                view! {
                                                                    <Button 
                                                                        intent=Intent::Success 
                                                                        size=Size::Sm
                                                                        state=if user_action_loading.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                                                        on_click={
                                                                            let email = email_unlock.clone();
                                                                            Callback::new(move |_| {
                                                                                let email = email.clone();
                                                                                set_user_action_loading.set(true);
                                                                                set_action_message.set(None);
                                                                                spawn_local(async move {
                                                                                    match unlock_user_account(email.clone()).await {
                                                                                        Ok(_) => {
                                                                                            set_action_message.set(Some("Account unlocked successfully".to_string()));
                                                                                            // Refresh user info
                                                                                            if let Ok(updated_info) = get_user_security_info(email).await {
                                                                                                set_user_info.set(Some(updated_info));
                                                                                            }
                                                                                        },
                                                                                        Err(e) => {
                                                                                            set_action_message.set(Some(format!("Error unlocking account: {}", e)));
                                                                                        }
                                                                                    }
                                                                                    set_user_action_loading.set(false);
                                                                                });
                                                                            })
                                                                        }
                                                                    >
                                                                        "🔓 Unlock Account"
                                                                    </Button>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <Button 
                                                                        intent=Intent::Warning 
                                                                        size=Size::Sm
                                                                        state=if user_action_loading.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                                                        on_click={
                                                                            let email = email_lock.clone();
                                                                            Callback::new(move |_| {
                                                                                let email = email.clone();
                                                                                set_user_action_loading.set(true);
                                                                                set_action_message.set(None);
                                                                                spawn_local(async move {
                                                                                    match admin_lock_user_account(email.clone(), 24).await { // Lock for 24 hours
                                                                                        Ok(_) => {
                                                                                            set_action_message.set(Some("Account locked for 24 hours".to_string()));
                                                                                            // Refresh user info
                                                                                            if let Ok(updated_info) = get_user_security_info(email).await {
                                                                                                set_user_info.set(Some(updated_info));
                                                                                            }
                                                                                        },
                                                                                        Err(e) => {
                                                                                            set_action_message.set(Some(format!("Error locking account: {}", e)));
                                                                                        }
                                                                                    }
                                                                                    set_user_action_loading.set(false);
                                                                                });
                                                                            })
                                                                        }
                                                                    >
                                                                        "🔒 Lock Account (24h)"
                                                                    </Button>
                                                                }.into_any()
                                                            }
                                                        }}
                                                        
                                                        // Revoke sessions
                                                        <Button 
                                                            intent=Intent::Danger 
                                                            size=Size::Sm
                                                            state=if user_action_loading.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                                            on_click={
                                                                let email = email_revoke.clone();
                                                                Callback::new(move |_| {
                                                                    let email = email.clone();
                                                                    set_user_action_loading.set(true);
                                                                    set_action_message.set(None);
                                                                    spawn_local(async move {
                                                                        match revoke_user_sessions(email.clone()).await {
                                                                            Ok(_) => {
                                                                                set_action_message.set(Some("All user sessions revoked".to_string()));
                                                                                // Refresh user info
                                                                                if let Ok(updated_info) = get_user_security_info(email).await {
                                                                                    set_user_info.set(Some(updated_info));
                                                                                }
                                                                            },
                                                                            Err(e) => {
                                                                                set_action_message.set(Some(format!("Error revoking sessions: {}", e)));
                                                                            }
                                                                        }
                                                                        set_user_action_loading.set(false);
                                                                    });
                                                                })
                                                            }
                                                        >
                                                            "🚪 Revoke All Sessions"
                                                        </Button>
                                                    </div>
                                                </Card>
                                            </div>
                                        }
                                    })
                                }}
                            </Card>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}

                // Status Overview
                <Alert intent=Intent::Info size=Size::Lg class="p-6">
                    <div>
                        <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold class="mb-2">
                            "System Status"
                        </Text>
                        <Text variant=TextVariant::Body intent=Intent::Secondary>
                            "All systems operational. Authentication working correctly."
                        </Text>
                    </div>
                </Alert>
            </div>
        </div>
    }
}