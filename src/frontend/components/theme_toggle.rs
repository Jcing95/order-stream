use leptos::prelude::*;
use leptos::logging;
use crate::frontend::state::theme::{ThemeState, button_secondary};

#[component]
pub fn ThemeToggle(theme_state: ThemeState) -> impl IntoView {
    // Extract the signals to avoid closure capture issues
    let is_dark = theme_state.is_dark();
    let preference_label = theme_state.preference_label();
    
    view! {
        <button
            class=format!("{} gap-2", button_secondary())
            on:click=move |_| {
                // Debug logging
                logging::log!("Theme toggle clicked!");
                theme_state.cycle_preference();
            }
            title=move || format!("Theme: {}", preference_label.get())
        >
            // Theme icon
            {move || {
                if is_dark.get() {
                    view! {
                        <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
                        </svg>
                    }.into_any()
                } else {
                    view! {
                        <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path>
                        </svg>
                    }.into_any()
                }
            }}
            
            // Theme label
            <span class="text-sm font-medium">
                {move || preference_label.get()}
            </span>
        </button>
    }
}