use leptos::prelude::*;
use crate::common::types::Category;

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
            <h3 class="text-lg font-semibold">"Categories"</h3>
            
            {move || {
                let cats = categories.get();
                if cats.is_empty() {
                    view! {
                        <div class="text-gray-500 italic p-4 border rounded-lg">
                            "No categories yet. Add one above to get started."
                        </div>
                    }.into_any()
                } else {
                    cats.into_iter().map(|category| {
                        let category_id = category.id.clone();
                        let on_delete_inner = on_delete_clone.clone();
                        view! {
                            <div class="flex items-center justify-between p-4 border rounded-lg bg-gray-50">
                                <div>
                                    <h4 class="font-medium">{move || category.name.clone()}</h4>
                                    <p class="text-sm text-gray-500">
                                        "ID: " {move || category.id.clone()}
                                    </p>
                                </div>
                                <button
                                    class="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
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