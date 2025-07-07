use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::common::types::{User, UserRole};
use crate::frontend::state::auth::use_auth_context;
use crate::frontend::design_system::{Spinner, theme::Size};

/// Route protection requirements
#[derive(Clone, Debug)]
pub enum RouteRequirement {
    /// User must be authenticated (any role)
    Authenticated,
    /// User must have specific role
    Role(UserRole),
    /// User must have one of these roles
    AnyRole(Vec<UserRole>),
    /// User must NOT be authenticated (login page)
    NotAuthenticated,
}

impl RouteRequirement {
    /// Check if user meets the requirement
    fn check(&self, user: Option<&User>) -> bool {
        match self {
            RouteRequirement::Authenticated => user.is_some(),
            RouteRequirement::Role(required_role) => {
                user.map(|u| &u.role == required_role).unwrap_or(false)
            }
            RouteRequirement::AnyRole(roles) => {
                user.map(|u| roles.contains(&u.role)).unwrap_or(false)
            }
            RouteRequirement::NotAuthenticated => user.is_none(),
        }
    }

    /// Get redirect path when requirement is not met
    fn redirect_path(&self, user: Option<&User>) -> &'static str {
        match self {
            RouteRequirement::Authenticated => "/signin",
            RouteRequirement::Role(_) | RouteRequirement::AnyRole(_) => {
                // If user exists but wrong role, redirect to appropriate page
                if let Some(user) = user {
                    match user.role {
                        UserRole::Admin => "/admin",
                        UserRole::Cashier => "/cashier", 
                        UserRole::Staff => "/stations",
                    }
                } else {
                    "/signin"
                }
            }
            RouteRequirement::NotAuthenticated => {
                // Redirect authenticated users to their role-appropriate page
                if let Some(user) = user {
                    match user.role {
                        UserRole::Admin => "/admin",
                        UserRole::Cashier => "/cashier",
                        UserRole::Staff => "/stations",
                    }
                } else {
                    "/"
                }
            }
        }
    }
}

/// Reusable route protection component
/// 
/// This component handles all common route protection patterns:
/// - Authentication checks
/// - Role-based access control  
/// - Loading states
/// - Automatic redirects
/// - Proper SSR/hydration handling
#[component]
pub fn RouteGuard<F>(
    /// Route access requirement
    requirement: RouteRequirement,
    /// Children function to render when access is granted
    children: F,
) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    let auth = use_auth_context();
    let navigate = use_navigate();
    let user = auth.user();
    let is_loading = auth.is_loading();

    // Handle redirects when access requirements are not met
    Effect::new_isomorphic({
        let navigate = navigate.clone();
        let requirement = requirement.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            if !is_loading.get() {
                let current_user = user.get();
                if !requirement.check(current_user.as_ref()) {
                    let redirect_path = requirement.redirect_path(current_user.as_ref());
                    navigate(redirect_path, Default::default());
                }
            }
        }
    });

    view! {
        {move || {
            if is_loading.get() {
                // Show loading spinner while auth is loading
                view! {
                    <div class="min-h-screen flex items-center justify-center">
                        <Spinner size=Size::Lg />
                    </div>
                }.into_any()
            } else {
                let current_user = user.get();
                if requirement.check(current_user.as_ref()) {
                    // Access granted - render children
                    children()
                } else {
                    // Access denied - show loading while redirect happens
                    view! {
                        <div class="min-h-screen flex items-center justify-center">
                            <Spinner size=Size::Lg />
                        </div>
                    }.into_any()
                }
            }
        }}
    }
}

/// Convenience components for common access patterns

#[component]
pub fn RequireAuth<F>(children: F) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    view! {
        <RouteGuard requirement=RouteRequirement::Authenticated children=children />
    }
}

#[component]
pub fn RequireAdmin<F>(children: F) -> impl IntoView 
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    view! {
        <RouteGuard requirement=RouteRequirement::Role(UserRole::Admin) children=children />
    }
}

#[component]
pub fn RequireCashier<F>(children: F) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    view! {
        <RouteGuard requirement=RouteRequirement::AnyRole(vec![UserRole::Admin, UserRole::Cashier]) children=children />
    }
}

#[component]
pub fn RequireGuest<F>(children: F) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    view! {
        <RouteGuard requirement=RouteRequirement::NotAuthenticated children=children />
    }
}