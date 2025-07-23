use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::backend::user::Login;
use crate::app::states::user;
use crate::app::components::atoms::icons;

#[component]
pub fn SignIn() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();

    // Handle successful login
    Effect::new(move |_| {
        if let Some(Ok(user)) = login_action.value().get() {
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
                        "Sign in to your account"
                    </h2>
                    <p class="mt-2 text-center text-sm text-text-muted">
                        "Welcome back to Order Stream"
                    </p>
                </div>
                
                <ActionForm 
                    action=login_action
                    attr:class="mt-8 space-y-6"
                >
                    <div class="space-y-4">
                        <div>
                            <label for="email" class="sr-only">"Email address"</label>
                            <input
                                id="email"
                                name="email"
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
                                name="password"
                                type="password"
                                autocomplete="current-password"
                                required
                                class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                                placeholder="Password"
                            />
                        </div>
                    </div>

                    <Show when=move || login_action.value().get().as_ref().map(|result| result.is_err()).unwrap_or(false)>
                        <div class="bg-error/10 border border-error/20 rounded-md p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-error">
                                        "Invalid email or password"
                                    </h3>
                                </div>
                            </div>
                        </div>
                    </Show>

                    <div>
                        <button
                            type="submit"
                            disabled=move || login_action.pending().get()
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <Show
                                when=move || login_action.pending().get()
                                fallback=|| view! { "Sign in" }
                            >
                                <span class="flex items-center">
                                    <icons::Spinner attr:class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"/>
                                    "Signing in..."
                                </span>
                            </Show>
                        </button>
                    </div>

                    <div class="text-center">
                        <p class="text-sm text-text-muted">
                            "Don't have an account? "
                            <a href="/signup" class="font-medium text-primary hover:opacity-80">
                                "Sign up"
                            </a>
                        </p>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}