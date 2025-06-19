use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{CreateItemRequest, Item};
use crate::frontend::components::{item_form::ItemForm, item_list::ItemList};
use crate::backend::api::items::{get_items, create_item};

#[component]
pub fn AdminPage() -> impl IntoView {
    let (items, set_items) = signal(Vec::<Item>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

    // Load items on component mount
    let load_items = Action::new(move |_: &()| async move {
        set_loading.set(true);
        set_error.set(None);
        
        match get_items().await {
            Ok(fetched_items) => {
                set_items.set(fetched_items);
            }
            Err(err) => {
                set_error.set(Some(format!("Failed to load items: {}", err)));
            }
        }
        
        set_loading.set(false);
    });

    // Load items when component mounts
    Effect::new(move |_| {
        load_items.dispatch(());
    });

    // Handle form submission
    let handle_submit = move |request: CreateItemRequest| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_item(request).await {
                Ok(new_item) => {
                    // Add the new item to the list
                    set_items.update(|items| items.push(new_item));
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to create item: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };

    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-6">"Admin Panel"</h1>
            
            {move || error.get().map(|err| view! {
                <div class="mb-4 text-red-600 bg-red-50 p-3 rounded">
                    {err}
                </div>
            })}
            
            {move || {
                if loading.get() {
                    view! {
                        <div class="mb-4 text-blue-600 bg-blue-50 p-3 rounded">
                            "Loading..."
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div>
                    <ItemForm on_submit=handle_submit />
                </div>
                <div>
                    <ItemList items=items.into() />
                </div>
            </div>
        </div>
    }
}