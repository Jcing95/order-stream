/// Design tokens for the theme system
/// Uses nested structure for better organization and partial overrides

use super::{Size, Intent};

/// Trait for token structs that can be indexed by Size
pub trait FromSize {
    fn from_size(&self, size: Size) -> &'static str;
}

/// Trait for token structs that can be indexed by Intent
pub trait FromIntent {
    fn from_intent(&self, intent: Intent) -> &'static str;
}

#[derive(Clone, Debug)]
pub struct ColorTokens {
    pub text: TextColors,
    pub background: BackgroundColors,
    pub border: BorderColors,
}

#[derive(Clone, Debug)]
pub struct TextColors {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub muted: &'static str,
    pub success: &'static str,
    pub danger: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
}

#[derive(Clone, Debug)]
pub struct BackgroundColors {
    // Intent backgrounds with hover states
    pub primary: &'static str,
    pub primary_hover: &'static str,
    pub secondary: &'static str,
    pub secondary_hover: &'static str,
    pub success: &'static str,
    pub success_hover: &'static str,
    pub danger: &'static str,
    pub danger_hover: &'static str,
    pub warning: &'static str,
    pub warning_hover: &'static str,
    pub info: &'static str,
    pub info_hover: &'static str,
    
    // Surface backgrounds
    pub page: &'static str,
    pub surface: &'static str,
    pub elevated: &'static str,
}

#[derive(Clone, Debug)]
pub struct BorderColors {
    pub default: &'static str,
    pub muted: &'static str,
    pub focus: &'static str,
    pub success: &'static str,
    pub danger: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
}

#[derive(Clone, Debug)]
pub struct SpacingTokens {
    pub xs: &'static str,     // 1
    pub sm: &'static str,     // 2  
    pub md: &'static str,     // 4
    pub lg: &'static str,     // 6
    pub xl: &'static str,     // 8
    pub xl2: &'static str,    // 12
    pub xl3: &'static str,    // 16
}

#[derive(Clone, Debug)]
pub struct TypographyTokens {
    pub xs: &'static str,     // text-xs
    pub sm: &'static str,     // text-sm
    pub base: &'static str,   // text-base
    pub lg: &'static str,     // text-lg
    pub xl: &'static str,     // text-xl
    pub xl2: &'static str,    // text-2xl
    pub xl3: &'static str,    // text-3xl
}

#[derive(Clone, Debug)]
pub struct BorderTokens {
    pub width: BorderWidthTokens,
    pub radius: BorderRadiusTokens,
}

#[derive(Clone, Debug)]
pub struct BorderWidthTokens {
    pub none: &'static str,   // border-0
    pub thin: &'static str,   // border
    pub thick: &'static str,  // border-2
}

#[derive(Clone, Debug)]
pub struct BorderRadiusTokens {
    pub none: &'static str,   // rounded-none
    pub sm: &'static str,     // rounded-sm
    pub md: &'static str,     // rounded-md
    pub lg: &'static str,     // rounded-lg
    pub xl: &'static str,     // rounded-xl
    pub full: &'static str,   // rounded-full
}

#[derive(Clone, Debug)]
pub struct ShadowTokens {
    pub none: &'static str,   // shadow-none
    pub sm: &'static str,     // shadow-sm
    pub md: &'static str,     // shadow-md
    pub lg: &'static str,     // shadow-lg
    pub xl: &'static str,     // shadow-xl
}

// Default implementations (Light theme)
impl Default for ColorTokens {
    fn default() -> Self {
        Self {
            text: TextColors::default(),
            background: BackgroundColors::default(),
            border: BorderColors::default(),
        }
    }
}

impl Default for TextColors {
    fn default() -> Self {
        Self {
            primary: "text-gray-900",
            secondary: "text-gray-700", 
            muted: "text-gray-500",
            success: "text-green-700",
            danger: "text-red-700",
            warning: "text-yellow-700",
            info: "text-blue-700",
        }
    }
}

