use leptos::prelude::*;

use crate::{
    backend::station::CreateStation,
    app::{
        states::category,
        components::atoms::icons,
    },
    common::types::OrderStatus,
};

#[component]
pub fn CreateStation() -> impl IntoView {
    let create_action = ServerAction::<CreateStation>::new();
    let category_state = category::get();
    let categories = category_state.get_categories();

    let (selected_categories, set_selected_categories) = signal::<Vec<String>>(Vec::new());
    let (selected_input_statuses, set_selected_input_statuses) = signal::<Vec<OrderStatus>>(Vec::new());
    let (selected_output_status, set_selected_output_status) = signal(OrderStatus::Ready);

    // Handle successful station creation
    Effect::new(move |_| {
        if let Some(Ok(_station)) = create_action.value().get() {
            // Station created successfully - could add success notification here
            // Clear form
            set_selected_categories.set(Vec::new());
            set_selected_input_statuses.set(Vec::new());
            set_selected_output_status.set(OrderStatus::Ready);
        }
    });

    let all_statuses = || vec![
        OrderStatus::Draft,
        OrderStatus::Ordered,
        OrderStatus::Ready,
        OrderStatus::Completed,
        OrderStatus::Cancelled,
    ];

    let toggle_category = move |category_id: String| {
        let mut current_ids = selected_categories.get();
        if current_ids.contains(&category_id) {
            current_ids.retain(|id| id != &category_id);
        } else {
            current_ids.push(category_id);
        }
        set_selected_categories.set(current_ids);
    };

    let toggle_input_status = move |status: OrderStatus| {
        let mut current_statuses = selected_input_statuses.get();
        if current_statuses.contains(&status) {
            current_statuses.retain(|s| s != &status);
        } else {
            current_statuses.push(status);
        }
        set_selected_input_statuses.set(current_statuses);
    };

    view! {
        <div class="max-w-2xl w-full space-y-8">            
            <ActionForm 
                action=create_action
                attr:class="mt-8 space-y-6"
            >
                <div class="space-y-4">
                    <div>
                        <label for="name" class="block text-sm font-medium text-text mb-2">"Station Name"</label>
                        <input
                            id="name"
                            name="name"
                            type="text"
                            required
                            class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                            placeholder="Enter station name"
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-2">"Categories"</label>
                        <div class="grid grid-cols-2 gap-2 p-3 border border-border bg-surface rounded-md max-h-48 overflow-y-auto">
                            <Show
                                when=move || !categories.get().is_empty()
                                fallback=|| view! {
                                    <div class="col-span-2 text-center py-4">
                                        <p class="text-text-muted text-sm">"No categories available. Create categories first."</p>
                                    </div>
                                }
                            >
                                <For
                                    each=move || categories.get()
                                    key=|category| category.id.clone()
                                    children=move |category| {
                                        let category_id = category.id.clone();
                                        let category_id_for_toggle = category_id.clone();
                                        view! {
                                            <label class="flex items-center space-x-2 p-2 border border-border rounded text-sm hover:bg-background cursor-pointer">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || selected_categories.get().contains(&category_id)
                                                    on:change=move |_| {
                                                        toggle_category(category_id_for_toggle.clone());
                                                    }
                                                    class="h-4 w-4 text-primary focus:ring-primary border-border rounded"
                                                />
                                                <span class="text-text">{category.name}</span>
                                            </label>
                                        }
                                    }
                                />
                            </Show>
                        </div>
                        <input 
                            type="hidden" 
                            name="category_ids_json" 
                            value=move || {
                                let cats = selected_categories.get();
                                serde_json::to_string(&cats).unwrap_or_default()
                            }
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-2">"Input Statuses"</label>
                        <div class="grid grid-cols-2 gap-2 p-3 border border-border bg-surface rounded-md">
                            <For
                                each=move || all_statuses()
                                key=|status| format!("{:?}", status)
                                children=move |status| {
                                    let status_for_toggle = status;
                                    view! {
                                        <label class="flex items-center space-x-2 p-2 border border-border rounded text-sm hover:bg-background cursor-pointer">
                                            <input
                                                type="checkbox"
                                                prop:checked=move || selected_input_statuses.get().contains(&status)
                                                on:change=move |_| {
                                                    toggle_input_status(status_for_toggle);
                                                }
                                                class="h-4 w-4 text-primary focus:ring-primary border-border rounded"
                                            />
                                            <span class="text-text">{format!("{:?}", status)}</span>
                                        </label>
                                    }
                                }
                            />
                        </div>
                        <input 
                            type="hidden" 
                            name="input_statuses_json" 
                            value=move || {
                                let statuses = selected_input_statuses.get();
                                serde_json::to_string(&statuses).unwrap_or_default()
                            }
                        />
                    </div>
                    
                    <div>
                        <label for="output_status" class="block text-sm font-medium text-text mb-2">"Output Status"</label>
                        <select
                            id="output_status"
                            name="output_status"
                            required
                            prop:value=move || format!("{:?}", selected_output_status.get())
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                match value.as_str() {
                                    "Draft" => set_selected_output_status.set(OrderStatus::Draft),
                                    "Ordered" => set_selected_output_status.set(OrderStatus::Ordered),
                                    "Ready" => set_selected_output_status.set(OrderStatus::Ready),
                                    "Completed" => set_selected_output_status.set(OrderStatus::Completed),
                                    "Cancelled" => set_selected_output_status.set(OrderStatus::Cancelled),
                                    _ => {}
                                }
                            }
                            class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                        >
                            <For
                                each=move || all_statuses()
                                key=|status| format!("{:?}", status)
                                children=move |status| {
                                    let status_str = format!("{:?}", status);
                                    let status_str_clone = status_str.clone();
                                    view! {
                                        <option value={status_str} selected=move || selected_output_status.get() == status>
                                            {status_str_clone}
                                        </option>
                                    }
                                }
                            />
                        </select>
                    </div>
                </div>

                <Show when=move || create_action.value().get().as_ref().map(|result| result.is_err()).unwrap_or(false)>
                    <div class="bg-red-50 border border-red-200 rounded-md p-4">
                        <div class="flex">
                            <div class="ml-3">
                                <h3 class="text-sm font-medium text-red-800">
                                    {move || {
                                        create_action.value().get()
                                            .and_then(|result| result.err())
                                            .map(|err| err.to_string())
                                            .unwrap_or_else(|| "An error occurred".to_string())
                                    }}
                                </h3>
                            </div>
                        </div>
                    </div>
                </Show>

                <div>
                    <button
                        type="submit"
                        disabled=move || create_action.pending().get() || selected_categories.get().is_empty() || selected_input_statuses.get().is_empty()
                        class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <Show
                            when=move || create_action.pending().get()
                            fallback=|| view! { "Create Station" }
                        >
                            <span class="flex items-center">
                                <icons::Spinner attr:class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"/>
                                "Creating station..."
                            </span>
                        </Show>
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}