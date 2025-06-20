use leptos::prelude::*;
use crate::frontend::state::theme::{ThemeState, Theme};

#[component]
pub fn ThemeToggle(theme_state: ThemeState) -> impl IntoView {
    view! {
        <button
            class=move || {
                if theme_state.current.get() == Theme::Dark {
                    "relative inline-flex h-8 w-14 items-center rounded-full transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-900 bg-gradient-to-r from-indigo-600 to-purple-600 shadow-lg hover:shadow-xl transform hover:scale-105"
                } else {
                    "relative inline-flex h-8 w-14 items-center rounded-full transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-50 bg-gradient-to-r from-blue-400 to-cyan-400 shadow-lg hover:shadow-xl transform hover:scale-105"
                }
            }
            on:click=move |_| theme_state.toggle()
            title="Toggle dark mode"
        >
            <span class=move || {
                if theme_state.current.get() == Theme::Dark {
                    "translate-x-7 inline-block h-6 w-6 transform rounded-full bg-white shadow-lg transition-all duration-300 ring-2 ring-purple-300"
                } else {
                    "translate-x-1 inline-block h-6 w-6 transform rounded-full bg-white shadow-lg transition-all duration-300 ring-2 ring-blue-300"
                }
            }>
                <span class="sr-only">"Toggle dark mode"</span>
                <div class="flex items-center justify-center h-full w-full">
                    {move || {
                        if theme_state.current.get() == Theme::Dark {
                            view! {
                                <svg class="h-4 w-4 text-gray-700" fill="currentColor" viewBox="0 0 20 20">
                                    <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg class="h-4 w-4 text-yellow-500" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path>
                                </svg>
                            }.into_any()
                        }
                    }}
                </div>
            </span>
        </button>
    }
}