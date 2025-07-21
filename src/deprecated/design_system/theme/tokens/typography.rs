use super::super::Size;

/// Trait for token structs that can be indexed by Size
pub trait FromSize {
    fn from_size(&self, size: Size) -> &'static str;
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