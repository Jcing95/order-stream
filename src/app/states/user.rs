use leptos::prelude::*;
use crate::common::types::User;

#[cfg(feature = "hydrate")]
use crate::app::states::websocket::{self, Message};

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: RwSignal<Option<User>>,
}

impl UserState {
    pub fn new() -> Self {
        let user: RwSignal<Option<User>> = RwSignal::new(None);

        // Subscribe to WebSocket updates for current user (client-side only)
        #[cfg(feature = "hydrate")]
        {
            Effect::new({
                let user = user;
                move |_| {
                    let ws_state = websocket::get();
                    let users_signal = ws_state.users;
                    
                    // Handle incoming WebSocket messages for users
                    Effect::new(move |_| {
                        if let Some(msg) = users_signal.get() {
                            match msg {
                                Message::Update(updated_user) => {
                                    // Only update if it's the current user
                                    if let Some(current_user) = user.get_untracked() {
                                        if current_user.id == updated_user.id {
                                            user.set(Some(updated_user));
                                        }
                                    }
                                },
                                Message::Delete(user_id) => {
                                    // If current user is deleted, log them out
                                    if let Some(current_user) = user.get_untracked() {
                                        if current_user.id == user_id {
                                            user.set(None);
                                        }
                                    }
                                },
                                Message::Add(_) => {
                                    // New users don't affect current user state
                                }
                            }
                        }
                    });
                }
            });
        }

        Self { user }
    }
}

pub fn provide() -> UserState {
    let user_state = UserState::new();
    provide_context(user_state.clone());
    user_state
}

pub fn get() -> UserState {
    expect_context::<UserState>()
}