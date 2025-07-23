use leptos::prelude::*;
use super::logout_button::LogoutButton;

#[component]
pub fn UserDropdown(
    user_email: String,
    user_initial: String,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

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
                        <LogoutButton 
                            class="block w-full text-left px-4 py-2 text-sm text-text hover:bg-surface-elevated hover:text-primary transition-colors"
                            on_click=Box::new(move || set_is_open.set(false))
                        />
                    </div>
                </div>
            </Show>
        </div>
    }
}