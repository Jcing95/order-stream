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

// Design System Utilities
// Centralized styling patterns for consistent UI

/// Primary button styling with consistent hover/focus states
pub fn button_primary() -> &'static str {
    "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-lg \
     bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 \
     dark:bg-blue-500 dark:hover:bg-blue-600 dark:focus:ring-blue-400 dark:focus:ring-offset-gray-900 \
     transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
}

/// Secondary button styling
pub fn button_secondary() -> &'static str {
    "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-lg \
     bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 \
     dark:bg-gray-800 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-700 dark:focus:ring-blue-400 dark:focus:ring-offset-gray-900 \
     transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
}

/// Danger/destructive button styling
pub fn button_danger() -> &'static str {
    "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-lg \
     bg-red-600 text-white hover:bg-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 \
     dark:bg-red-500 dark:hover:bg-red-600 dark:focus:ring-red-400 dark:focus:ring-offset-gray-900 \
     transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
}

/// Small button variant
pub fn button_small() -> &'static str {
    "px-3 py-1.5 text-xs"
}

/// Large button variant
pub fn button_large() -> &'static str {
    "px-6 py-3 text-base"
}

/// Card/surface background styling
pub fn card_surface() -> &'static str {
    "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-xl shadow-sm"
}

/// Elevated card styling (with more shadow)
pub fn card_elevated() -> &'static str {
    "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-xl shadow-lg"
}

/// Input field styling
pub fn input_field() -> &'static str {
    "block w-full px-3 py-2 text-sm border rounded-lg \
     bg-white text-gray-900 placeholder-gray-500 border-gray-300 \
     dark:bg-gray-700 dark:border-gray-500 dark:text-gray-100 dark:placeholder-gray-400 \
     focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:focus:ring-blue-400 dark:focus:border-blue-400 \
     transition-colors duration-200"
}

/// Label styling for forms
pub fn label_text() -> &'static str {
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
}

/// Primary text color
pub fn text_primary() -> &'static str {
    "text-gray-900 dark:text-gray-100"
}

/// Secondary text color
pub fn text_secondary() -> &'static str {
    "text-gray-600 dark:text-gray-400"
}

/// Muted text color
pub fn text_muted() -> &'static str {
    "text-gray-500 dark:text-gray-500"
}

/// Success text color
pub fn text_success() -> &'static str {
    "text-green-700 dark:text-green-300"
}

/// Error text color
pub fn text_error() -> &'static str {
    "text-red-700 dark:text-red-300"
}

/// Warning text color
pub fn text_warning() -> &'static str {
    "text-yellow-700 dark:text-yellow-300"
}

/// Page background
pub fn page_background() -> &'static str {
    "min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200"
}

/// Tab navigation styling
pub fn tab_nav() -> &'static str {
    "border-b border-gray-200 dark:border-gray-600"
}

/// Active tab styling
pub fn tab_active() -> &'static str {
    "inline-flex items-center px-4 py-2 text-sm font-medium border-b-2 border-blue-600 text-blue-600 \
     dark:border-blue-400 dark:text-blue-400"
}

/// Inactive tab styling
pub fn tab_inactive() -> &'static str {
    "inline-flex items-center px-4 py-2 text-sm font-medium border-b-2 border-transparent \
     text-gray-500 hover:text-gray-700 hover:border-gray-300 \
     dark:text-gray-400 dark:hover:text-gray-200 dark:hover:border-gray-500 \
     transition-colors duration-200"
}

/// Alert/notification base styling
pub fn alert_base() -> &'static str {
    "p-4 rounded-xl border shadow-sm"
}

/// Success alert styling
pub fn alert_success() -> &'static str {
    "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-700 text-green-700 dark:text-green-300"
}

/// Error alert styling
pub fn alert_error() -> &'static str {
    "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-700 text-red-700 dark:text-red-300"
}

/// Warning alert styling
pub fn alert_warning() -> &'static str {
    "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-700 text-yellow-700 dark:text-yellow-300"
}

/// Info alert styling
pub fn alert_info() -> &'static str {
    "bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-700 text-blue-700 dark:text-blue-300"
}

/// Loading spinner styling
pub fn spinner() -> &'static str {
    "animate-spin rounded-full h-5 w-5 border-2 border-gray-200 border-t-blue-600 dark:border-gray-600 dark:border-t-blue-400"
}

/// Icon color for interactive elements
pub fn icon_interactive() -> &'static str {
    "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
}

/// Gradient text styling for headers
pub fn text_gradient() -> &'static str {
    "bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400"
}

