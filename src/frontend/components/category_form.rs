use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::CreateCategoryRequest;

#[component]
pub fn CategoryForm<F>(on_submit: F) -> impl IntoView 
where
    F: Fn(CreateCategoryRequest) + 'static + Clone,
{
    let (name, set_name) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    let on_submit_clone = on_submit.clone();
    let submit_form = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        set_error.set(None);
        
        let request = CreateCategoryRequest {
            name: name.get().trim().to_string(),
        };

        // Validate
        if let Err(err) = request.validate() {
            set_error.set(Some(err));
            return;
        }

        // Submit
        on_submit_clone(request);
        
        // Clear form
        set_name.set(String::new());
    };

    view! {
        <form on:submit=submit_form class="space-y-4 p-4 border rounded-lg">
            <h3 class="text-lg font-semibold">"Add New Category"</h3>
            
            {move || error.get().map(|err| view! {
                <div class="text-red-600 bg-red-50 p-2 rounded">
                    {err}
                </div>
            })}
            
            <div>
                <label class="block text-sm font-medium text-gray-700">
                    "Category Name"
                </label>
                <input
                    type="text"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    placeholder="e.g., Drinks, Food, Snacks"
                    required
                />
            </div>
            
            <button
                type="submit"
                class="w-full bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
            >
                "Add Category"
            </button>
        </form>
    }
}