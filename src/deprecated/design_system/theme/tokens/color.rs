use super::super::Intent;

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