/// Component variant system
/// Defines the different variant types that components can use

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Intent {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ComponentState {
    #[default]
    Enabled,
    Disabled,
    Loading,
}

impl Size {
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::Xs => "xs",
            Size::Sm => "sm", 
            Size::Md => "md",
            Size::Lg => "lg",
            Size::Xl => "xl",
        }
    }
}

impl Intent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Intent::Primary => "primary",
            Intent::Secondary => "secondary",
            Intent::Success => "success",
            Intent::Danger => "danger",
            Intent::Warning => "warning",
            Intent::Info => "info",
        }
    }
}

impl ComponentState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentState::Enabled => "enabled",
            ComponentState::Disabled => "disabled",
            ComponentState::Loading => "loading",
        }
    }

    pub fn get_modifier_classes(&self) -> &'static str {
        match self {
            ComponentState::Enabled => "",
            ComponentState::Disabled => "opacity-50 cursor-not-allowed pointer-events-none",
            ComponentState::Loading => "opacity-75 cursor-wait",
        }
    }
}

/// Helper trait for components to build classes from variants
pub trait VariantBuilder {
    fn build_classes(&self, size: Size, intent: Intent, state: ComponentState) -> String;
}

/// Utility functions for common variant patterns
pub mod utils {
    use super::*;
    use crate::frontend::design_system::theme::tokens::*;

    /// Get padding classes based on size
    pub fn get_padding_classes(size: Size, spacing: &SpacingTokens) -> String {
        let (x, y) = match size {
            Size::Xs => (spacing.xs, spacing.xs),
            Size::Sm => (spacing.sm, spacing.xs),
            Size::Md => (spacing.md, spacing.sm),
            Size::Lg => (spacing.lg, spacing.md),
            Size::Xl => (spacing.xl, spacing.lg),
        };
        format!("px-{} py-{}", x, y)
    }

    /// Get typography classes based on size
    pub fn get_typography_classes(size: Size, typography: &TypographyTokens) -> &'static str {
        match size {
            Size::Xs => typography.xs,
            Size::Sm => typography.sm,
            Size::Md => typography.base,
            Size::Lg => typography.lg,
            Size::Xl => typography.xl,
        }
    }

    /// Get border radius classes based on size
    pub fn get_border_radius_classes(size: Size, borders: &BorderTokens) -> &'static str {
        match size {
            Size::Xs => borders.radius.sm,
            Size::Sm => borders.radius.md,
            Size::Md => borders.radius.md,
            Size::Lg => borders.radius.lg,
            Size::Xl => borders.radius.xl,
        }
    }

    /// Get background color classes based on intent
    pub fn get_background_classes(intent: Intent, colors: &BackgroundColors) -> (&'static str, &'static str) {
        match intent {
            Intent::Primary => (colors.primary, colors.primary_hover),
            Intent::Secondary => (colors.secondary, colors.secondary_hover),
            Intent::Success => (colors.success, colors.success_hover),
            Intent::Danger => (colors.danger, colors.danger_hover),
            Intent::Warning => (colors.warning, colors.warning_hover),
            Intent::Info => (colors.info, colors.info_hover),
        }
    }

    /// Get text color classes based on intent
    pub fn get_text_classes(intent: Intent, colors: &TextColors) -> &'static str {
        match intent {
            Intent::Primary => colors.primary,
            Intent::Secondary => colors.secondary,
            Intent::Success => colors.success,
            Intent::Danger => colors.danger,
            Intent::Warning => colors.warning,
            Intent::Info => colors.info,
        }
    }

    /// Get border color classes based on intent
    pub fn get_border_classes(intent: Intent, colors: &BorderColors) -> &'static str {
        match intent {
            Intent::Primary => colors.default,
            Intent::Secondary => colors.default,
            Intent::Success => colors.success,
            Intent::Danger => colors.danger,
            Intent::Warning => colors.warning,
            Intent::Info => colors.info,
        }
    }

    /// Combine multiple class strings, filtering out empty ones
    pub fn combine_classes(classes: &[&str]) -> String {
        classes
            .iter()
            .filter(|&&s| !s.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::design_system::theme::tokens::*;

    #[test]
    fn test_size_variants() {
        assert_eq!(Size::Md.as_str(), "md");
        assert_eq!(Size::Lg.as_str(), "lg");
    }

    #[test]
    fn test_intent_variants() {
        assert_eq!(Intent::Primary.as_str(), "primary");
        assert_eq!(Intent::Danger.as_str(), "danger");
    }

    #[test]
    fn test_state_variants() {
        assert_eq!(ComponentState::Enabled.as_str(), "enabled");
        assert_eq!(ComponentState::Disabled.get_modifier_classes(), "opacity-50 cursor-not-allowed pointer-events-none");
    }

    #[test]
    fn test_padding_classes() {
        let spacing = SpacingTokens::default();
        let classes = utils::get_padding_classes(Size::Md, &spacing);
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
    }

    #[test]
    fn test_combine_classes() {
        let combined = utils::combine_classes(&["bg-blue-600", "", "hover:bg-blue-700", "px-4"]);
        assert_eq!(combined, "bg-blue-600 hover:bg-blue-700 px-4");
    }
}