use leptos::prelude::*;

use crate::{
    app::{
        components::atoms::icons,
        states::user,
    },
    backend::user::UpdateUser,
    common::types::Role,
};

#[component]
fn UserDisplayItem(
    user: crate::common::types::User,
    on_edit: WriteSignal<Option<String>>,
) -> impl IntoView {
    let id = user.id.clone();
    let email = user.email.clone();
    let role = user.role.clone();
    
    let role_text = match role {
        Role::Visitor => "Visitor",
        Role::Staff => "Staff", 
        Role::Cashier => "Cashier",
        Role::Admin => "Admin",
    };
    
    let role_class = match role {
        Role::Admin => "text-red-600",
        Role::Cashier => "text-blue-600",
        Role::Staff => "text-green-600",
        Role::Visitor => "text-gray-600",
    };

    view! {
        <div class="p-3 bg-surface-elevated rounded-md border border-border">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <div class="flex items-center justify-between">
                        <span class="text-text font-medium">
                            {email}
                        </span>
                        <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                    </div>
                    <div class="mt-1">
                        <span class={format!("text-xs font-medium {}", role_class)}>{role_text}</span>
                    </div>
                </div>
                
                <div class="flex items-center space-x-2 ml-4">
                    <button
                        class="bg-border/80 text-blue-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click={
                            let id = id.clone();
                            move |_| {
                                on_edit.set(Some(id.clone()));
                            }
                        }
                    >
                        <icons::Edit />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn UserEditItem(
    user: crate::common::types::User,
    on_cancel: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (edit_role, set_edit_role) = signal(user.role.clone());
    
    let update_action = ServerAction::<UpdateUser>::new();
    
    let id = user.id.clone();
    let email = user.email.clone();
    let original_role = user.role.clone();

    view! {
        <div class="p-3 bg-surface-elevated rounded-md border border-border">
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Email"</label>
                        <input
                            type="email"
                            prop:value=email.clone()
                            disabled=true
                            class="w-full px-2 py-1 border border-border bg-surface-muted text-text-muted rounded text-sm cursor-not-allowed"
                        />
                        <p class="text-xs text-text-muted mt-1">"Email cannot be modified"</p>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Role"</label>
                        <select
                            prop:value=move || match edit_role.get() {
                                Role::Visitor => "Visitor",
                                Role::Staff => "Staff",
                                Role::Cashier => "Cashier", 
                                Role::Admin => "Admin",
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let role = match val.as_str() {
                                    "Admin" => Role::Admin,
                                    "Cashier" => Role::Cashier,
                                    "Staff" => Role::Staff,
                                    "Visitor" => Role::Visitor,
                                    _ => Role::Staff,
                                };
                                set_edit_role.set(role);
                            }
                            class="w-full px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                        >
                            <option value="Visitor" selected=move || matches!(edit_role.get(), Role::Visitor)>"Visitor"</option>
                            <option value="Staff" selected=move || matches!(edit_role.get(), Role::Staff)>"Staff"</option>
                            <option value="Cashier" selected=move || matches!(edit_role.get(), Role::Cashier)>"Cashier"</option>
                            <option value="Admin" selected=move || matches!(edit_role.get(), Role::Admin)>"Admin"</option>
                        </select>
                    </div>
                </div>
                
                <div class="flex justify-end space-x-2">
                    <ActionForm 
                        action=update_action
                        on:submit=move |_| {
                            let user_state = user::get();
                            user_state.refresh_users();
                            on_cancel.set(None);
                        }
                    >
                        <input type="hidden" name="id" value={id.clone()} />
                        <input type="hidden" name="update[role]" value=move || match edit_role.get() {
                            Role::Visitor => "Visitor",
                            Role::Staff => "Staff", 
                            Role::Cashier => "Cashier",
                            Role::Admin => "Admin",
                        } />
                        <button
                            type="submit"
                            class="bg-border/80 text-green-600 hover:bg-border hover:scale-105 p-2 rounded"
                        >
                            <icons::Accept />
                        </button>
                    </ActionForm>
                    
                    <button
                        class="bg-border/80 text-gray-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click=move |_| {
                            set_edit_role.set(original_role.clone());
                            on_cancel.set(None);
                        }
                    >
                        <icons::Cancel />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Users() -> impl IntoView {
    let user_state = user::get();
    let users = user_state.get_users();
    let (editing_id, set_editing_id) = signal::<Option<String>>(None);

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <h2 class="text-xl font-semibold text-text mb-4">"Users"</h2>

            <Show
                when=move || !users.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-8">
                        <p class="text-text-muted">"No users found"</p>
                    </div>
                }
            >
                <div class="space-y-2">
                    <For
                        each=move || users.get()
                        key=|user| user.id.clone()
                        children=move |user| {
                            let user_id = user.id.clone();
                            let user_id_for_editing = user_id.clone();
                            let user_id_for_display = user_id.clone();
                            let user_id_for_edit = user_id.clone();
                            let user_fallback = user.clone();
                            let user_edit = user.clone();
                            
                            let is_editing = move || editing_id.get() == Some(user_id_for_editing.clone());
                            
                            view! {
                                <Show
                                    when=is_editing
                                    fallback=move || {
                                        let current_user = users.get()
                                            .iter()
                                            .find(|u| u.id == user_id_for_display)
                                            .cloned()
                                            .unwrap_or_else(|| user_fallback.clone());
                                        view! {
                                            <UserDisplayItem 
                                                user=current_user
                                                on_edit=set_editing_id
                                            />
                                        }
                                    }
                                >
                                    {
                                        let user_id_for_edit_clone = user_id_for_edit.clone();
                                        let user_edit_clone = user_edit.clone();
                                        move || {
                                            let current_user = users.get()
                                                .iter()
                                                .find(|u| u.id == user_id_for_edit_clone)
                                                .cloned()
                                                .unwrap_or_else(|| user_edit_clone.clone());
                                            view! {
                                                <UserEditItem 
                                                    user=current_user
                                                    on_cancel=set_editing_id
                                                />
                                            }
                                        }
                                    }
                                </Show>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}