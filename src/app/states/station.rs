use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Station;
use crate::backend::station::get_stations;
use crate::common::resource_type::Message;
use crate::app::states::websocket;

#[derive(Debug, Clone)]
pub struct StationState {
    stations: ReadSignal<Vec<Station>>,
    set_stations: WriteSignal<Vec<Station>>,
}

impl StationState {
    pub fn new() -> Self {
        let (stations, set_stations) = signal(Vec::new());
        
        // Load stations once on initialization using Effect
        Effect::new({
            let set_stations = set_stations;
            move |_| {
                spawn_local(async move {
                    match get_stations().await {
                        Ok(stations_data) => set_stations.set(stations_data),
                        Err(_) => {}, // Keep empty vec on error
                    }
                });
            }
        });

        let station_state = Self {
            stations,
            set_stations,
        };

        // Connect to websocket updates
        let websocket_state = websocket::get();
        Effect::new({
            let station_state = station_state.clone();
            let websocket_state = websocket_state.clone();
            move |_| {
                if let Some(message) = websocket_state.stations.get() {
                    match message {
                        Message::Add(station) => {
                            station_state.add_station(station);
                        }
                        Message::Update(station) => {
                            station_state.update_station(station);
                        }
                        Message::Delete(id) => {
                            station_state.remove_station(&id);
                        }
                    }
                    // Clear the signal after processing to allow new messages to trigger
                    websocket_state.stations.set(None);
                }
            }
        });

        station_state
    }

    pub fn get_stations(&self) -> ReadSignal<Vec<Station>> {
        self.stations
    }

    pub fn set_stations(&self, stations: Vec<Station>) {
        self.set_stations.set(stations);
    }

    pub fn add_station(&self, station: Station) {
        let mut current_stations = self.stations.get_untracked();
        current_stations.push(station);
        self.set_stations.set(current_stations);
    }

    pub fn update_station(&self, updated_station: Station) {
        let current_stations = self.stations.get_untracked();
        let updated_stations: Vec<Station> = current_stations
            .into_iter()
            .map(|station| {
                if station.id == updated_station.id {
                    updated_station.clone()
                } else {
                    station
                }
            })
            .collect();
        self.set_stations.set(updated_stations);
    }

    pub fn remove_station(&self, station_id: &str) {
        let current_stations = self.stations.get_untracked();
        let filtered_stations: Vec<Station> = current_stations
            .into_iter()
            .filter(|station| station.id != station_id)
            .collect();
        self.set_stations.set(filtered_stations);
    }
}

pub fn provide() -> StationState {
    let station_state = StationState::new();
    provide_context(station_state.clone());
    station_state
}

pub fn get() -> StationState {
    expect_context::<StationState>()
}