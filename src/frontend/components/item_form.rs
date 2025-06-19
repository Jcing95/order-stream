use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::CreateItemRequest;

#[component]
pub fn ItemForm<F>(on_submit: F) -> impl IntoView 
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
            category: category.get().trim().to_string(),
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
        <form on:submit=submit_form class="space-y-4 p-4 border rounded-lg">
            <h3 class="text-lg font-semibold">"Add New Item"</h3>
            
            {move || error.get().map(|err| view! {
                <div class="text-red-600 bg-red-50 p-2 rounded">
                    {err}
                </div>
            })}
            
            <div>
                <label class="block text-sm font-medium text-gray-700">
                    "Name"
                </label>
                <input
                    type="text"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    required
                />
            </div>
            
            <div>
                <label class="block text-sm font-medium text-gray-700">
                    "Category"
                </label>
                <input
                    type="text"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                    prop:value=move || category.get()
                    on:input=move |ev| set_category.set(event_target_value(&ev))
                    required
                />
            </div>
            
            <div>
                <label class="block text-sm font-medium text-gray-700">
                    "Price"
                </label>
                <input
                    type="number"
                    step="0.01"
                    min="0"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                    prop:value=move || price.get()
                    on:input=move |ev| set_price.set(event_target_value(&ev))
                    required
                />
            </div>
            
            <button
                type="submit"
                class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
                "Add Item"
            </button>
        </form>
    }
}