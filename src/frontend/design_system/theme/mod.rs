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

/// Theme context for providing theme to components
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
}

impl ThemeContext {
    pub fn new(theme: Theme) -> Self {
        Self {
            theme: RwSignal::new(theme),
        }
    }

    /// Provide theme context to the component tree
    pub fn provide(theme: Theme) {
        provide_context(ThemeContext::new(theme));
    }

    /// Get the current theme from context (reactive)
    pub fn use_theme() -> Signal<Theme> {
        let context = use_context::<ThemeContext>()
            .expect("Theme context not found. Make sure to call ThemeContext::provide() in your app root.");
        Signal::derive(move || context.theme.get())
    }

    /// Get the theme signal for reactive updates
    pub fn use_theme_signal() -> RwSignal<Theme> {
        use_context::<ThemeContext>()
            .expect("Theme context not found. Make sure to call ThemeContext::provide() in your app root.")
            .theme
    }

    /// Switch to a different theme
    pub fn set_theme(theme: Theme) {
        if let Some(context) = use_context::<ThemeContext>() {
            context.theme.set(theme);
        }
    }

}


