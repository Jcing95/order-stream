use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::backend::user::Login;
use crate::app::states::user;

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
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Sign in to your account"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
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
                                class="relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
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
                                class="relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Password"
                            />
                        </div>
                    </div>

                    <Show when=move || login_action.value().get().as_ref().map(|result| result.is_err()).unwrap_or(false)>
                        <div class="bg-red-50 border border-red-200 rounded-md p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-red-800">
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
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <Show
                                when=move || login_action.pending().get()
                                fallback=|| view! { "Sign in" }
                            >
                                <span class="flex items-center">
                                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    "Signing in..."
                                </span>
                            </Show>
                        </button>
                    </div>

                    <div class="text-center">
                        <p class="text-sm text-gray-600">
                            "Don't have an account? "
                            <a href="/signup" class="font-medium text-indigo-600 hover:text-indigo-500">
                                "Sign up"
                            </a>
                        </p>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}