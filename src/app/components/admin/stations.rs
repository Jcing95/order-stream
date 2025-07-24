use leptos::prelude::*;

use crate::{
    app::{
        components::atoms::icons,
        states::{category, station},
    },
    backend::station::{delete_station, UpdateStation},
    common::types::OrderStatus,
};

#[component]
fn StationDisplayItem(
    station: crate::common::types::Station,
    on_edit: WriteSignal<Option<String>>,
) -> impl IntoView {
    let category_state = category::get();
    let categories = category_state.get_categories();
    
    let delete_action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            let _ = delete_station(input.clone()).await;
        }
    });

    let id = station.id.clone();
    let name = station.name.clone();
    let category_ids = station.category_ids.clone();
    let input_statuses = station.input_statuses.clone();
    let output_status = station.output_status;
    
    // Get category names reactively
    let category_names = move || {
        let cats = categories.get();
        category_ids
            .iter()
            .filter_map(|cat_id| {
                cats.iter()
                    .find(|c| &c.id == cat_id)
                    .map(|c| c.name.clone())
            })
            .collect::<Vec<String>>()
            .join(", ")
    };

    let status_display = move || {
        let input_str = input_statuses
            .iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}] → {:?}", input_str, output_status)
    };

    view! {
        <div class="p-3 bg-surface-elevated rounded-md border border-border">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <div class="flex items-center justify-between">
                        <span class="text-text font-medium">
                            {name}
                        </span>
                        <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                    </div>
                    <div class="mt-1 text-sm text-text-muted">
                        <div>{"Categories: "}{move || category_names()}</div>
                        <div class="mt-1">{"Status Flow: "}{move || status_display()}</div>
                    </div>
                </div>
                
                <div class="flex items-center space-x-2 ml-4">
                    <button
                        class="bg-border/80 text-blue-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click={
                            let id = id.clone();
                            move |_| {
                                on_edit.set(Some(id.clone()));
                            }
                        }
                    >
                        <icons::Edit />
                    </button>
                    
                    <button
                        class="bg-border/80 text-red-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click={
                            let id = id.clone();
                            move |_| {
                                delete_action.dispatch(id.clone());
                            }
                        }
                    >
                        <icons::Trash />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StationEditItem(
    station: crate::common::types::Station,
    on_cancel: WriteSignal<Option<String>>,
) -> impl IntoView {
    let category_state = category::get();
    let categories = category_state.get_categories();
    
    let (edit_name, set_edit_name) = signal(station.name.clone());
    let (edit_category_ids, set_edit_category_ids) = signal(station.category_ids.clone());
    let (edit_input_statuses, set_edit_input_statuses) = signal(station.input_statuses.clone());
    let (edit_output_status, set_edit_output_status) = signal(station.output_status);
    
    let update_action = ServerAction::<UpdateStation>::new();
    
    let id = station.id.clone();
    let original_name = station.name.clone();
    let original_category_ids = station.category_ids.clone();
    let original_input_statuses = station.input_statuses.clone();
    let original_output_status = station.output_status;

    let all_statuses = || vec![
        OrderStatus::Draft,
        OrderStatus::Ordered,
        OrderStatus::Ready,
        OrderStatus::Completed,
        OrderStatus::Cancelled,
    ];

    let toggle_category = move |category_id: String| {
        let mut current_ids = edit_category_ids.get();
        if current_ids.contains(&category_id) {
            current_ids.retain(|id| id != &category_id);
        } else {
            current_ids.push(category_id);
        }
        set_edit_category_ids.set(current_ids);
    };

    let toggle_input_status = move |status: OrderStatus| {
        let mut current_statuses = edit_input_statuses.get();
        if current_statuses.contains(&status) {
            current_statuses.retain(|s| s != &status);
        } else {
            current_statuses.push(status);
        }
        set_edit_input_statuses.set(current_statuses);
    };

    view! {
        <div class="p-3 bg-surface-elevated rounded-md border border-border">
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <span class="text-text-muted text-sm">{"ID: "}{id.clone()}</span>
                </div>
                
                <div class="grid grid-cols-1 gap-3">
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Name"</label>
                        <input
                            type="text"
                            prop:value=move || edit_name.get()
                            on:input=move |ev| {
                                set_edit_name.set(event_target_value(&ev));
                            }
                            class="w-full px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-2">"Categories"</label>
                        <div class="grid grid-cols-2 gap-2">
                            <For
                                each=move || categories.get()
                                key=|cat| cat.id.clone()
                                children=move |cat| {
                                    let cat_id = cat.id.clone();
                                    let cat_id_for_toggle = cat_id.clone();
                                    view! {
                                        <label class="flex items-center space-x-2 p-2 border border-border rounded text-sm">
                                            <input
                                                type="checkbox"
                                                prop:checked=move || edit_category_ids.get().contains(&cat_id)
                                                on:change=move |_| {
                                                    toggle_category(cat_id_for_toggle.clone());
                                                }
                                                class="h-4 w-4 text-primary focus:ring-primary border-border rounded"
                                            />
                                            <span class="text-text">{cat.name}</span>
                                        </label>
                                    }
                                }
                            />
                        </div>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-2">"Input Statuses"</label>
                        <div class="grid grid-cols-2 gap-2">
                            <For
                                each=move || all_statuses()
                                key=|status| format!("{:?}", status)
                                children=move |status| {
                                    let status_for_toggle = status;
                                    view! {
                                        <label class="flex items-center space-x-2 p-2 border border-border rounded text-sm">
                                            <input
                                                type="checkbox"
                                                prop:checked=move || edit_input_statuses.get().contains(&status)
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
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-text mb-1">"Output Status"</label>
                        <select
                            prop:value=move || format!("{:?}", edit_output_status.get())
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                match value.as_str() {
                                    "Draft" => set_edit_output_status.set(OrderStatus::Draft),
                                    "Ordered" => set_edit_output_status.set(OrderStatus::Ordered),
                                    "Ready" => set_edit_output_status.set(OrderStatus::Ready),
                                    "Completed" => set_edit_output_status.set(OrderStatus::Completed),
                                    "Cancelled" => set_edit_output_status.set(OrderStatus::Cancelled),
                                    _ => {}
                                }
                            }
                            class="w-full px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                        >
                            <For
                                each=move || all_statuses()
                                key=|status| format!("{:?}", status)
                                children=move |status| {
                                    let status_str = format!("{:?}", status);
                                    let status_str_clone = status_str.clone();
                                    view! {
                                        <option value={status_str} selected=move || edit_output_status.get() == status>
                                            {status_str_clone}
                                        </option>
                                    }
                                }
                            />
                        </select>
                    </div>
                </div>
                
                <div class="flex justify-end space-x-2">
                    <ActionForm 
                        action=update_action
                        on:submit=move |_| {
                            on_cancel.set(None);
                        }
                    >
                        <input type="hidden" name="id" value={id.clone()} />
                        <input type="hidden" name="name" value=move || edit_name.get() />
                        <input 
                            type="hidden" 
                            name="category_ids_json" 
                            value=move || {
                                serde_json::to_string(&edit_category_ids.get()).unwrap_or_default()
                            }
                        />
                        <input 
                            type="hidden" 
                            name="input_statuses_json" 
                            value=move || {
                                serde_json::to_string(&edit_input_statuses.get()).unwrap_or_default()
                            }
                        />
                        <input type="hidden" name="output_status" value=move || format!("{:?}", edit_output_status.get()) />
                        <button
                            type="submit"
                            class="bg-border/80 text-green-600 hover:bg-border hover:scale-105 p-2 rounded"
                        >
                            <icons::Accept />
                        </button>
                    </ActionForm>
                    
                    <button
                        class="bg-border/80 text-gray-600 hover:bg-border hover:scale-105 p-2 rounded"
                        on:click=move |_| {
                            set_edit_name.set(original_name.clone());
                            set_edit_category_ids.set(original_category_ids.clone());
                            set_edit_input_statuses.set(original_input_statuses.clone());
                            set_edit_output_status.set(original_output_status);
                            on_cancel.set(None);
                        }
                    >
                        <icons::Cancel />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Stations() -> impl IntoView {
    let station_state = station::get();
    let stations = station_state.get_stations();
    let (editing_id, set_editing_id) = signal::<Option<String>>(None);

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <h2 class="text-xl font-semibold text-text mb-4">"Stations"</h2>

            <Show
                when=move || !stations.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-8">
                        <p class="text-text-muted">"No stations found"</p>
                    </div>
                }
            >
                <div class="space-y-2">
                    <For
                        each=move || {
                            let stations_data = stations.get();
                            stations_data
                        }
                        key=|station| station.id.clone()
                        children=move |station| {
                            let station_id = station.id.clone();
                            let station_id_for_editing = station_id.clone();
                            let station_id_for_display = station_id.clone();
                            let station_id_for_edit = station_id.clone();
                            let station_fallback = station.clone();
                            let station_edit = station.clone();
                            
                            let is_editing = move || editing_id.get() == Some(station_id_for_editing.clone());
                            
                            view! {
                                <Show
                                    when=is_editing
                                    fallback=move || {
                                        let current_station = stations.get()
                                            .iter()
                                            .find(|s| s.id == station_id_for_display)
                                            .cloned()
                                            .unwrap_or_else(|| station_fallback.clone());
                                        view! {
                                            <StationDisplayItem 
                                                station=current_station
                                                on_edit=set_editing_id
                                            />
                                        }
                                    }
                                >
                                    {
                                        let station_id_for_edit_clone = station_id_for_edit.clone();
                                        let station_edit_clone = station_edit.clone();
                                        move || {
                                            let current_station = stations.get()
                                                .iter()
                                                .find(|s| s.id == station_id_for_edit_clone)
                                                .cloned()
                                                .unwrap_or_else(|| station_edit_clone.clone());
                                            view! {
                                                <StationEditItem 
                                                    station=current_station
                                                    on_cancel=set_editing_id
                                                />
                                            }
                                        }
                                    }
                                </Show>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}