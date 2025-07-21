use super::super::Size;

/// Trait for token structs that can be indexed by Size
pub trait FromSize {
    fn from_size(&self, size: Size) -> &'static str;
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