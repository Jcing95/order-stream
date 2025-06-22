use leptos::prelude::*;
use leptos::logging;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemePreference {
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResolvedTheme {
    Light,
    Dark,
}

impl ThemePreference {
    pub fn cycle(self) -> Self {
        match self {
            ThemePreference::System => ThemePreference::Light,
            ThemePreference::Light => ThemePreference::Dark,
            ThemePreference::Dark => ThemePreference::System,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            ThemePreference::System => "system",
            ThemePreference::Light => "light",
            ThemePreference::Dark => "dark",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "light" => ThemePreference::Light,
            "dark" => ThemePreference::Dark,
            _ => ThemePreference::System,
        }
    }
}

impl ResolvedTheme {
    pub fn is_dark(self) -> bool {
        matches!(self, ResolvedTheme::Dark)
    }
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub preference: RwSignal<ThemePreference>,
    pub resolved: Signal<ResolvedTheme>,
}

impl ThemeState {
    pub fn new() -> Self {
        // Start with system preference
        let preference = RwSignal::new(ThemePreference::System);
        
        // Initialize system_dark - will be updated by client-side effect if available
        let system_dark = RwSignal::new(false);

        // Resolve the actual theme based on preference and system setting
        let resolved = Signal::derive(move || {
            let pref = preference.get();
            let sys_dark = system_dark.get();
            let result = match pref {
                ThemePreference::Light => ResolvedTheme::Light,
                ThemePreference::Dark => ResolvedTheme::Dark,
                ThemePreference::System => {
                    if sys_dark {
                        ResolvedTheme::Dark
                    } else {
                        ResolvedTheme::Light
                    }
                }
            };
            
            logging::log!("Theme resolved: pref={:?}, sys_dark={}, result={:?}", pref, sys_dark, result);            
            result
        });

        // Only run client-side effects when hydrating
        #[cfg(feature = "hydrate")]
        Effect::new({
            let preference = preference;
            let system_dark = system_dark;
            move |_| {
                if let Some(window) = web_sys::window() {
                    // Check for saved preference in localStorage
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(saved_preference)) = storage.get_item("theme-preference") {
                            let saved_preference = ThemePreference::from_string(&saved_preference);
                            preference.set(saved_preference);
                        }
                    }

                    // Check system preference
                    if let Ok(media_query_list) = window.match_media("(prefers-color-scheme: dark)") {
                        if let Some(mql) = media_query_list {
                            system_dark.set(mql.matches());
                        }
                    }
                }
            }
        });

        // Apply theme class to document root
        Effect::new({
            let resolved = resolved;
            move |_| {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(html) = document.document_element() {
                            let class_list = html.class_list();
                            match resolved.get() {
                                ResolvedTheme::Dark => {
                                    logging::log!("Applying dark class to document");
                                    let _ = class_list.add_1("dark");
                                }
                                ResolvedTheme::Light => {
                                    logging::log!("Removing dark class from document");
                                    let _ = class_list.remove_1("dark");
                                }
                            }
                        }
                    }
                }
            }
        });

        Self { preference, resolved }
    }

    pub fn cycle_preference(&self) {
        let current = self.preference.get();
        let new_preference = current.cycle();
        
        logging::log!("Theme cycling from: {:?}", current);
        logging::log!("Theme cycling to: {:?}", new_preference);
        
        self.preference.set(new_preference);
        
        // Save to localStorage (client-side only)
        #[cfg(feature = "hydrate")]
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme-preference", new_preference.to_string());
            }
        }
    }

    pub fn is_dark(&self) -> Signal<bool> {
        let resolved = self.resolved;
        Signal::derive(move || resolved.get().is_dark())
    }

    pub fn preference_label(&self) -> Signal<&'static str> {
        let preference = self.preference;
        Signal::derive(move || {
            match preference.get() {
                ThemePreference::System => "Auto",
                ThemePreference::Light => "Light",
                ThemePreference::Dark => "Dark",
            }
        })
    }
}
