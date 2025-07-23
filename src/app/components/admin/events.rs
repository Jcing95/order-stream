use leptos::prelude::*;

use crate::{
    app::{components::atoms::icons, states::event},
    backend::event::{delete_event, UpdateEvent},
};

#[component]
fn EventDisplayItem(
    event: crate::common::types::Event,
    on_edit: WriteSignal<Option<String>>,
) -> impl IntoView {
    let delete_action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            let _ = delete_event(input.clone()).await;
        }
    });

    let id = event.id.clone();
    let name = event.name.clone();

    view! {
        <div class="flex items-center justify-between p-3 bg-surface-elevated rounded-md border border-border">
            <div class="flex-1 flex items-center justify-between">
                <span class="text-text font-medium">{name}</span>
                <span class="text-text-muted text-sm ml-4">{"ID: "}{id.clone()}</span>
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
    }
}

#[component]
fn EventEditItem(
    event: crate::common::types::Event,
    on_cancel: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (edit_name, set_edit_name) = signal(event.name.clone());
    let update_action = ServerAction::<UpdateEvent>::new();
    
    // Close edit mode when update succeeds
    Effect::new(move |_| {
        if let Some(Ok(_)) = update_action.value().get() {
            on_cancel.set(None);
        }
    });
    
    let id = event.id.clone();
    let original_name = event.name.clone();

    view! {
        <div class="flex items-center justify-between p-3 bg-surface-elevated rounded-md border border-border">
            <div class="flex-1 flex items-center justify-between">
                <input
                    type="text"
                    prop:value=move || edit_name.get()
                    on:input=move |ev| {
                        set_edit_name.set(event_target_value(&ev));
                    }
                    class="flex-1 px-2 py-1 border border-border bg-surface text-text rounded focus:outline-none focus:ring-primary focus:border-primary text-sm"
                />
                <span class="text-text-muted text-sm ml-4">{"ID: "}{id.clone()}</span>
            </div>
            
            <div class="flex items-center space-x-2 ml-4">
                <ActionForm 
                    action=update_action
                    on:submit=move |_| {
                        on_cancel.set(None);
                    }
                >
                    <input type="hidden" name="id" value={id.clone()} />
                    <input type="hidden" name="update[name]" value=move || edit_name.get() />
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
                        on_cancel.set(None);
                    }
                >
                    <icons::Cancel />
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Events() -> impl IntoView {
    let event_state = event::get();
    let events = event_state.get_events();
    let (editing_id, set_editing_id) = signal::<Option<String>>(None);

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <h2 class="text-xl font-semibold text-text mb-4">"Events"</h2>

            <Show
                when=move || !events.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-8">
                        <p class="text-text-muted">"No events found"</p>
                    </div>
                }
            >
                <div class="space-y-2">
                    <For
                        each=move || events.get()
                        key=|event| event.id.clone()
                        children=move |event| {
                            let event_id = event.id.clone();
                            let event_id_for_editing = event_id.clone();
                            let event_id_for_display = event_id.clone();
                            let event_id_for_edit = event_id.clone();
                            let event_fallback = event.clone();
                            let event_edit = event.clone();
                            
                            let is_editing = move || editing_id.get() == Some(event_id_for_editing.clone());
                            
                            view! {
                                <Show
                                    when=is_editing
                                    fallback=move || {
                                        let current_event = events.get()
                                            .iter()
                                            .find(|e| e.id == event_id_for_display)
                                            .cloned()
                                            .unwrap_or_else(|| event_fallback.clone());
                                        view! {
                                            <EventDisplayItem 
                                                event=current_event
                                                on_edit=set_editing_id
                                            />
                                        }
                                    }
                                >
                                    {
                                        let event_id_for_edit_clone = event_id_for_edit.clone();
                                        let event_edit_clone = event_edit.clone();
                                        move || {
                                            let current_event = events.get()
                                                .iter()
                                                .find(|e| e.id == event_id_for_edit_clone)
                                                .cloned()
                                                .unwrap_or_else(|| event_edit_clone.clone());
                                            view! {
                                                <EventEditItem 
                                                    event=current_event
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