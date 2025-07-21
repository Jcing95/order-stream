pub mod color;
pub mod spacing;
pub mod typography;
pub mod border;
pub mod shadow;
pub mod overrides;

// Re-export everything from the individual modules
pub use color::*;
pub use spacing::{SpacingTokens, RawSpacing, XSpacing, YSpacing, TopSpacing, BottomSpacing, LeftSpacing, RightSpacing, XMarginSpacing, YMarginSpacing, TopMarginSpacing, BottomMarginSpacing, LeftMarginSpacing, RightMarginSpacing, GapSpacing, FromSize};
pub use typography::{TypographyTokens};
pub use border::{BorderTokens, BorderWidthTokens, BorderRadiusTokens};
pub use shadow::*;
pub use overrides::*;

// Implement FromSize for types that need it
use super::super::Size;

impl FromSize for TypographyTokens {
    fn from_size(&self, size: Size) -> &'static str {
        typography::FromSize::from_size(self, size)
    }
}

impl FromSize for BorderRadiusTokens {
    fn from_size(&self, size: Size) -> &'static str {
        border::FromSize::from_size(self, size)
    }
}