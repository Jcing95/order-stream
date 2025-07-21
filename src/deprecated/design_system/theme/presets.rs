/// Predefined theme presets (light and dark)
use super::{
    Theme, ColorTokens, SpacingTokens, TypographyTokens, BorderTokens, ShadowTokens,
    TextColors, BackgroundColors, BorderColors, FocusRingColors,
};

impl Theme {
    /// Create a light theme with default light color palette
    pub fn light() -> Self {
        Self {
            name: "light",
            colors: light_color_tokens(),
            spacing: SpacingTokens::default(),
            typography: TypographyTokens::default(),
            borders: BorderTokens::default(),
            shadows: ShadowTokens::default(),
        }
    }

    /// Create a dark theme with default dark color palette
    pub fn dark() -> Self {
        Self {
            name: "dark",
            colors: dark_color_tokens(),
            spacing: SpacingTokens::default(),
            typography: TypographyTokens::default(),
            borders: BorderTokens::default(),
            shadows: ShadowTokens::default(),
        }
    }
}

/// Light theme color tokens - Modern 2024 design with warm, accessible colors
fn light_color_tokens() -> ColorTokens {
    ColorTokens {
        text: TextColors {
            primary: "text-slate-900",      // Rich, modern black
            secondary: "text-slate-700",    // Softer secondary text
            muted: "text-slate-500",        // Subtle muted text
            success: "text-emerald-700",    // Modern emerald green
            danger: "text-rose-700",        // Warm rose red
            warning: "text-amber-700",      // Rich amber
            info: "text-sky-700",           // Clear sky blue
            
            // Code/inline elements
            code: "text-slate-800",         // Slightly darker for code
            
            // Alert text colors for light theme
            alert_success: "text-emerald-800",
            alert_danger: "text-rose-800", 
            alert_warning: "text-amber-800",
            alert_info: "text-sky-800",
        },
        background: BackgroundColors {
            primary: "bg-indigo-500",           // Softer indigo primary
            primary_hover: "hover:bg-indigo-600",
            secondary: "bg-slate-200",          // Light secondary with subtle warmth
            secondary_hover: "hover:bg-slate-300",
            success: "bg-emerald-500",          // Muted emerald
            success_hover: "hover:bg-emerald-600",
            danger: "bg-rose-500",              // Muted rose
            danger_hover: "hover:bg-rose-600",
            warning: "bg-amber-400",            // Softer amber
            warning_hover: "hover:bg-amber-500",
            info: "bg-sky-500",                 // Muted sky
            info_hover: "hover:bg-sky-600",
            
            page: "bg-slate-50",                // Warm neutral background
            surface: "bg-white",                // Pure white surfaces
            elevated: "bg-white",               // Clean elevated surfaces
            
            // Code/inline elements
            code: "bg-slate-100",               // Light code background
            
            // Alert backgrounds for light theme
            alert_success: "bg-emerald-50",
            alert_danger: "bg-rose-50",
            alert_warning: "bg-amber-50", 
            alert_info: "bg-sky-50",
        },
        border: BorderColors {
            default: "border-slate-300",        // Subtle modern borders
            muted: "border-slate-200",          // Very subtle borders
            focus: "focus:border-indigo-500",   // Match primary color
            success: "border-emerald-300",      // Success state borders
            danger: "border-rose-300",          // Error state borders
            warning: "border-amber-300",        // Warning state borders
            info: "border-sky-300",             // Info state borders
        },
        focus_ring: FocusRingColors {
            primary: "focus:ring-indigo-500",   // Primary focus ring
            secondary: "focus:ring-slate-500",  // Secondary focus ring
            success: "focus:ring-emerald-500",  // Success focus ring
            danger: "focus:ring-rose-500",      // Danger focus ring
            warning: "focus:ring-amber-500",    // Warning focus ring
            info: "focus:ring-sky-500",         // Info focus ring
        },
    }
}

/// Dark theme color tokens - Modern 2024 design with comfortable, accessible dark colors
fn dark_color_tokens() -> ColorTokens {
    ColorTokens {
        text: TextColors {
            primary: "text-slate-100",      // Soft white, easier on eyes than pure white
            secondary: "text-slate-300",    // Comfortable secondary text
            muted: "text-slate-400",        // Subtle muted text
            success: "text-emerald-400",    // Desaturated emerald for dark mode
            danger: "text-rose-400",        // Softer rose for dark mode
            warning: "text-amber-400",      // Warm amber for dark mode
            info: "text-sky-400",           // Gentle sky blue
            
            // Code/inline elements
            code: "text-slate-200",         // Light code text for dark theme
            
            // Alert text colors for dark theme
            alert_success: "text-emerald-200",
            alert_danger: "text-rose-200",
            alert_warning: "text-amber-200", 
            alert_info: "text-sky-200",
        },
        background: BackgroundColors {
            primary: "bg-indigo-600",           // Muted indigo for dark mode
            primary_hover: "hover:bg-indigo-500",
            secondary: "bg-slate-700",          // Modern dark secondary
            secondary_hover: "hover:bg-slate-600",
            success: "bg-emerald-600",          // Muted emerald for dark mode
            success_hover: "hover:bg-emerald-500",
            danger: "bg-rose-600",              // Muted rose for dark mode
            danger_hover: "hover:bg-rose-500",
            warning: "bg-amber-600",            // Muted amber for dark mode
            warning_hover: "hover:bg-amber-500",
            info: "bg-sky-600",                 // Muted sky for dark mode
            info_hover: "hover:bg-sky-500",
            
            page: "bg-slate-950",               // Deep, comfortable dark background
            surface: "bg-slate-900",            // Card surfaces with subtle warmth
            elevated: "bg-slate-800",           // Elevated elements like modals
            
            // Code/inline elements
            code: "bg-slate-800",               // Dark code background
            
            // Alert backgrounds for dark theme
            alert_success: "bg-emerald-900/30",
            alert_danger: "bg-rose-900/30",
            alert_warning: "bg-amber-900/30",
            alert_info: "bg-sky-900/30",
        },
        border: BorderColors {
            default: "border-slate-700",        // Subtle borders that don't overpower
            muted: "border-slate-800",          // Very subtle borders
            focus: "focus:border-indigo-400",   // Bright focus for accessibility
            success: "border-emerald-500",      // Success state borders
            danger: "border-rose-500",          // Error state borders
            warning: "border-amber-500",        // Warning state borders
            info: "border-sky-500",             // Info state borders
        },
        focus_ring: FocusRingColors {
            primary: "focus:ring-indigo-400",   // Bright primary focus ring for dark mode
            secondary: "focus:ring-slate-400",  // Secondary focus ring for dark mode
            success: "focus:ring-emerald-400",  // Success focus ring for dark mode
            danger: "focus:ring-rose-400",      // Danger focus ring for dark mode
            warning: "focus:ring-amber-400",    // Warning focus ring for dark mode
            info: "focus:ring-sky-400",         // Info focus ring for dark mode
        },
    }
}