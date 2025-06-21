use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use crate::frontend::pages::admin::AdminPage;
use crate::frontend::pages::home::Home;
use crate::frontend::state::theme::{ThemeState, page_background};
use crate::frontend::design_system::{Theme, ThemeContext, ColorTokensOverride, TextColors, BackgroundColors, BorderColors};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/order-stream.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    let theme_state = ThemeState::new();
    provide_context(theme_state);

    // Define custom themes
    let light_theme = Theme::default();
    
    let dark_theme = Theme::builder()
        .name("dark")
        .colors(ColorTokensOverride {
            text: Some(TextColors {
                primary: "text-gray-100",
                secondary: "text-gray-300",
                muted: "text-gray-400",
                success: "text-green-300",
                danger: "text-red-300",
                warning: "text-yellow-300",
                info: "text-blue-300",
            }),
            background: Some(BackgroundColors {
                primary: "bg-blue-500",
                primary_hover: "hover:bg-blue-600",
                secondary: "bg-gray-700",
                secondary_hover: "hover:bg-gray-600",
                success: "bg-green-500",
                success_hover: "hover:bg-green-600",
                danger: "bg-red-500",
                danger_hover: "hover:bg-red-600",
                warning: "bg-yellow-500",
                warning_hover: "hover:bg-yellow-600",
                info: "bg-blue-500",
                info_hover: "hover:bg-blue-600",
                page: "bg-gray-900",
                surface: "bg-gray-800",
                elevated: "bg-gray-700",
            }),
            border: Some(BorderColors {
                default: "border-gray-600",
                muted: "border-gray-700",
                focus: "focus:border-blue-400",
                success: "border-green-600",
                danger: "border-red-600",
                warning: "border-yellow-600",
                info: "border-blue-600",
            }),
        })
        .build();

    // Provide design system theme context
    ThemeContext::provide(light_theme.clone());
    
    // Store themes in context for components to use
    provide_context((light_theme, dark_theme));

    view! {
        <div class=page_background()>
            <Router>
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=Home/>
                    <Route path=StaticSegment("admin") view=AdminPage/>
                </FlatRoutes>
            </Router>
        </div>
    }
}
