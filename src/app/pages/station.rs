use crate::app::components::station_view::StationView;
use crate::app::components::stations::Stations;
use crate::backend::station::get_station;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
// Route handler for dynamic station URLs /stations/:name
#[derive(Params, PartialEq, Clone)]
struct StationParams {
    id: String,
}

#[component]
pub fn StationPage() -> impl IntoView {
    let params = use_params::<StationParams>();

    // Create a resource to fetch the station
    let station_resource = Resource::new(
        move || params.with(|params| params.clone()),
        |params| async move {
            if params.is_err() {
                return None;
            }
            get_station(params.unwrap().id).await.ok()
        },
    );

    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                <Suspense fallback=move || view! {
                    <div class="flex items-center justify-center py-12">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
                    </div>
                }>
                    {move || {
                        station_resource.get().map(|station_opt| {
                            match station_opt {
                                Some(station) => view! {
                                    <div>
                                        <div class="mb-8">
                                            <h1 class="text-3xl font-bold text-text mb-2">{station.name.clone()}</h1>
                                            <p class="text-text-muted">"Manage items for this station"</p>
                                        </div>
                                        <StationView station_id=station.id.clone() />
                                    </div>
                                }.into_any(),
                                None => view! {
                                    <div>
                                        <div class="mb-8">
                                            <h1 class="text-3xl font-bold text-text mb-2">"Station Not Found"</h1>
                                            <p class="text-text-muted">"The requested station does not exist. Choose from the available stations below:"</p>
                                        </div>
                                        <Stations />
                                    </div>
                                }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
