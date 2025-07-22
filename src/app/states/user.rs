use leptos::prelude::*;
use crate::common::types::{User, Role};
use leptos::logging::log;

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: RwSignal<Option<User>>,
    pub loading: RwSignal<bool>,
}

impl UserState {
    pub fn new() -> Self {
        let user: RwSignal<Option<User>> = RwSignal::new(None);
        let loading: RwSignal<bool> = RwSignal::new(true); // Start as loading
        Self { user, loading }
    }

    pub fn set_loading(&self, loading: bool) {
        self.loading.set(loading);
    }

    pub fn is_loading(&self) -> bool {
        self.loading.get()
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.get().is_some()
    }

    pub fn has_role(&self, role: Role) -> bool {
        if let Some(user) = self.user.get() {
            user.role == role || user.role == Role::Admin
        } else {
            false
        }
    }

    pub fn has_any_role(&self, roles: &[Role]) -> bool {
        log!("Checking user {:?} for roles {:?}", self.user.get(), roles);
        if let Some(user) = self.user.get() {
            // Admin always has access
            if user.role == Role::Admin {
                return true;
            }
            roles.contains(&user.role)
        } else {
            false
        }
    }

    pub fn can_access_user_resource(&self, user_id: &str) -> bool {
        if let Some(user) = self.user.get() {
            // Admin can access any user's resources
            user.role == Role::Admin || user.id == user_id
        } else {
            false
        }
    }

    pub fn set_user(&self, user: User) {
        self.user.set(Some(user));
    }

    pub fn logout(&self) {
        self.user.set(None);
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