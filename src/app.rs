use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use crate::frontend::pages::admin::AdminPage;
use crate::frontend::pages::home::Home;
use crate::frontend::pages::design_system::DesignSystemPage;
use crate::frontend::state::theme::ThemeState;
use crate::frontend::design_system::{Theme, ThemeContext};

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
    
    // Initialize the old theme state system (for compatibility)
    let theme_state = ThemeState::new();
    provide_context(theme_state);

    // Initialize design system theme based on old theme state
    let initial_theme = if theme_state.is_dark().get_untracked() {
        Theme::dark()
    } else {
        Theme::light()
    };
    ThemeContext::provide(initial_theme);

    // Sync the old theme state with the design system theme
    Effect::new(move |_| {
        let is_dark = theme_state.is_dark().get();
        let new_theme = if is_dark {
            Theme::dark()
        } else {
            Theme::light()
        };
        ThemeContext::set_theme(new_theme);
    });

    // Create reactive page background based on design system theme
    let page_bg_class = Signal::derive(move || {
        let theme = ThemeContext::use_theme().get();
        format!("min-h-screen transition-colors duration-200 {}", theme.colors.background.page)
    });

    view! {
        <div class=move || page_bg_class.get()>
            <Router>
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=Home/>
                    <Route path=StaticSegment("admin") view=AdminPage/>
                    <Route path=StaticSegment("design-system") view=DesignSystemPage/>
                </FlatRoutes>
            </Router>
        </div>
    }
}
