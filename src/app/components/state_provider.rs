use crate::app::states;
use leptos::prelude::*;

#[component]
pub fn StateProvider<F>(children: F) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    // Provide all states within the reactive context
    states::websocket::provide();
    states::category::provide();
    states::event::provide();
    states::settings::provide();
    states::user::provide();
    states::product::provide();
    states::station::provide();
    states::order::provide();
    
    #[cfg(feature = "hydrate")]
    {
        let user_state = states::user::get();
        // Only initialize user on client-side to avoid threading issues
        use crate::backend::user::get_current_user;
        use leptos::logging::log;

        // Initialize user state by fetching current user from session
        let user_resource = Resource::new_blocking(
            || (), // No dependencies
            |_| async move {
                log!("Fetching current user...");
                get_current_user().await
            },
        );

        // Update user state when resource loads
        Effect::new(move |_| {
            if let Some(result) = user_resource.get() {
                match result {
                    Ok(current_user) => {
                        log!("Found authenticated user: {:?}", current_user);
                        user_state.set_user(current_user);
                    }
                    Err(_) => {
                        log!("No authenticated user found");
                        // Keep user as None
                    }
                }
                // Either way, we're done loading
                user_state.set_loading(false);
            }
        });
    }

    children()
}
