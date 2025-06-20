use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub current: RwSignal<Theme>,
}

impl ThemeState {
    pub fn new() -> Self {
        // Start with light theme, will be updated during hydration
        let theme = RwSignal::new(Theme::Light);

        // Only run client-side effects when hydrating
        #[cfg(feature = "hydrate")]
        Effect::new({
            let theme = theme;
            move |_| {
                if let Some(window) = web_sys::window() {
                    // Check for saved theme in localStorage
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(saved_theme)) = storage.get_item("theme") {
                            let saved_theme = Theme::from_string(&saved_theme);
                            theme.set(saved_theme);
                            return;
                        }
                    }

                    // No saved theme, check system preference
                    if let Ok(media_query_list) = window.match_media("(prefers-color-scheme: dark)") {
                        if let Some(mql) = media_query_list {
                            if mql.matches() {
                                theme.set(Theme::Dark);
                            }
                        }
                    }
                }
            }
        });

        Self { current: theme }
    }

    pub fn toggle(&self) {
        let new_theme = self.current.get().toggle();
        self.current.set(new_theme);
        
        // Save to localStorage (client-side only)
        #[cfg(feature = "hydrate")]
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", new_theme.to_string());
            }
        }
    }

    pub fn is_dark(&self) -> Signal<bool> {
        let current = self.current.read_only();
        Signal::derive(move || current.get() == Theme::Dark)
    }
}

