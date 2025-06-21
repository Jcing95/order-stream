use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use crate::frontend::design_system::{Button, ThemeContext, Theme, Size, Intent};

#[component]
pub fn Home() -> impl IntoView {
    // Get themes from context
    let (light_theme, dark_theme) = use_context::<(Theme, Theme)>()
        .expect("Themes not found in context");
    view! {
        <Title text="Order Stream Demo"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-col items-center justify-center flex-1 space-y-8">
                    <h1 class="text-4xl font-bold">"Order Stream"</h1>
                    <p class="text-xl text-center max-w-md">
                        "Streamlining food and drink logistics at small events"
                    </p>
                    
                    <div class="flex flex-col space-y-4">
                        <A href="/admin">
                            <div class="bg-white text-blue-800 px-6 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors">
                                "Admin Panel"
                            </div>
                        </A>
                        
                        // Theme test section
                        <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 space-y-4">
                            <h3 class="text-lg font-semibold">"Design System Test"</h3>
                            
                            <div class="flex space-x-2">
                                <Button 
                                    size=Size::Md
                                    intent=Intent::Primary
                                    on_click=Callback::new(move |_| {
                                        ThemeContext::set_theme(light_theme.clone());
                                    })
                                >
                                    "Light"
                                </Button>
                                
                                <Button 
                                    size=Size::Md
                                    intent=Intent::Secondary
                                    on_click=Callback::new(move |_| {
                                        ThemeContext::set_theme(dark_theme.clone());
                                    })
                                >
                                    "Dark"
                                </Button>
                            </div>
                            
                            <div class="flex space-x-2">
                                <Button size=Size::Sm intent=Intent::Secondary>
                                    "Small"
                                </Button>
                                <Button size=Size::Md intent=Intent::Success>
                                    "Success"
                                </Button>
                                <Button size=Size::Lg intent=Intent::Danger>
                                    "Danger"
                                </Button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}