use leptos::prelude::*;
use crate::common::types::{User, UserRole};
use crate::backend::services::auth::logout_user;

#[cfg(feature = "hydrate")]
use crate::backend::services::auth::get_current_user;

#[derive(Clone, Debug)]
pub struct AuthState {
    user: RwSignal<Option<User>>,
    is_loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(None),
            is_loading: RwSignal::new(true),
            error: RwSignal::new(None),
        }
    }

    // Read-only access to auth state
    pub fn user(&self) -> ReadSignal<Option<User>> {
        self.user.read_only()
    }

    pub fn is_loading(&self) -> ReadSignal<bool> {
        self.is_loading.read_only()
    }

    pub fn error(&self) -> ReadSignal<Option<String>> {
        self.error.read_only()
    }

    // Computed signals
    pub fn is_authenticated(&self) -> Signal<bool> {
        let user = self.user;
        Signal::derive(move || user.get().is_some())
    }

    pub fn user_role(&self) -> Signal<Option<UserRole>> {
        let user = self.user;
        Signal::derive(move || user.get().map(|u| u.role))
    }

    // Internal state setters
    fn set_user(&self, user: Option<User>) {
        self.user.set(user);
        self.error.set(None); // Clear errors on successful auth state change
    }

    fn set_loading(&self, loading: bool) {
        self.is_loading.set(loading);
    }

    fn set_error(&self, error: Option<String>) {
        self.error.set(error);
    }

    // Public auth operations
    pub async fn initialize(&self) {
        // Only initialize on the client side to avoid hydration mismatches
        #[cfg(feature = "hydrate")]
        {
            self.set_loading(true);
            self.set_error(None);
            
            match get_current_user().await {
                Ok(user) => {
                    self.set_user(user);
                },
                Err(e) => {
                    self.set_user(None);
                    // Don't set error for initial load failures (user might not be logged in)
                    leptos::logging::log!("Auth initialization: {}", e);
                }
            }
            
            self.set_loading(false);
        }
        
        // On SSR, just set loading to false and don't make any server calls
        #[cfg(feature = "ssr")]
        {
            self.set_loading(false);
            self.set_user(None);
        }
    }

    pub async fn logout(&self) {
        self.set_loading(true);
        
        match logout_user().await {
            Ok(_) => {
                self.set_user(None);
                // Redirect to home page after logout
                #[cfg(feature = "hydrate")]
                {
                    if let Some(window) = web_sys::window() {
                        _ = window.location().set_href("/");
                    }
                }
            },
            Err(e) => {
                self.set_error(Some(format!("Logout failed: {}", e)));
                // Force local logout even if server logout fails
                self.set_user(None);
                // Still redirect to home page
                #[cfg(feature = "hydrate")]
                {
                    if let Some(window) = web_sys::window() {
                        _ = window.location().set_href("/");
                    }
                }
            }
        }
        
        self.set_loading(false);
    }

    // Set user after successful login (called from login page)
    pub fn set_authenticated_user(&self, user: User) {
        self.set_user(Some(user));
    }

    // Role-based authorization helpers
    pub fn can_access_admin(&self) -> Signal<bool> {
        let user = self.user;
        Signal::derive(move || {
            matches!(user.get().map(|u| u.role), Some(UserRole::Admin))
        })
    }

    pub fn can_access_cashier(&self) -> Signal<bool> {
        let user = self.user;
        Signal::derive(move || {
            matches!(user.get().map(|u| u.role), Some(UserRole::Admin | UserRole::Cashier))
        })
    }

    pub fn can_access_stations(&self) -> Signal<bool> {
        let user = self.user;
        Signal::derive(move || {
            user.get().is_some() // All authenticated users can access stations
        })
    }
}

// Context provider/consumer
pub fn provide_auth_context() -> AuthState {
    let auth_state = AuthState::new();
    provide_context(auth_state.clone());
    auth_state
}

pub fn use_auth_context() -> AuthState {
    expect_context::<AuthState>()
}

// Convenience hook for common auth patterns
pub fn use_auth() -> (ReadSignal<Option<User>>, Signal<bool>, ReadSignal<bool>) {
    let auth = use_auth_context();
    (auth.user(), auth.is_authenticated(), auth.is_loading())
}

// Hook for role-based access control
pub fn use_auth_permissions() -> (Signal<bool>, Signal<bool>, Signal<bool>) {
    let auth = use_auth_context();
    (auth.can_access_admin(), auth.can_access_cashier(), auth.can_access_stations())
}