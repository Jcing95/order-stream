
#[derive(Clone, Debug)]
pub struct ShadowTokens {
    pub none: &'static str,   // shadow-none
    pub sm: &'static str,     // shadow-sm
    pub md: &'static str,     // shadow-md
    pub lg: &'static str,     // shadow-lg
    pub xl: &'static str,     // shadow-xl
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