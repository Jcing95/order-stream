use crate::backend::user::logout;
use crate::app::states::user;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos::task::spawn_local;
use leptos::logging::error;

#[component]
pub fn LogoutButton(
    #[prop(into)] class: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let user_state = user::get();

    view! {
        <button
            class=class
            on:click={
                let navigate = navigate.clone();
                let user_state = user_state.clone();
                move |_| {
                    // Call optional callback first (e.g., to close mobile menu)
                    if let Some(callback) = &on_click {
                        callback();
                    }
                    
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
        >
            "Logout"
        </button>
    }
}