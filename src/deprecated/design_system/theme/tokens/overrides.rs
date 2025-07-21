use super::color::*;
use super::spacing::*;
use super::typography::*;
use super::border::*;
use super::shadow::*;

/// Partial override types for theme customization
#[derive(Default, Clone, Debug)]
pub struct ColorTokensOverride {
    pub text: Option<TextColors>,
    pub background: Option<BackgroundColors>,
    pub border: Option<BorderColors>,
}

#[derive(Default, Clone, Debug)]
pub struct SpacingTokensOverride {
    pub raw: Option<RawSpacing>,
    pub px: Option<XSpacing>,
    pub py: Option<YSpacing>,
    pub pt: Option<TopSpacing>,
    pub pb: Option<BottomSpacing>,
    pub pl: Option<LeftSpacing>,
    pub pr: Option<RightSpacing>,
    pub mx: Option<XMarginSpacing>,
    pub my: Option<YMarginSpacing>,
    pub mt: Option<TopMarginSpacing>,
    pub mb: Option<BottomMarginSpacing>,
    pub ml: Option<LeftMarginSpacing>,
    pub mr: Option<RightMarginSpacing>,
    pub gap: Option<GapSpacing>,
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
        if let Some(raw) = override_val.raw { self.raw = raw; }
        if let Some(px) = override_val.px { self.px = px; }
        if let Some(py) = override_val.py { self.py = py; }
        if let Some(pt) = override_val.pt { self.pt = pt; }
        if let Some(pb) = override_val.pb { self.pb = pb; }
        if let Some(pl) = override_val.pl { self.pl = pl; }
        if let Some(pr) = override_val.pr { self.pr = pr; }
        if let Some(mx) = override_val.mx { self.mx = mx; }
        if let Some(my) = override_val.my { self.my = my; }
        if let Some(mt) = override_val.mt { self.mt = mt; }
        if let Some(mb) = override_val.mb { self.mb = mb; }
        if let Some(ml) = override_val.ml { self.ml = ml; }
        if let Some(mr) = override_val.mr { self.mr = mr; }
        if let Some(gap) = override_val.gap { self.gap = gap; }
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