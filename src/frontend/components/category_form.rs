use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::CreateCategoryRequest;
use crate::frontend::state::theme::{card_surface, input_field, label_text, button_primary, text_primary, alert_base, alert_error};

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
        <form on:submit=submit_form class=format!("space-y-4 p-4 {}", card_surface())>
            <h3 class=format!("text-lg font-semibold {}", text_primary())>"Add New Category"</h3>
            
            {move || error.get().map(|err| view! {
                <div class=format!("{} {}", alert_base(), alert_error())>
                    {err}
                </div>
            })}
            
            <div>
                <label class=label_text()>
                    "Category Name"
                </label>
                <input
                    type="text"
                    class=format!("mt-1 {}", input_field())
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    placeholder="e.g., Drinks, Food, Snacks"
                    required
                />
            </div>
            
            <button
                type="submit"
                class=format!("w-full {}", button_primary())
            >
                "Add Category"
            </button>
        </form>
    }
}