impl FromIntent for TextColors {
    fn from_intent(&self, intent: Intent) -> &'static str {
        match intent {
            Intent::Primary => self.primary,
            Intent::Secondary => self.secondary,
            Intent::Success => self.success,
            Intent::Danger => self.danger,
            Intent::Warning => self.warning,
            Intent::Info => self.info,
        }
    }
}

impl Default for BackgroundColors {
    fn default() -> Self {
        Self {
            primary: "bg-blue-600",
            primary_hover: "hover:bg-blue-700",
            secondary: "bg-gray-600",
            secondary_hover: "hover:bg-gray-700",
            success: "bg-green-600",
            success_hover: "hover:bg-green-700",
            danger: "bg-red-600",
            danger_hover: "hover:bg-red-700",
            warning: "bg-yellow-600",
            warning_hover: "hover:bg-yellow-700",
            info: "bg-blue-600",
            info_hover: "hover:bg-blue-700",
            
            page: "bg-gray-50",
            surface: "bg-white",
            elevated: "bg-white",
        }
    }
}

impl FromIntent for BackgroundColors {
    fn from_intent(&self, intent: Intent) -> &'static str {
        match intent {
            Intent::Primary => self.primary,
            Intent::Secondary => self.secondary,
            Intent::Success => self.success,
            Intent::Danger => self.danger,
            Intent::Warning => self.warning,
            Intent::Info => self.info,
        }
    }
}

impl Default for BorderColors {
    fn default() -> Self {
        Self {
            default: "border-gray-300",
            muted: "border-gray-200",
            focus: "focus:border-blue-500",
            success: "border-green-300",
            danger: "border-red-300",
            warning: "border-yellow-300",
            info: "border-blue-300",
        }
    }
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xs: "1",
            sm: "2",
            md: "4",
            lg: "6",
            xl: "8",
            xl2: "12",
            xl3: "16",
        }
    }
}

impl FromSize for SpacingTokens {
    fn from_size(&self, size: Size) -> &'static str {
        match size {
            Size::Xs => self.xs,
            Size::Sm => self.sm,
            Size::Md => self.md,
            Size::Lg => self.lg,
            Size::Xl => self.xl,
        }
    }
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            xs: "text-xs",
            sm: "text-sm",
            base: "text-base",
            lg: "text-lg",
            xl: "text-xl",
            xl2: "text-2xl",
            xl3: "text-3xl",
        }
    }
}

impl FromSize for TypographyTokens {
    fn from_size(&self, size: Size) -> &'static str {
        match size {
            Size::Xs => self.xs,
            Size::Sm => self.sm,
            Size::Md => self.base,
            Size::Lg => self.lg,
            Size::Xl => self.xl,
        }
    }
}

impl Default for BorderTokens {
    fn default() -> Self {
        Self {
            width: BorderWidthTokens::default(),
            radius: BorderRadiusTokens::default(),
        }
    }
}

impl Default for BorderWidthTokens {
    fn default() -> Self {
        Self {
            none: "border-0",
            thin: "border",
            thick: "border-2",
        }
    }
}

impl Default for BorderRadiusTokens {
    fn default() -> Self {
        Self {
            none: "rounded-none",
            sm: "rounded-sm",
            md: "rounded-md",
            lg: "rounded-lg",
            xl: "rounded-xl",
            full: "rounded-full",
        }
    }
}

impl FromSize for BorderRadiusTokens {
    fn from_size(&self, size: Size) -> &'static str {
        match size {
            Size::Xs => self.sm,
            Size::Sm => self.md,
            Size::Md => self.md,
            Size::Lg => self.lg,
            Size::Xl => self.xl,
        }
    }
}

impl Default for ShadowTokens {
    fn default() -> Self {
        Self {
            none: "shadow-none",
            sm: "shadow-sm",
            md: "shadow-md",
            lg: "shadow-lg",
            xl: "shadow-xl",
        }
    }
}

