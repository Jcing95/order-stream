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
    pub focus_ring: FocusRingColors,
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
    
    // Code/inline elements
    pub code: &'static str,
    
    // Alert text colors (for use on alert backgrounds)
    pub alert_success: &'static str,
    pub alert_danger: &'static str,
    pub alert_warning: &'static str,
    pub alert_info: &'static str,
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
    
    // Code/inline elements
    pub code: &'static str,
    
    // Alert backgrounds
    pub alert_success: &'static str,
    pub alert_danger: &'static str,
    pub alert_warning: &'static str,
    pub alert_info: &'static str,
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
pub struct FocusRingColors {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub success: &'static str,
    pub danger: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
}

#[derive(Clone, Debug)]
pub struct SpacingTokens {
    pub raw: RawSpacing,
    pub px: XSpacing,
    pub py: YSpacing,
    pub pt: TopSpacing,
    pub pb: BottomSpacing,
    pub pl: LeftSpacing,
    pub pr: RightSpacing,
    pub mx: XMarginSpacing,
    pub my: YMarginSpacing,
    pub mt: TopMarginSpacing,
    pub mb: BottomMarginSpacing,
    pub ml: LeftMarginSpacing,
    pub mr: RightMarginSpacing,
    pub gap: GapSpacing,
}

#[derive(Clone, Debug)]
pub struct RawSpacing {
    pub xs: &'static str,     // 1
    pub sm: &'static str,     // 2  
    pub md: &'static str,     // 4
    pub lg: &'static str,     // 6
    pub xl: &'static str,     // 8
    pub xl2: &'static str,    // 12
    pub xl3: &'static str,    // 16
}

#[derive(Clone, Debug)]
pub struct XSpacing {
    pub xs: &'static str,     // px-1
    pub sm: &'static str,     // px-2
    pub md: &'static str,     // px-4
    pub lg: &'static str,     // px-6
    pub xl: &'static str,     // px-8
    pub xl2: &'static str,    // px-12
    pub xl3: &'static str,    // px-16
}

#[derive(Clone, Debug)]
pub struct YSpacing {
    pub xs: &'static str,     // py-1
    pub sm: &'static str,     // py-2
    pub md: &'static str,     // py-4
    pub lg: &'static str,     // py-6
    pub xl: &'static str,     // py-8
    pub xl2: &'static str,    // py-12
    pub xl3: &'static str,    // py-16
}

#[derive(Clone, Debug)]
pub struct TopSpacing {
    pub xs: &'static str,     // pt-1
    pub sm: &'static str,     // pt-2
    pub md: &'static str,     // pt-4
    pub lg: &'static str,     // pt-6
    pub xl: &'static str,     // pt-8
    pub xl2: &'static str,    // pt-12
    pub xl3: &'static str,    // pt-16
}

#[derive(Clone, Debug)]
pub struct BottomSpacing {
    pub xs: &'static str,     // pb-1
    pub sm: &'static str,     // pb-2
    pub md: &'static str,     // pb-4
    pub lg: &'static str,     // pb-6
    pub xl: &'static str,     // pb-8
    pub xl2: &'static str,    // pb-12
    pub xl3: &'static str,    // pb-16
}

#[derive(Clone, Debug)]
pub struct LeftSpacing {
    pub xs: &'static str,     // pl-1
    pub sm: &'static str,     // pl-2
    pub md: &'static str,     // pl-4
    pub lg: &'static str,     // pl-6
    pub xl: &'static str,     // pl-8
    pub xl2: &'static str,    // pl-12
    pub xl3: &'static str,    // pl-16
}

#[derive(Clone, Debug)]
pub struct RightSpacing {
    pub xs: &'static str,     // pr-1
    pub sm: &'static str,     // pr-2
    pub md: &'static str,     // pr-4
    pub lg: &'static str,     // pr-6
    pub xl: &'static str,     // pr-8
    pub xl2: &'static str,    // pr-12
    pub xl3: &'static str,    // pr-16
}

