use crate::app::{
    components::atoms::{connection_indicator::ConnectionIndicator, icons::{Moon, OrderStream, Sun, SystemTheme}},
    states::{user, websocket},
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        }
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let user_state = user::get();
    let user = user_state.user;

    let (theme, set_theme) = signal(Theme::System);

    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("theme") {
                    set_theme.set(Theme::from_str(&saved_theme));
                }
            }
        }
    });

    // Update document class whenever theme changes
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    html.remove_attribute("class").ok();
                    match theme.get() {
                        Theme::Dark => {
                            html.class_list().add_1("dark").ok();
                        }
                        Theme::Light => {
                            html.class_list().add_1("light").ok();
                        }
                        Theme::System => {
                            if window
                                .match_media("(prefers-color-scheme: dark)")
                                .map(|mq| mq.unwrap().matches())
                                .unwrap_or(false)
                            {
                                html.class_list().add_1("dark").ok();
                            } else {
                                html.class_list().add_1("light").ok();
                            }
                        }
                    }
                }
            }
        }
    });

    let toggle_theme = move |_| {
        set_theme.update(|t| {
            *t = match *t {
                Theme::System => Theme::Light,
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::System,
            }
        });

        // Save to localStorage when theme changes
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                storage.set_item("theme", theme.get().as_str()).ok();
            }
        }
    };

    let websocket = websocket::get();

    view! {
        <Meta name="color-scheme" content=move || theme.get().as_str() />
        <nav class="bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center">
                        <A href="/" attr:class="flex items-center space-x-2 text-gray-900 dark:text-white hover:text-blue-600 dark:hover:text-blue-400 transition-colors">
                            <OrderStream attr:class="size-8 text-primary"/>
                            <span class="font-semibold text-lg">Order Stream</span>
                        </A>
                        <ConnectionIndicator state=websocket.state/>
                    </div>

                    <div class="flex items-center space-x-4">
                        <button
                            on:click=toggle_theme
                            class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors rounded-md hover:bg-gray-100 dark:hover:bg-gray-800"
                            title=move || format!("Theme: {}", match theme.get() {
                                Theme::Light => "Light",
                                Theme::Dark => "Dark",
                                Theme::System => "System",
                            })
                        >
                            {move || match theme.get() {
                                Theme::Light => view!{<Sun/>}.into_any(),
                                Theme::Dark => view!{<Moon/>}.into_any(),
                                Theme::System => view!{<SystemTheme/>}.into_any(),
                            }}
                        </button>

                        <Show
                            when=move || user.get().is_some()
                            fallback=|| view! {
                                <A
                                    href="/signin"
                                    attr:class="flex items-center space-x-1 px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors"
                                >
                                    <svg class="w-5 h-5 text-gray-700 dark:text-gray-300" fill="currentColor" viewBox="0 0 16 16">
                                        <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0"/>
                                        <path fill-rule="evenodd" d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8m8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1"/>
                                    </svg>
                                    <span>Login</span>
                                </A>
                            }
                        >
                            {move || {
                                if let Some(current_user) = user.get() {
                                    view! {
                                        <div class="flex items-center space-x-2">
                                            <span class="text-sm text-gray-700 dark:text-gray-300">
                                                {current_user.email.clone()}
                                            </span>
                                            <div class="w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center">
                                                <span class="text-white text-sm font-medium">
                                                    {current_user.email.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                                </span>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
                        </Show>
                    </div>
                </div>
            </div>
        </nav>
    }
}
