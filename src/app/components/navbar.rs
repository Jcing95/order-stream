use crate::app::{
    components::{
        atoms::{connection_indicator::ConnectionIndicator, icons::{Moon, OrderStream, Sun, SystemTheme}, user_dropdown::UserDropdown},
        role_gated::RoleGated,
    },
    states::{user, websocket},
};
use crate::common::types::Role;
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
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

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
        <nav class="bg-surface border-b border-border">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Left side - Logo and connection indicator
                    <div class="flex items-center">
                        <A href="/" attr:class="flex items-center space-x-2 text-text hover:text-primary transition-colors">
                            <OrderStream attr:class="size-8 text-primary"/>
                            <span class="font-semibold text-lg">Order Stream</span>
                        </A>
                        <div class="ml-2">
                            <ConnectionIndicator state=websocket.state/>
                        </div>
                        
                        // Desktop Navigation Links
                        <div class="hidden md:flex ml-8 space-x-4">
                            <RoleGated 
                                roles=vec![Role::Admin]
                                children=|| {
                                    view! {
                                        <A 
                                            href="/admin" 
                                            attr:class="text-text hover:text-primary px-3 py-2 rounded-md text-sm font-medium transition-colors"
                                        >
                                            Admin
                                        </A>
                                    }.into_any()
                                }
                            />
                            <RoleGated 
                                roles=vec![Role::Admin, Role::Cashier]
                                children=|| {
                                    view! {
                                        <A 
                                            href="/cashier" 
                                            attr:class="text-text hover:text-primary px-3 py-2 rounded-md text-sm font-medium transition-colors"
                                        >
                                            Kasse
                                        </A>
                                    }.into_any()
                                }
                            />
                            <RoleGated 
                                roles=vec![Role::Admin, Role::Cashier, Role::Staff]
                                children=|| {
                                    view! {
                                        <A 
                                            href="/stations" 
                                            attr:class="text-text hover:text-primary px-3 py-2 rounded-md text-sm font-medium transition-colors"
                                        >
                                            Stationen
                                        </A>
                                    }.into_any()
                                }
                            />
                        </div>
                    </div>

                    // Right side - Theme toggle and user menu
                    <div class="flex items-center space-x-2 sm:space-x-4">
                        <button
                            on:click=toggle_theme
                            class="p-2 text-text-muted hover:text-text transition-colors rounded-md hover:bg-surface-elevated"
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

                        // Desktop User Menu
                        <div class="hidden sm:block">
                            <Show
                                when=move || user.get().is_some()
                                fallback=|| view! {
                                    <A
                                        href="/signin"
                                        attr:class="flex items-center space-x-1 px-3 py-2 text-sm text-text hover:text-primary hover:bg-surface-elevated rounded-md transition-colors"
                                    >
                                        <svg class="w-5 h-5 text-text" fill="currentColor" viewBox="0 0 16 16">
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
                                            <UserDropdown 
                                                user_email=current_user.email.clone()
                                                user_initial=current_user.email.chars().next().unwrap_or('U').to_uppercase().to_string()
                                            />
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }}
                            </Show>
                        </div>

                        // Mobile menu button
                        <div class="md:hidden">
                            <button
                                on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                                class="p-2 rounded-md text-text-muted hover:text-text hover:bg-surface-elevated transition-colors"
                                title="Toggle menu"
                            >
                                // Hamburger icon
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>

                // Mobile Menu
                <div class=move || format!("md:hidden {}", if mobile_menu_open.get() { "block" } else { "hidden" })>
                    <div class="px-2 pt-2 pb-3 space-y-1 bg-surface-elevated border-t border-border">
                        // Mobile Navigation Links
                        <RoleGated 
                            roles=vec![Role::Admin]
                            children=move || {
                                view! {
                                    <A 
                                        href="/admin" 
                                        attr:class="block text-text hover:text-primary hover:bg-surface px-3 py-2 rounded-md text-base font-medium transition-colors"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                    >
                                        Admin
                                    </A>
                                }.into_any()
                            }
                        />
                        <RoleGated 
                            roles=vec![Role::Admin, Role::Cashier]
                            children=move || {
                                view! {
                                    <A 
                                        href="/cashier" 
                                        attr:class="block text-text hover:text-primary hover:bg-surface px-3 py-2 rounded-md text-base font-medium transition-colors"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                    >
                                        Kasse
                                    </A>
                                }.into_any()
                            }
                        />
                        <RoleGated 
                            roles=vec![Role::Admin, Role::Cashier, Role::Staff]
                            children=move || {
                                view! {
                                    <A 
                                        href="/stations" 
                                        attr:class="block text-text hover:text-primary hover:bg-surface px-3 py-2 rounded-md text-base font-medium transition-colors"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                    >
                                        Stationen
                                    </A>
                                }.into_any()
                            }
                        />

                        // Mobile User Menu
                        <div class="border-t border-border pt-2 mt-2">
                            <Show
                                when=move || user.get().is_some()
                                fallback=move || view! {
                                    <A
                                        href="/signin"
                                        attr:class="flex items-center space-x-2 px-3 py-2 text-base text-text hover:text-primary hover:bg-surface rounded-md transition-colors"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                    >
                                        <svg class="w-5 h-5 text-text" fill="currentColor" viewBox="0 0 16 16">
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
                                            <div class="px-3 py-2">
                                                <div class="flex items-center space-x-3">
                                                    <div class="w-8 h-8 bg-primary text-white rounded-full flex items-center justify-center text-sm font-medium">
                                                        {current_user.email.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                                    </div>
                                                    <div class="text-sm text-text">{current_user.email}</div>
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
            </div>
        </nav>
    }
}