/// Partial override types for theme customization
#[derive(Default, Clone, Debug)]
pub struct ColorTokensOverride {
    pub text: Option<TextColors>,
    pub background: Option<BackgroundColors>,
    pub border: Option<BorderColors>,
}

#[derive(Default, Clone, Debug)]
pub struct SpacingTokensOverride {
    pub xs: Option<&'static str>,
    pub sm: Option<&'static str>,
    pub md: Option<&'static str>,
    pub lg: Option<&'static str>,
    pub xl: Option<&'static str>,
    pub xl2: Option<&'static str>,
    pub xl3: Option<&'static str>,
}

#[derive(Default, Clone, Debug)]
pub struct TypographyTokensOverride {
    pub xs: Option<&'static str>,
    pub sm: Option<&'static str>,
    pub base: Option<&'static str>,
    pub lg: Option<&'static str>,
    pub xl: Option<&'static str>,
    pub xl2: Option<&'static str>,
    pub xl3: Option<&'static str>,
}

#[derive(Default, Clone, Debug)]
pub struct BorderTokensOverride {
    pub width: Option<BorderWidthTokens>,
    pub radius: Option<BorderRadiusTokens>,
}

#[derive(Default, Clone, Debug)]
pub struct ShadowTokensOverride {
    pub none: Option<&'static str>,
    pub sm: Option<&'static str>,
    pub md: Option<&'static str>,
    pub lg: Option<&'static str>,
    pub xl: Option<&'static str>,
}

// Merge implementations for partial overrides
impl ColorTokens {
    pub fn merge(mut self, override_val: ColorTokensOverride) -> Self {
        if let Some(text) = override_val.text {
            self.text = text;
        }
        if let Some(background) = override_val.background {
            self.background = background;
        }
        if let Some(border) = override_val.border {
            self.border = border;
        }
        self
    }
}

impl SpacingTokens {
    pub fn merge(mut self, override_val: SpacingTokensOverride) -> Self {
        if let Some(xs) = override_val.xs { self.xs = xs; }
        if let Some(sm) = override_val.sm { self.sm = sm; }
        if let Some(md) = override_val.md { self.md = md; }
        if let Some(lg) = override_val.lg { self.lg = lg; }
        if let Some(xl) = override_val.xl { self.xl = xl; }
        if let Some(xl2) = override_val.xl2 { self.xl2 = xl2; }
        if let Some(xl3) = override_val.xl3 { self.xl3 = xl3; }
        self
    }
}

impl TypographyTokens {
    pub fn merge(mut self, override_val: TypographyTokensOverride) -> Self {
        if let Some(xs) = override_val.xs { self.xs = xs; }
        if let Some(sm) = override_val.sm { self.sm = sm; }
        if let Some(base) = override_val.base { self.base = base; }
        if let Some(lg) = override_val.lg { self.lg = lg; }
        if let Some(xl) = override_val.xl { self.xl = xl; }
        if let Some(xl2) = override_val.xl2 { self.xl2 = xl2; }
        if let Some(xl3) = override_val.xl3 { self.xl3 = xl3; }
        self
    }
}

impl BorderTokens {
    pub fn merge(mut self, override_val: BorderTokensOverride) -> Self {
        if let Some(width) = override_val.width { self.width = width; }
        if let Some(radius) = override_val.radius { self.radius = radius; }
        self
    }
}

impl ShadowTokens {
    pub fn merge(mut self, override_val: ShadowTokensOverride) -> Self {
        if let Some(none) = override_val.none { self.none = none; }
        if let Some(sm) = override_val.sm { self.sm = sm; }
        if let Some(md) = override_val.md { self.md = md; }
        if let Some(lg) = override_val.lg { self.lg = lg; }
        if let Some(xl) = override_val.xl { self.xl = xl; }
        self
    }
}