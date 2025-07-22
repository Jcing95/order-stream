use leptos::prelude::*;
use crate::common::types::Role;
use crate::app::states::user;

#[component]
pub fn RoleGated<F>(
    #[prop(optional)] roles: Option<Vec<Role>>,
    #[prop(optional)] user_id: Option<String>,
    children: F,
) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    let user_state = user::get();

    view! {
        {move || {
            // If no roles specified, content is public
            if roles.is_none() && user_id.is_none() {
                return children();
            }

            // Check role-based access
            if let Some(ref required_roles) = roles {
                if user_state.has_any_role(required_roles) {
                    return children();
                }
            }

            // Check user-specific access
            if let Some(ref target_user_id) = user_id {
                if user_state.can_access_user_resource(target_user_id) {
                    return children();
                }
            }

            // Not authorized - render nothing
            view! { <></> }.into_any()
        }}
    }
}