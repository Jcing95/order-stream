use leptos::prelude::*;
use crate::common::types::Category;
use crate::frontend::state::theme::{card_surface, text_primary, text_secondary, text_muted, button_danger, button_small};

#[component]
pub fn CategoryList<F>(
    categories: ReadSignal<Vec<Category>>,
    on_delete: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Clone + Send,
{
    let on_delete_clone = on_delete.clone();

    view! {
        <div class="space-y-4">
            <h3 class=format!("text-lg font-semibold {}", text_primary())>"Categories"</h3>
            
            {move || {
                let cats = categories.get();
                if cats.is_empty() {
                    view! {
                        <div class=format!("italic p-4 {} {}", text_muted(), card_surface())>
                            "No categories yet. Add one above to get started."
                        </div>
                    }.into_any()
                } else {
                    cats.into_iter().map(|category| {
                        let category_id = category.id.clone();
                        let on_delete_inner = on_delete_clone.clone();
                        view! {
                            <div class=format!("flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600")>
                                <div>
                                    <h4 class=format!("font-medium {}", text_primary())>{move || category.name.clone()}</h4>
                                    <p class=format!("text-sm {}", text_secondary())>
                                        "ID: " {move || category.id.clone()}
                                    </p>
                                </div>
                                <button
                                    class=format!("{} {}", button_danger(), button_small())
                                    on:click=move |_| on_delete_inner(category_id.clone())
                                >
                                    "Delete"
                                </button>
                            </div>
                        }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}