#[derive(Clone, Debug)]
pub struct XMarginSpacing {
    pub xs: &'static str,     // mx-1
    pub sm: &'static str,     // mx-2
    pub md: &'static str,     // mx-4
    pub lg: &'static str,     // mx-6
    pub xl: &'static str,     // mx-8
    pub xl2: &'static str,    // mx-12
    pub xl3: &'static str,    // mx-16
}

#[derive(Clone, Debug)]
pub struct YMarginSpacing {
    pub xs: &'static str,     // my-1
    pub sm: &'static str,     // my-2
    pub md: &'static str,     // my-4
    pub lg: &'static str,     // my-6
    pub xl: &'static str,     // my-8
    pub xl2: &'static str,    // my-12
    pub xl3: &'static str,    // my-16
}

#[derive(Clone, Debug)]
pub struct TopMarginSpacing {
    pub xs: &'static str,     // mt-1
    pub sm: &'static str,     // mt-2
    pub md: &'static str,     // mt-4
    pub lg: &'static str,     // mt-6
    pub xl: &'static str,     // mt-8
    pub xl2: &'static str,    // mt-12
    pub xl3: &'static str,    // mt-16
}

#[derive(Clone, Debug)]
pub struct BottomMarginSpacing {
    pub xs: &'static str,     // mb-1
    pub sm: &'static str,     // mb-2
    pub md: &'static str,     // mb-4
    pub lg: &'static str,     // mb-6
    pub xl: &'static str,     // mb-8
    pub xl2: &'static str,    // mb-12
    pub xl3: &'static str,    // mb-16
}

#[derive(Clone, Debug)]
pub struct LeftMarginSpacing {
    pub xs: &'static str,     // ml-1
    pub sm: &'static str,     // ml-2
    pub md: &'static str,     // ml-4
    pub lg: &'static str,     // ml-6
    pub xl: &'static str,     // ml-8
    pub xl2: &'static str,    // ml-12
    pub xl3: &'static str,    // ml-16
}

#[derive(Clone, Debug)]
pub struct RightMarginSpacing {
    pub xs: &'static str,     // mr-1
    pub sm: &'static str,     // mr-2
    pub md: &'static str,     // mr-4
    pub lg: &'static str,     // mr-6
    pub xl: &'static str,     // mr-8
    pub xl2: &'static str,    // mr-12
    pub xl3: &'static str,    // mr-16
}

#[derive(Clone, Debug)]
pub struct GapSpacing {
    pub xs: &'static str,     // gap-1
    pub sm: &'static str,     // gap-2
    pub md: &'static str,     // gap-4
    pub lg: &'static str,     // gap-6
    pub xl: &'static str,     // gap-8
    pub xl2: &'static str,    // gap-12
    pub xl3: &'static str,    // gap-16
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
            focus_ring: FocusRingColors::default(),
        }
    }
}

impl Default for FocusRingColors {
    fn default() -> Self {
        Self {
            primary: "focus:ring-blue-500",
            secondary: "focus:ring-gray-500",
            success: "focus:ring-green-500",
            danger: "focus:ring-red-500",
            warning: "focus:ring-yellow-500",
            info: "focus:ring-blue-500",
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
            
            // Code/inline elements
            code: "text-gray-800",
            
            // Default alert text colors
            alert_success: "text-green-800",
            alert_danger: "text-red-800",
            alert_warning: "text-yellow-800",
            alert_info: "text-blue-800",
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
            
            // Code/inline elements
            code: "bg-gray-100",
            
            // Default alert backgrounds
            alert_success: "bg-green-50",
            alert_danger: "bg-red-50",
            alert_warning: "bg-yellow-50",
            alert_info: "bg-blue-50",
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
            raw: RawSpacing::default(),
            px: XSpacing::default(),
            py: YSpacing::default(),
            pt: TopSpacing::default(),
            pb: BottomSpacing::default(),
            pl: LeftSpacing::default(),
            pr: RightSpacing::default(),
            mx: XMarginSpacing::default(),
            my: YMarginSpacing::default(),
            mt: TopMarginSpacing::default(),
            mb: BottomMarginSpacing::default(),
            ml: LeftMarginSpacing::default(),
            mr: RightMarginSpacing::default(),
            gap: GapSpacing::default(),
        }
    }
}

