use leptos::prelude::*;
use leptos_router::components::A;
use crate::app::states::{category, station};

#[component]
fn StationCard(station: crate::common::types::Station) -> impl IntoView {
    let category_state = category::get();
    let categories = category_state.get_categories();
    
    let station_id = station.id.clone();
    let station_name = station.name.clone();
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
        <A 
            href=format!("/station/{}", station_id)
            attr:class="block p-6 bg-surface rounded-lg border border-border hover:border-primary hover:shadow-lg transition-all duration-200"
        >
            <div class="flex items-center justify-between mb-3">
                <h3 class="text-xl font-semibold text-text">{station_name}</h3>
                <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                </svg>
            </div>
            
            <div class="space-y-2 text-sm text-text-muted">
                <div>
                    <span class="font-medium">"Categories: "</span>
                    {move || {
                        let names = category_names();
                        if names.is_empty() {
                            "None".to_string()
                        } else {
                            names
                        }
                    }}
                </div>
                <div>
                    <span class="font-medium">"Status Flow: "</span>
                    {move || status_display()}
                </div>
            </div>
        </A>
    }
}

#[component]
pub fn Stations() -> impl IntoView {
    let station_state = station::get();
    let stations = station_state.get_stations();

    view! {
        <div class="bg-surface rounded-lg border border-border p-6">
            <h2 class="text-xl font-semibold text-text mb-6">"Available Stations"</h2>

            <Show
                when=move || !stations.get().is_empty()
                fallback=|| view! {
                    <div class="text-center py-12">
                        <svg class="mx-auto h-12 w-12 text-text-muted mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                        </svg>
                        <p class="text-text-muted text-lg">"No stations available"</p>
                        <p class="text-text-muted text-sm mt-2">"Contact an administrator to set up stations"</p>
                    </div>
                }
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <For
                        each=move || stations.get()
                        key=|station| station.id.clone()
                        children=move |station| {
                            view! {
                                <StationCard station=station />
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}