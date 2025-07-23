use leptos::prelude::*;
use leptos_router::components::A;
use crate::app::states::station;
use crate::app::components::atoms::icons;

#[component]
fn StationCard(station: crate::common::types::Station) -> impl IntoView {
    let station_id = station.id.clone();
    let station_name = station.name.clone();

    view! {
        <A 
            href=format!("/station/{}", station_id)
            attr:class="block p-6 bg-surface rounded-lg border border-border hover:border-primary hover:shadow-lg transition-all duration-200"
        >
            <div class="flex items-center justify-between mb-3">
                <h3 class="text-xl font-semibold text-text">{station_name}</h3>
                <icons::ChevronRight attr:class="w-5 h-5 text-primary"/>
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
                        <icons::Building attr:class="mx-auto h-12 w-12 text-text-muted mb-4"/>
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