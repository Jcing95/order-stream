use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::{CreateItemRequest, Category};
use crate::frontend::state::theme::{card_elevated, input_field, label_text, button_primary, text_gradient, alert_base, alert_error};

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
        <form on:submit=submit_form class=format!("space-y-6 p-6 {}", card_elevated())>
            <h3 class=format!("text-xl font-bold {}", text_gradient())>"Add New Item"</h3>
            
            {move || error.get().map(|err| view! {
                <div class=format!("{} {}", alert_base(), alert_error())>
                    {err}
                </div>
            })}
            
            <div class="space-y-2">
                <label class=label_text()>
                    "Name"
                </label>
                <input
                    type="text"
                    class=input_field()
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    required
                    placeholder="Enter item name"
                />
            </div>
            
            <div class="space-y-2">
                <label class=label_text()>
                    "Category"
                </label>
                <select
                    class=input_field()
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
                <label class=label_text()>
                    "Price"
                </label>
                <input
                    type="number"
                    step="0.01"
                    min="0"
                    class=input_field()
                    prop:value=move || price.get()
                    on:input=move |ev| set_price.set(event_target_value(&ev))
                    required
                    placeholder="0.00"
                />
            </div>
            
            <button
                type="submit"
                class=format!("w-full {}", button_primary())
            >
                "Add Item"
            </button>
        </form>
    }
}