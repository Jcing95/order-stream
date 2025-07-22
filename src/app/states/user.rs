use leptos::prelude::*;
use crate::common::types::User;

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: RwSignal<Option<User>>,
}

impl UserState {
    pub fn new() -> Self {
        let user: RwSignal<Option<User>> = RwSignal::new(None);
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