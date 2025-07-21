pub mod tokens;
pub mod variants;
pub mod presets;

pub use tokens::*;
pub use variants::*;

use leptos::prelude::*;

/// Main theme struct that can be partially overridden by users
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub typography: TypographyTokens,
    pub borders: BorderTokens,
    pub shadows: ShadowTokens,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {

    /// Create theme builder for custom themes
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::new()
    }

    /// Apply theme overrides while keeping defaults for unspecified values
    pub fn with_overrides(mut self, overrides: ThemeOverrides) -> Self {
        if let Some(colors) = overrides.colors {
            self.colors = self.colors.merge(colors);
        }
        if let Some(spacing) = overrides.spacing {
            self.spacing = self.spacing.merge(spacing);
        }
        if let Some(typography) = overrides.typography {
            self.typography = self.typography.merge(typography);
        }
        if let Some(borders) = overrides.borders {
            self.borders = self.borders.merge(borders);
        }
        if let Some(shadows) = overrides.shadows {
            self.shadows = self.shadows.merge(shadows);
        }
        self
    }
}

/// Builder for creating custom themes
#[derive(Default)]
pub struct ThemeBuilder {
    name: Option<&'static str>,
    overrides: ThemeOverrides,
}

impl ThemeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn colors(mut self, colors: ColorTokensOverride) -> Self {
        self.overrides.colors = Some(colors);
        self
    }

    pub fn spacing(mut self, spacing: SpacingTokensOverride) -> Self {
        self.overrides.spacing = Some(spacing);
        self
    }

    pub fn typography(mut self, typography: TypographyTokensOverride) -> Self {
        self.overrides.typography = Some(typography);
        self
    }

    pub fn borders(mut self, borders: BorderTokensOverride) -> Self {
        self.overrides.borders = Some(borders);
        self
    }

    pub fn shadows(mut self, shadows: ShadowTokensOverride) -> Self {
        self.overrides.shadows = Some(shadows);
        self
    }

    pub fn build(self) -> Theme {
        let mut theme = Theme::default().with_overrides(self.overrides);
        if let Some(name) = self.name {
            theme.name = name;
        }
        theme
    }
}

/// Partial theme overrides
#[derive(Default, Clone, Debug)]
pub struct ThemeOverrides {
    pub colors: Option<ColorTokensOverride>,
    pub spacing: Option<SpacingTokensOverride>,
    pub typography: Option<TypographyTokensOverride>,
    pub borders: Option<BorderTokensOverride>,
    pub shadows: Option<ShadowTokensOverride>,
}

/// Theme preference options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemePreference {
    System,
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

/// Theme context for providing theme to components
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub preference: RwSignal<ThemePreference>,
    pub system_dark: RwSignal<bool>,
}

impl ThemeContext {
    pub fn new(initial_theme: Theme) -> Self {
        let preference = RwSignal::new(ThemePreference::System);
        let system_dark = RwSignal::new(false);
        let theme = RwSignal::new(initial_theme);
        
        Self {
            theme,
            preference,
            system_dark,
        }
    }

    /// Provide theme context with enhanced functionality
    pub fn provide(initial_theme: Theme) {
        let context = ThemeContext::new(initial_theme);
        
        // Set up reactive theme resolution
        let _theme_signal = context.theme;
        let _preference_signal = context.preference;
        let _system_dark_signal = context.system_dark;
        
        // Resolve theme based on preference and system setting
        #[cfg(feature = "hydrate")]
        Effect::new(move |_| {
            let pref = context.preference.get();
            let sys_dark = context.system_dark.get();
            let resolved_theme = match pref {
                ThemePreference::Light => Theme::light(),
                ThemePreference::Dark => Theme::dark(),
                ThemePreference::System => {
                    if sys_dark {
                        Theme::dark()
                    } else {
                        Theme::light()
                    }
                }
            };
            context.theme.set(resolved_theme);
        });
        
        // Initialize from localStorage and system preferences (client-side only)
        #[cfg(feature = "hydrate")]
        Effect::new({
            let preference = context.preference;
            let system_dark = context.system_dark;
            move |_| {
                if let Some(window) = web_sys::window() {
                    // Load saved preference from localStorage
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
        #[cfg(feature = "hydrate")]
        Effect::new({
            let theme = context.theme;
            move |_| {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(html) = document.document_element() {
                            let class_list = html.class_list();
                            let current_theme = theme.get();
                            if current_theme.name == "dark" {
                                let _ = class_list.add_1("dark");
                            } else {
                                let _ = class_list.remove_1("dark");
                            }
                        }
                    }
                }
            }
        });
        
        provide_context(context);
    }

    /// Get the current theme from context (reactive)
    pub fn use_theme() -> Signal<Theme> {
        let context = use_context::<ThemeContext>()
            .expect("Theme context not found. Make sure to call ThemeContext::provide() in your app root.");
        Signal::derive(move || context.theme.get())
    }

    /// Get theme preference from context (reactive)
    pub fn use_preference() -> Signal<ThemePreference> {
        let context = use_context::<ThemeContext>()
            .expect("Theme context not found. Make sure to call ThemeContext::provide() in your app root.");
        Signal::derive(move || context.preference.get())
    }

    /// Get if current theme is dark (reactive)
    pub fn is_dark() -> Signal<bool> {
        let context = use_context::<ThemeContext>()
            .expect("Theme context not found. Make sure to call ThemeContext::provide() in your app root.");
        Signal::derive(move || context.theme.get().name == "dark")
    }

    /// Switch to a different theme preference
    pub fn set_preference(preference: ThemePreference) {
        if let Some(context) = use_context::<ThemeContext>() {
            context.preference.set(preference);
            
            // Save to localStorage (client-side only)
            #[cfg(feature = "hydrate")]
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme-preference", preference.to_string());
                }
            }
        }
    }

    /// Cycle through theme preferences
    pub fn cycle_preference() {
        if let Some(context) = use_context::<ThemeContext>() {
            let current = context.preference.get();
            let new_preference = current.cycle();
            Self::set_preference(new_preference);
        }
    }

    /// Get preference label for UI
    pub fn preference_label() -> Signal<&'static str> {
        let context = use_context::<ThemeContext>()
            .expect("Theme context not found. Make sure to call ThemeContext::provide() in your app root.");
        Signal::derive(move || {
            match context.preference.get() {
                ThemePreference::System => "Auto",
                ThemePreference::Light => "Light",
                ThemePreference::Dark => "Dark",
            }
        })
    }

    /// Switch to a different theme (direct theme change)
    pub fn set_theme(theme: Theme) {
        if let Some(context) = use_context::<ThemeContext>() {
            context.theme.set(theme);
        }
    }
}


