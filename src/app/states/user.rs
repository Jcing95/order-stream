use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{User, Role};
use crate::backend::user::get_all_users;
use leptos::logging::log;

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: RwSignal<Option<User>>,
    pub loading: RwSignal<bool>,
    pub users: ReadSignal<Vec<User>>,
    pub set_users: WriteSignal<Vec<User>>,
}

impl UserState {
    pub fn new() -> Self {
        let user: RwSignal<Option<User>> = RwSignal::new(None);
        let loading: RwSignal<bool> = RwSignal::new(true); // Start as loading
        let (users, set_users) = signal(Vec::new());
        
        // Load users once on initialization
        Effect::new({
            let set_users = set_users;
            move |_| {
                spawn_local(async move {
                    match get_all_users().await {
                        Ok(user_list) => set_users.set(user_list),
                        Err(_) => {}, // Keep empty vec on error
                    }
                });
            }
        });
        
        Self { user, loading, users, set_users }
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

    pub fn get_users(&self) -> ReadSignal<Vec<User>> {
        self.users
    }

    pub fn update_user_in_list(&self, updated: User) {
        let current_users = self.users.get_untracked();
        let new_users: Vec<User> = current_users
            .iter()
            .map(|u| {
                if u.id == updated.id {
                    updated.clone()
                } else {
                    u.clone()
                }
            })
            .collect();
        self.set_users.set(new_users);
    }

    pub fn refresh_users(&self) {
        let set_users = self.set_users;
        spawn_local(async move {
            match get_all_users().await {
                Ok(user_list) => set_users.set(user_list),
                Err(_) => {}, // Keep existing users on error
            }
        });
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