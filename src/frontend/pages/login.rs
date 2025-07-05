use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{LoginRequest, RegisterRequest, UserRole};
use crate::backend::services::auth::{login_user, register_user};
use crate::frontend::state::auth::use_auth_context;
use crate::frontend::design_system::{
    atoms::TextVariant,
    theme::{Intent, Size},
    Text, Card,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum AuthMode {
    Login,
    Register,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth_context = use_auth_context();
    let (auth_mode, set_auth_mode) = signal(AuthMode::Login);
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (role, set_role) = signal(UserRole::Staff);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);

    let navigate = leptos_router::hooks::use_navigate();
    
    let handle_submit = {
        let auth_context = auth_context.clone();
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            
            set_is_loading.set(true);
            set_error_message.set(None);

            let email_val = email.get();
            let password_val = password.get();
            let mode = auth_mode.get();
            let role_val = role.get(); // Get the role value before the async context
            let nav = navigate.clone();
            let auth_ctx = auth_context.clone();

            spawn_local(async move {
                let result = match mode {
                    AuthMode::Login => {
                        let request = LoginRequest {
                            email: email_val,
                            password: password_val,
                        };
                        login_user(request).await
                    },
                    AuthMode::Register => {
                        let request = RegisterRequest {
                            email: email_val,
                            password: password_val,
                            role: role_val,
                        };
                        register_user(request).await
                    }
                };

                match result {
                    Ok(auth_response) => {
                        // Update auth context with the authenticated user
                        auth_ctx.set_authenticated_user(auth_response.user.clone());
                        
                        // Success! Redirect to appropriate page based on role
                        let redirect_path = match auth_response.user.role {
                            UserRole::Admin => "/admin",
                            UserRole::Cashier => "/cashier", 
                            UserRole::Staff => "/stations",
                        };
                        nav(redirect_path, Default::default());
                    },
                    Err(e) => {
                        set_error_message.set(Some(e.to_string()));
                    }
                }
                
                set_is_loading.set(false);
            });
        }
    };

    let toggle_mode = move |_| {
        match auth_mode.get() {
            AuthMode::Login => set_auth_mode.set(AuthMode::Register),
            AuthMode::Register => set_auth_mode.set(AuthMode::Login),
        }
        set_error_message.set(None);
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div class="text-center">
                    <Text variant=TextVariant::Heading size=Size::Xl>
                        "Order Stream"
                    </Text>
                    <Text variant=TextVariant::Body intent=Intent::Secondary class="mt-2">
                        {move || match auth_mode.get() {
                            AuthMode::Login => "Sign in to your account",
                            AuthMode::Register => "Create a new account",
                        }}
                    </Text>
                </div>

                <Card>
                    <form on:submit=handle_submit class="space-y-6">
                        {move || {
                            error_message.get().map(|msg| {
                                view! {
                                    <div class="bg-red-50 dark:bg-red-900/10 border border-red-200 dark:border-red-800 rounded-md p-4">
                                        <Text variant=TextVariant::Body intent=Intent::Danger class="text-sm">
                                            {msg}
                                        </Text>
                                    </div>
                                }
                            })
                        }}

                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-200">
                                "Email address"
                            </label>
                            <input
                                type="email"
                                class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                                placeholder="Enter your email"
                                required
                                prop:value=move || email.get()
                                on:input=move |e| set_email.set(event_target_value(&e))
                                prop:disabled=move || is_loading.get()
                            />
                        </div>

                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-200">
                                "Password"
                            </label>
                            <input
                                type="password"
                                class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                                placeholder="Enter your password"
                                required
                                prop:value=move || password.get()
                                on:input=move |e| set_password.set(event_target_value(&e))
                                prop:disabled=move || is_loading.get()
                            />
                        </div>

                        {move || match auth_mode.get() {
                            AuthMode::Register => {
                                view! {
                                    <div class="space-y-2">
                                        <label class="block text-sm font-medium text-gray-700 dark:text-gray-200">
                                            "Role"
                                        </label>
                                        <select 
                                            class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                                            on:change=move |ev| {
                                                let value = event_target_value(&ev);
                                                let selected_role = match value.as_str() {
                                                    "Admin" => UserRole::Admin,
                                                    "Cashier" => UserRole::Cashier,
                                                    _ => UserRole::Staff,
                                                };
                                                set_role.set(selected_role);
                                            }
                                        >
                                            <option value="Staff" selected=move || role.get() == UserRole::Staff>
                                                "Staff"
                                            </option>
                                            <option value="Cashier" selected=move || role.get() == UserRole::Cashier>
                                                "Cashier"
                                            </option>
                                            <option value="Admin" selected=move || role.get() == UserRole::Admin>
                                                "Admin"
                                            </option>
                                        </select>
                                    </div>
                                }.into_any()
                            },
                            AuthMode::Login => view! { <div></div> }.into_any()
                        }}

                        <button
                            type="submit"
                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                            disabled=is_loading
                        >
                            {move || if is_loading.get() {
                                match auth_mode.get() {
                                    AuthMode::Login => "Signing in...",
                                    AuthMode::Register => "Creating account...",
                                }
                            } else {
                                match auth_mode.get() {
                                    AuthMode::Login => "Sign in",
                                    AuthMode::Register => "Create account",
                                }
                            }}
                        </button>

                        <div class="text-center">
                            <button
                                type="button"
                                class="text-sm text-blue-600 dark:text-blue-400 hover:text-blue-500 dark:hover:text-blue-300"
                                on:click=toggle_mode
                            >
                                {move || match auth_mode.get() {
                                    AuthMode::Login => "Need an account? Sign up",
                                    AuthMode::Register => "Already have an account? Sign in",
                                }}
                            </button>
                        </div>
                    </form>
                </Card>
            </div>
        </div>
    }
}