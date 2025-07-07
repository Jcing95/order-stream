use leptos::prelude::*;
use crate::common::types::{Station, Category, CreateStationRequest};
use crate::frontend::components::{station_form::StationForm, station_list::StationList};

#[component]
pub fn StationSection<F1, F2>(
    stations: ReadSignal<Vec<Station>>,
    categories: ReadSignal<Vec<Category>>,
    on_submit: F1,
    on_delete: F2,
) -> impl IntoView
where
    F1: Fn(CreateStationRequest) + 'static + Clone + Send + Sync,
    F2: Fn(String) + 'static + Clone + Send + Sync,
{
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div>
                <StationForm categories=categories.into() on_submit=on_submit />
            </div>
            <div>
                <StationList stations=stations categories=categories on_delete=on_delete />
            </div>
        </div>
    }
}