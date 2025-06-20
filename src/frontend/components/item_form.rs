use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::{CreateItemRequest, Category};

#[component]
pub fn ItemForm<F>(
    categories: ReadSignal<Vec<Category>>,
    on_submit: F,
) -> impl IntoView 
where
    F: Fn(CreateItemRequest) + 'static + Clone,
{
    let (name, set_name) = signal(String::new());
    let (category, set_category) = signal(String::new());
    let (price, set_price) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    let on_submit_clone = on_submit.clone();
    let submit_form = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        set_error.set(None);
        
        // Parse price
        let price_value = match price.get().parse::<f64>() {
            Ok(p) if p >= 0.0 => p,
            Ok(_) => {
                set_error.set(Some("Price cannot be negative".to_string()));
                return;
            }
            Err(_) => {
                set_error.set(Some("Please enter a valid price".to_string()));
                return;
            }
        };

        let request = CreateItemRequest {
            name: name.get().trim().to_string(),
            category_id: category.get().trim().to_string(),
            price: price_value,
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
        set_category.set(String::new());
        set_price.set(String::new());
    };

    view! {
        <form on:submit=submit_form class="space-y-6 p-6 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 backdrop-blur-sm">
            <h3 class="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">"Add New Item"</h3>
            
            {move || error.get().map(|err| view! {
                <div class="text-red-700 dark:text-red-300 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-3 rounded-lg shadow-sm">
                    {err}
                </div>
            })}
            
            <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-700 dark:text-gray-300">
                    "Name"
                </label>
                <input
                    type="text"
                    class="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 placeholder-gray-400 dark:placeholder-gray-500"
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    required
                    placeholder="Enter item name"
                />
            </div>
            
            <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-700 dark:text-gray-300">
                    "Category"
                </label>
                <select
                    class="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200"
                    prop:value=move || category.get()
                    on:change=move |ev| set_category.set(event_target_value(&ev))
                    required
                >
                    <option value="">"Select a category..."</option>
                    {move || {
                        categories.get().into_iter().map(|cat| {
                            view! {
                                <option value={cat.id.clone()}>{move || cat.name.clone()}</option>
                            }
                        }).collect_view()
                    }}
                </select>
            </div>
            
            <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-700 dark:text-gray-300">
                    "Price"
                </label>
                <input
                    type="number"
                    step="0.01"
                    min="0"
                    class="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 placeholder-gray-400 dark:placeholder-gray-500"
                    prop:value=move || price.get()
                    on:input=move |ev| set_price.set(event_target_value(&ev))
                    required
                    placeholder="0.00"
                />
            </div>
            
            <button
                type="submit"
                class="w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold py-3 px-6 rounded-lg shadow-lg hover:shadow-xl transform hover:scale-[1.02] focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-gray-800 transition-all duration-200"
            >
                "Add Item"
            </button>
        </form>
    }
}