impl Default for RawSpacing {
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

impl Default for XSpacing {
    fn default() -> Self {
        Self {
            xs: "px-1",
            sm: "px-2",
            md: "px-4",
            lg: "px-6",
            xl: "px-8",
            xl2: "px-12",
            xl3: "px-16",
        }
    }
}

impl Default for YSpacing {
    fn default() -> Self {
        Self {
            xs: "py-1",
            sm: "py-2",
            md: "py-4",
            lg: "py-6",
            xl: "py-8",
            xl2: "py-12",
            xl3: "py-16",
        }
    }
}

impl Default for TopSpacing {
    fn default() -> Self {
        Self {
            xs: "pt-1",
            sm: "pt-2",
            md: "pt-4",
            lg: "pt-6",
            xl: "pt-8",
            xl2: "pt-12",
            xl3: "pt-16",
        }
    }
}

impl Default for BottomSpacing {
    fn default() -> Self {
        Self {
            xs: "pb-1",
            sm: "pb-2",
            md: "pb-4",
            lg: "pb-6",
            xl: "pb-8",
            xl2: "pb-12",
            xl3: "pb-16",
        }
    }
}

impl Default for LeftSpacing {
    fn default() -> Self {
        Self {
            xs: "pl-1",
            sm: "pl-2",
            md: "pl-4",
            lg: "pl-6",
            xl: "pl-8",
            xl2: "pl-12",
            xl3: "pl-16",
        }
    }
}

impl Default for RightSpacing {
    fn default() -> Self {
        Self {
            xs: "pr-1",
            sm: "pr-2",
            md: "pr-4",
            lg: "pr-6",
            xl: "pr-8",
            xl2: "pr-12",
            xl3: "pr-16",
        }
    }
}

impl Default for XMarginSpacing {
    fn default() -> Self {
        Self {
            xs: "mx-1",
            sm: "mx-2",
            md: "mx-4",
            lg: "mx-6",
            xl: "mx-8",
            xl2: "mx-12",
            xl3: "mx-16",
        }
    }
}

impl Default for YMarginSpacing {
    fn default() -> Self {
        Self {
            xs: "my-1",
            sm: "my-2",
            md: "my-4",
            lg: "my-6",
            xl: "my-8",
            xl2: "my-12",
            xl3: "my-16",
        }
    }
}

impl Default for TopMarginSpacing {
    fn default() -> Self {
        Self {
            xs: "mt-1",
            sm: "mt-2",
            md: "mt-4",
            lg: "mt-6",
            xl: "mt-8",
            xl2: "mt-12",
            xl3: "mt-16",
        }
    }
}

impl Default for BottomMarginSpacing {
    fn default() -> Self {
        Self {
            xs: "mb-1",
            sm: "mb-2",
            md: "mb-4",
            lg: "mb-6",
            xl: "mb-8",
            xl2: "mb-12",
            xl3: "mb-16",
        }
    }
}

impl Default for LeftMarginSpacing {
    fn default() -> Self {
        Self {
            xs: "ml-1",
            sm: "ml-2",
            md: "ml-4",
            lg: "ml-6",
            xl: "ml-8",
            xl2: "ml-12",
            xl3: "ml-16",
        }
    }
}

impl Default for RightMarginSpacing {
    fn default() -> Self {
        Self {
            xs: "mr-1",
            sm: "mr-2",
            md: "mr-4",
            lg: "mr-6",
            xl: "mr-8",
            xl2: "mr-12",
            xl3: "mr-16",
        }
    }
}

impl Default for GapSpacing {
    fn default() -> Self {
        Self {
            xs: "gap-1",
            sm: "gap-2",
            md: "gap-4",
            lg: "gap-6",
            xl: "gap-8",
            xl2: "gap-12",
            xl3: "gap-16",
        }
    }
}

impl FromSize for SpacingTokens {
    fn from_size(&self, size: Size) -> &'static str {
        self.raw.from_size(size)
    }
}

impl FromSize for RawSpacing {
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

impl FromSize for XSpacing {
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

impl FromSize for YSpacing {
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