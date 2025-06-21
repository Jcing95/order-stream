/// Design System Module
/// 
/// This module provides a comprehensive design system following Atomic Design principles:
/// - **Atoms**: Basic building blocks (Button, Input, Text, etc.)
/// - **Molecules**: Combinations of atoms (FormField, SearchBar, Card, etc.)
/// - **Theme**: Theming system with design tokens and variants
/// 
/// The design system is built with:
/// - Token-based theming with light/dark mode support
/// - Variant system (size, intent, state) for consistent styling
/// - Tailwind CSS classes (no custom CSS)
/// - Leptos components for type safety and reactivity
/// 
/// # Usage
/// 
/// ```rust
/// use leptos::prelude::*;
/// use crate::frontend::design_system::{*, theme::*};
/// 
/// #[component]
/// fn MyApp() -> impl IntoView {
///     // Provide theme context
///     ThemeContext::provide(Theme::light());
///     
///     view! {
///         <div class="p-4">
///             <Button 
///                 size=Size::Lg 
///                 intent=Intent::Primary
///                 on_click=move |_| {
///                     // Handle click
///                 }
///             >
///                 "Click me"
///             </Button>
///             <Input 
///                 placeholder="Enter text"
///                 size=Size::Md
///                 state=ComponentState::Enabled
///             />
///         </div>
///     }
/// }
/// ```

pub mod theme;
pub mod atoms;
pub mod molecules;

// Re-export everything for convenience
pub use theme::*;
pub use atoms::*;
// pub use molecules::*;