use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::backend::user::CreateUser;
use crate::app::states::user;
use crate::app::components::atoms::icons;

#[component]
pub fn SignUp() -> impl IntoView {
    let signup_action = ServerAction::<CreateUser>::new();

    // Handle successful signup
    Effect::new(move |_| {
        if let Some(Ok(user)) = signup_action.value().get() {
            let user_state = user::get();
            user_state.user.set(Some(user));
            
            let navigate = use_navigate();
            navigate("/", Default::default());
        }
    });

    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-elevated py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-text">
                        "Create your account"
                    </h2>
                    <p class="mt-2 text-center text-sm text-text-muted">
                        "Join us and start streaming your orders"
                    </p>
                </div>
                
                <ActionForm 
                    action=signup_action
                    attr:class="mt-8 space-y-6"
                >
                    <div class="space-y-4">
                        <div>
                            <label for="email" class="sr-only">"Email address"</label>
                            <input
                                id="email"
                                name="req[email]"
                                type="email"
                                autocomplete="email"
                                required
                                class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                                placeholder="Email address"
                            />
                        </div>
                        
                        <div>
                            <label for="password" class="sr-only">"Password"</label>
                            <input
                                id="password"
                                name="req[password]"
                                type="password"
                                autocomplete="new-password"
                                required
                                class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                                placeholder="Password"
                            />
                        </div>
                        
                    </div>

                    <Show when=move || signup_action.value().get().as_ref().map(|result| result.is_err()).unwrap_or(false)>
                        <div class="bg-error/10 border border-error/20 rounded-md p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-error">
                                        {move || {
                                            signup_action.value().get()
                                                .and_then(|result| result.err())
                                                .map(|err| {
                                                    if err.to_string().contains("already exists") {
                                                        "An account with this email already exists".to_string()
                                                    } else {
                                                        "Failed to create account. Please try again.".to_string()
                                                    }
                                                })
                                                .unwrap_or_else(|| "An error occurred".to_string())
                                        }}
                                    </h3>
                                </div>
                            </div>
                        </div>
                    </Show>

                    <div>
                        <button
                            type="submit"
                            disabled=move || signup_action.pending().get()
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <Show
                                when=move || signup_action.pending().get()
                                fallback=|| view! { "Sign up" }
                            >
                                <span class="flex items-center">
                                    <icons::Spinner attr:class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"/>
                                    "Creating account..."
                                </span>
                            </Show>
                        </button>
                    </div>

                    <div class="text-center">
                        <p class="text-sm text-text-muted">
                            "Already have an account? "
                            <a href="/signin" class="font-medium text-primary hover:opacity-80">
                                "Sign in"
                            </a>
                        </p>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}
