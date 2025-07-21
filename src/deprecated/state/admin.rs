use leptos::prelude::*;
use crate::common::types::UserSecurityInfo;
use crate::backend::services::auth::{
    get_user_security_info, admin_lock_user_account, unlock_user_account, revoke_user_sessions
};

/// AdminState with proper Actions pattern for user management
#[derive(Clone)]
pub struct AdminState {
    // User input signals
    pub selected_user_email: RwSignal<String>,
    pub action_message: RwSignal<Option<String>>,
    pub show_user_management: RwSignal<bool>,
    pub current_user_info: RwSignal<Option<UserSecurityInfo>>,
    
    // Actions for admin operations
    pub lookup_user_action: Action<String, Result<UserSecurityInfo, String>>,
    pub lock_user_action: Action<(String, u32), Result<(), String>>,
    pub unlock_user_action: Action<String, Result<(), String>>,
    pub revoke_sessions_action: Action<String, Result<(), String>>,
    pub reset_action: Action<(), ()>,
}

impl AdminState {
    pub fn new() -> Self {
        // User input signals
        let selected_user_email = RwSignal::new(String::new());
        let action_message = RwSignal::new(None);
        let show_user_management = RwSignal::new(false);
        let current_user_info = RwSignal::new(None);
        
        // Lookup user action
        let lookup_user_action = Action::new(move |email: &String| {
            let email = email.clone();
            async move {
                match get_user_security_info(email).await {
                    Ok(info) => {
                        current_user_info.set(Some(info.clone()));
                        Ok(info)
                    }
                    Err(e) => {
                        let error_msg = format!("Error: {}", e);
                        Err(error_msg)
                    }
                }
            }
        });
        
        // Lock user action
        let lock_user_action = Action::new(move |(email, hours): &(String, u32)| {
            let email = email.clone();
            let hours = *hours;
            async move {
                match admin_lock_user_account(email, hours).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Error: {}", e))
                }
            }
        });
        
        // Unlock user action
        let unlock_user_action = Action::new(move |email: &String| {
            let email = email.clone();
            async move {
                match unlock_user_account(email).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Error: {}", e))
                }
            }
        });
        
        // Revoke sessions action
        let revoke_sessions_action = Action::new(move |email: &String| {
            let email = email.clone();
            async move {
                match revoke_user_sessions(email).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Error: {}", e))
                }
            }
        });
        
        // Simple demonstration action
        let reset_action = Action::new(move |_: &()| async move {
            leptos::logging::log!("Admin reset action executed");
        });
        
        Self {
            selected_user_email,
            action_message,
            show_user_management,
            current_user_info,
            lookup_user_action,
            lock_user_action,
            unlock_user_action,
            revoke_sessions_action,
            reset_action,
        }
    }
    
    // Helper methods for easier usage
    pub fn lookup_user(&self, email: String) {
        self.lookup_user_action.dispatch(email);
    }
    
    pub fn lock_user(&self, email: String, hours: u32) {
        self.lock_user_action.dispatch((email, hours));
    }
    
    pub fn unlock_user(&self, email: String) {
        self.unlock_user_action.dispatch(email);
    }
    
    pub fn revoke_user_sessions(&self, email: String) {
        self.revoke_sessions_action.dispatch(email);
    }
    
    // Derived signals for loading states
    pub fn is_loading(&self) -> Signal<bool> {
        let lookup_pending = self.lookup_user_action.pending();
        let lock_pending = self.lock_user_action.pending();
        let unlock_pending = self.unlock_user_action.pending();
        let revoke_pending = self.revoke_sessions_action.pending();
        
        Signal::derive(move || {
            lookup_pending.get() ||
            lock_pending.get() ||
            unlock_pending.get() ||
            revoke_pending.get()
        })
    }
    
    // Derived signal for current user info
    pub fn current_user_info(&self) -> Signal<Option<UserSecurityInfo>> {
        let current_user_info = self.current_user_info;
        Signal::derive(move || current_user_info.get())
    }
}

// Context provider for AdminState  
pub fn provide_admin_state() -> AdminState {
    let admin_state = AdminState::new();
    provide_context(admin_state.clone());
    admin_state
}

pub fn use_admin_state() -> AdminState {
    expect_context::<AdminState>()
}