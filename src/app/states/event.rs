use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Event;
use crate::backend::event::get_events;
use crate::common::resource_type::Message;
use crate::app::states::websocket;



#[derive(Debug, Clone)]
pub struct EventState {
    events: ReadSignal<Vec<Event>>,
    set_events: WriteSignal<Vec<Event>>,
}

impl EventState {
    pub fn new() -> Self {
        let (events, set_events) = signal(Vec::new());
        
        // Load events once on initialization using Effect
        Effect::new({
            let set_events = set_events;
            move |_| {
                spawn_local(async move {
                    match get_events().await {
                        Ok(evt) => set_events.set(evt),
                        Err(_) => {}, // Keep empty vec on error
                    }
                });
            }
        });

        let event_state = Self {
            events,
            set_events,
        };

        // Connect to websocket updates
        let websocket_state = websocket::get();
        Effect::new({
            let event_state = event_state.clone();
            let websocket_state = websocket_state.clone();
            move |_| {
                if let Some(message) = websocket_state.events.get() {
                    match message {
                        Message::Add(event) => {
                            event_state.add_event(event);
                        }
                        Message::Update(event) => {
                            event_state.update_event(event);
                        }
                        Message::Delete(id) => {
                            event_state.remove_event(&id);
                        }
                    }
                    // Clear the signal after processing to allow new messages to trigger
                    websocket_state.events.set(None);
                }
            }
        });

        event_state
    }

    /// Get the events signal for reactive UI - this is your simple interface
    pub fn get_events(&self) -> ReadSignal<Vec<Event>> {
        self.events
    }

    /// Add a single event (for WebSocket updates when someone creates an event)
    pub fn add_event(&self, event: Event) {
        self.set_events.update(|events| events.push(event));
    }
    
    /// Update a single event (for WebSocket updates when someone modifies an event)
    pub fn update_event(&self, updated: Event) {
        self.set_events.update(|events| {
            let mut new_events = Vec::new();
            for event in events.iter() {
                if event.id == updated.id {
                    new_events.push(updated.clone());
                } else {
                    new_events.push(event.clone());
                }
            }
            *events = new_events;
        });
    }
    
    /// Remove an event (for WebSocket updates when someone deletes an event)
    pub fn remove_event(&self, id: &str) {
        self.set_events.update(|events| events.retain(|e| e.id != id));
    }
    
    /// Replace all events (for full refresh if needed)
    pub fn set_events(&self, events: Vec<Event>) {
        self.set_events.set(events);
    }
}

pub fn provide() -> EventState {
    let event_state = EventState::new();
    provide_context(event_state.clone());
    event_state
}

pub fn get() -> EventState {
    expect_context::<EventState>()
}