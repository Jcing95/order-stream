use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Settings;
use crate::backend::settings::get_settings;
use crate::common::resource_type::Message;
use crate::app::states::websocket;



#[derive(Debug, Clone)]
pub struct SettingsState {
    settings: ReadSignal<Option<Settings>>,
    set_settings: WriteSignal<Option<Settings>>,
}

impl SettingsState {
    pub fn new() -> Self {
        let (settings, set_settings) = signal(None);
        
        // Load settings once on initialization using Effect
        Effect::new({
            let set_settings = set_settings;
            move |_| {
                spawn_local(async move {
                    match get_settings().await {
                        Ok(s) => set_settings.set(Some(s)),
                        Err(_) => {}, // Keep None on error
                    }
                });
            }
        });

        let settings_state = Self {
            settings,
            set_settings,
        };

        // Connect to websocket updates
        let websocket_state = websocket::get();
        Effect::new({
            let settings_state = settings_state.clone();
            let websocket_state = websocket_state.clone();
            move |_| {
                if let Some(message) = websocket_state.settings.get() {
                    match message {
                        Message::Add(s) => {
                            settings_state.set_settings(s);
                        }
                        Message::Update(s) => {
                            settings_state.set_settings(s);
                        }
                        Message::Delete(_) => {
                            settings_state.set_settings.set(None);
                        }
                    }
                    // Clear the signal after processing to allow new messages to trigger
                    websocket_state.settings.set(None);
                }
            }
        });

        settings_state
    }

    /// Get the settings signal for reactive UI
    pub fn get_settings(&self) -> ReadSignal<Option<Settings>> {
        self.settings
    }

    /// Set the settings (for WebSocket updates)
    pub fn set_settings(&self, settings: Settings) {
        self.set_settings.set(Some(settings));
    }
    
    /// Get the current active event ID if available
    pub fn get_active_event_id(&self) -> Option<String> {
        self.settings.get().and_then(|s| s.active_event_id)
    }
}

pub fn provide() -> SettingsState {
    let settings_state = SettingsState::new();
    provide_context(settings_state.clone());
    settings_state
}

pub fn get() -> SettingsState {
    expect_context::<SettingsState>()
}