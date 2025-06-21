/// Component variant system
/// Defines the different variant types that components can use

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Intent {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ComponentState {
    #[default]
    Enabled,
    Disabled,
    Loading,
}

impl Size {
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::Xs => "xs",
            Size::Sm => "sm", 
            Size::Md => "md",
            Size::Lg => "lg",
            Size::Xl => "xl",
        }
    }
}

impl Intent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Intent::Primary => "primary",
            Intent::Secondary => "secondary",
            Intent::Success => "success",
            Intent::Danger => "danger",
            Intent::Warning => "warning",
            Intent::Info => "info",
        }
    }
}

impl ComponentState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentState::Enabled => "enabled",
            ComponentState::Disabled => "disabled",
            ComponentState::Loading => "loading",
        }
    }

    pub fn get_modifier_classes(&self) -> &'static str {
        match self {
            ComponentState::Enabled => "",
            ComponentState::Disabled => "opacity-50 cursor-not-allowed pointer-events-none",
            ComponentState::Loading => "opacity-75 cursor-wait",
        }
    }
}

