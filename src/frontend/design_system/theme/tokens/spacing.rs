use super::super::Size;

/// Trait for token structs that can be indexed by Size
pub trait FromSize {
    fn from_size(&self, size: Size) -> &'static str;
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

// Default implementations
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

// FromSize implementations
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