use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{use_interval, UseIntervalReturn};

#[component]
pub fn ConnectionIndicator(#[prop(into)] state: Signal<ConnectionReadyState>) -> impl IntoView {
    let emoji = "⚡";
    #[cfg(feature = "hydrate")]
    // Get a signal that increments every second
    let UseIntervalReturn { counter, .. } = use_interval(4000);

    // Map the counter to our rotating emoji
    #[cfg(feature = "hydrate")]
    let emoji = move || {
        let i = (counter.get() % 2) as usize;
        match state.get() {
            ConnectionReadyState::Connecting => ["⚡", "🌀"][i],
            ConnectionReadyState::Open => ["🍕", "🍻"][i],
            ConnectionReadyState::Closing => ["🔕", "⌛"][i],
            ConnectionReadyState::Closed => ["❌", "💤"][i],
        }
    };

    let indicator = move || match state.get() {
        ConnectionReadyState::Connecting => "🟡",
        ConnectionReadyState::Open => "",
        ConnectionReadyState::Closing => "🟠",
        ConnectionReadyState::Closed => "🔴",
    };

    view! {
        <div class="ml-1 scale-75 animate-pulse">
            {indicator}{emoji}
        </div>
    }
}
