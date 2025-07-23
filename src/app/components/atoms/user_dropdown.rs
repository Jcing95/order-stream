use crate::backend::user::logout;
use crate::app::states::user;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos::task::spawn_local;
use leptos::logging::error;

#[component]
pub fn UserDropdown(
    user_email: String,
    user_initial: String,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let navigate = use_navigate();
    let user_state = user::get();

    view! {
        <div class="relative">
            <div 
                class="flex items-center space-x-2 cursor-pointer"
                on:click=move |_| set_is_open.update(|open| *open = !*open)
            >
                <span class="text-sm text-text">
                    {user_email.clone()}
                </span>
                <div class="w-8 h-8 bg-primary rounded-full flex items-center justify-center">
                    <span class="text-surface text-sm font-medium">
                        {user_initial}
                    </span>
                </div>
            </div>
            
            <Show when=move || is_open.get()>
                <div class="absolute right-0 mt-1 w-48 bg-surface border border-border rounded-md shadow-lg z-50">
                    <div class="py-1">
                        <button
                            on:click={
                                let navigate = navigate.clone();
                                let user_state = user_state.clone();
                                move |_| {
                                    set_is_open.set(false);
                                    let navigate = navigate.clone();
                                    let user_state = user_state.clone();
                                    spawn_local(async move {
                                        match logout().await {
                                            Ok(_) => {
                                                // Update client-side user state
                                                user_state.logout();
                                                navigate("/signin", Default::default());
                                            }
                                            Err(e) => {
                                                error!("Logout failed: {:?}", e);
                                            }
                                        }
                                    });
                                }
                            }
                            class="block w-full text-left px-4 py-2 text-sm text-text hover:bg-surface-elevated hover:text-primary transition-colors"
                        >
                            Logout
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}