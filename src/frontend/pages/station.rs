use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
// Station type is used in StationView component prop
use crate::frontend::components::{
    station_view::{StationView, StationViewMode},
    loading::LoadingSpinner
};
use crate::frontend::design_system::{
    Text, Alert, Card, CardVariant, Button,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};
use crate::backend::services::stations::{get_station_by_name, get_stations};
use crate::common::types::Station;

// New component for database-driven station routing
#[component]
pub fn DynamicStationPage(
    station_name: String,
) -> impl IntoView {
    let station_name_clone = station_name.clone();
    
    // Load station configuration from database - using proper Resource pattern
    let station_resource = Resource::new(
        move || station_name.clone(),
        |station_name| get_station_by_name(station_name)
    );

    view! {
        <Suspense fallback=move || view! { <LoadingSpinner /> }>
            {move || {
                match station_resource.get() {
                    Some(Ok(station)) => {
                        view! {
                            <StationView 
                                station=station
                                view_mode=StationViewMode::OrderCards
                            />
                        }.into_any()
                    },
                    Some(Err(e)) => {
                        let error_name = station_name_clone.clone();
                        view! {
                            <div class="container mx-auto p-6">
                                <Alert intent=Intent::Danger size=Size::Lg>
                                    <div class="space-y-2">
                                        <Text 
                                            variant=TextVariant::Heading 
                                            size=Size::Lg 
                                            weight=FontWeight::Bold
                                        >
                                            "Station Not Found"
                                        </Text>
                                        <Text variant=TextVariant::Body size=Size::Md>
                                            "Could not load station '" {error_name} "': " {e.to_string()}
                                        </Text>
                                        <Text variant=TextVariant::Body size=Size::Sm>
                                            "Please contact an administrator to set up this station."
                                        </Text>
                                    </div>
                                </Alert>
                            </div>
                        }.into_any()
                    },
                    None => {
                        view! { <LoadingSpinner /> }.into_any()
                    }
                }
            }}
        </Suspense>
    }
}
#[component]
pub fn StationsOverviewPage() -> impl IntoView {
    // Load all stations from database - using proper Resource pattern
    let stations_resource = Resource::new(|| (), |_| get_stations());

    view! {
        <div class="container mx-auto p-6">
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-8">
                    <Text 
                        variant=TextVariant::Heading 
                        size=Size::Xl 
                        weight=FontWeight::Bold
                        as_element="h1"
                    >
                        "Order Stations"
                    </Text>
                    <Text 
                        variant=TextVariant::Body 
                        size=Size::Lg 
                        intent=Intent::Secondary
                        class="mt-4"
                    >
                        "Select your station to view and manage orders"
                    </Text>
                </div>

                <Suspense fallback=move || view! { <LoadingSpinner /> }>
                    {move || {
                        match stations_resource.get() {
                            Some(Ok(stations)) => {
                                if stations.is_empty() {
                                    view! {
                                        <div class="text-center">
                                            <Alert intent=Intent::Info size=Size::Lg>
                                                <div class="space-y-4">
                                                    <Text 
                                                        variant=TextVariant::Heading 
                                                        size=Size::Lg 
                                                        weight=FontWeight::Bold
                                                    >
                                                        "No Stations Configured"
                                                    </Text>
                                                    <Text 
                                                        variant=TextVariant::Body 
                                                        size=Size::Md
                                                    >
                                                        "No stations have been set up yet. Contact an administrator to configure stations."
                                                    </Text>
                                                    <Button
                                                        size=Size::Md
                                                        intent=Intent::Primary
                                                        on_click=Callback::new(|_| {
                                                            let navigate = use_navigate();
                                                            navigate("/admin", Default::default());
                                                        })
                                                    >
                                                        "Go to Admin Panel"
                                                    </Button>
                                                </div>
                                            </Alert>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                            {stations.into_iter().map(|station| {
                                                view! {
                                                    <DatabaseStationCard station=station />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            },
                            Some(Err(_)) => {
                                view! {
                                    <div class="text-center">
                                        <Alert intent=Intent::Danger size=Size::Lg>
                                            <div class="space-y-2">
                                                <Text 
                                                    variant=TextVariant::Heading 
                                                    size=Size::Lg 
                                                    weight=FontWeight::Bold
                                                >
                                                    "Error Loading Stations"
                                                </Text>
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Md
                                                >
                                                    "Unable to load station configuration. Please try again or contact an administrator."
                                                </Text>
                                            </div>
                                        </Alert>
                                    </div>
                                }.into_any()
                            },
                            None => {
                                view! { <LoadingSpinner /> }.into_any()
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn DatabaseStationCard(
    station: Station,
) -> impl IntoView {
    let station_url = format!("/stations/{}", station.name.to_lowercase().replace(" ", "-"));
    let station_name = station.name.clone();
    
    // Create a description based on the station configuration
    let description = format!(
        "Handles {} • Shows {} orders",
        if station.category_ids.is_empty() { 
            "all categories".to_string() 
        } else { 
            format!("{} categories", station.category_ids.len()) 
        },
        station.input_statuses.len()
    );

    view! {
        <Card variant=CardVariant::Default>
            <a 
                href=station_url
                class="block p-6 transition-colors duration-200"
            >
                <div class="flex items-center mb-4">
                    <span class="text-3xl mr-3">"🏪"</span>
                    <Text 
                        variant=TextVariant::Heading 
                        size=Size::Lg 
                        weight=FontWeight::Semibold
                    >
                        {station_name}
                    </Text>
                </div>
                <Text 
                    variant=TextVariant::Body 
                    size=Size::Md 
                    intent=Intent::Secondary
                    class="mb-3"
                >
                    {description}
                </Text>
                <div class="flex items-center justify-between">
                    <Text 
                        variant=TextVariant::Caption 
                        size=Size::Sm 
                        intent=Intent::Secondary
                    >
                        "Click to open station"
                    </Text>
                    <Text 
                        variant=TextVariant::Body 
                        size=Size::Md 
                        intent=Intent::Info
                        as_element="span"
                    >
                        "→"
                    </Text>
                </div>
            </a>
        </Card>
    }
}