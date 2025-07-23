use leptos::prelude::*;
use crate::app::components::stations::Stations;

#[component]
pub fn StationsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-text mb-2">"Stations"</h1>
                    <p class="text-text-muted">"Select a station to manage orders and items"</p>
                </div>
                
                <Stations />
            </div>
        </div>
    }
}