use leptos::prelude::*;

use crate::{app::states::category, backend::category::delete_category};

#[component]
pub fn Categories() -> impl IntoView {
    let category_state = category::get();
    let categories = category_state.get_categories();

    let delete_action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            let _ = delete_category(input.clone()).await;
        }
    });

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <h2 class="text-xl font-semibold text-text mb-4">"Categories"</h2>

            <Show
                when=move || !categories.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-8">
                        <p class="text-text-muted">"No categories found"</p>
                    </div>
                }
            >
                <div class="space-y-2">
                    <For
                        each=move || categories.get()
                        key=|category| category.id.clone()
                        children=move |category| {
                            let id = category.id.clone();
                            view! {
                                <div class="flex items-center justify-between p-3 bg-surface-elevated rounded-md border border-border">
                                    <span class="text-text font-medium">{category.name}</span>
                                    <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                                    <button
                                        class="text-red-600 hover:underline"
                                        on:click=move |_| {
                                            delete_action.dispatch(id.clone());
                                        }
                                    >
                                        "Delete"
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}
