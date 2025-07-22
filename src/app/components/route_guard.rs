use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_router::hooks::use_navigate;
use crate::common::types::Role;
use crate::app::states::user;

#[component]
pub fn RouteGuard<F>(
    #[prop(optional)] roles: Option<Vec<Role>>,
    #[prop(optional)] user_id: Option<String>,
    children: F,
) -> impl IntoView
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    let user_state = user::get();
    #[cfg(feature = "hydrate")]
    let navigate = use_navigate();

    view! {
        {move || {
            // Show loading state while user is being fetched
            if user_state.is_loading() {
                return view! {
                    <div class="flex items-center justify-center min-h-screen">
                        <div class="text-center">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
                            <p class="text-gray-600">"Loading..."</p>
                        </div>
                    </div>
                }.into_any();
            }

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

            // Not authorized - redirect and show error
            #[cfg(feature = "hydrate")]
            navigate("/signin", Default::default());
            
            view! {
                <div class="flex items-center justify-center min-h-screen">
                    <div class="text-center">
                        <h1 class="text-xl font-semibold text-gray-900 mb-2">
                            "Access Denied"
                        </h1>
                        <p class="text-gray-600">
                            "You don't have permission to access this page."
                        </p>
                    </div>
                </div>
            }.into_any()
        }}
    }
}