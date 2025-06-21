use leptos::prelude::*;
use crate::frontend::design_system::{Button, Card, CardVariant};
use crate::frontend::design_system::theme::{Size, Intent};

#[component]
pub fn TabNav(
    active_tab: ReadSignal<String>,
    set_active_tab: WriteSignal<String>,
    tabs: Vec<(&'static str, &'static str)>, // (id, label)
) -> impl IntoView {
    view! {
        <div class="mb-8">
            <Card variant=CardVariant::Default padding=Size::Xs>
                <nav class="flex space-x-1">
                    {tabs.into_iter().map(|(id, label)| {
                        let tab_id = id.to_string();
                        let is_active = Signal::derive({
                            let tab_id = tab_id.clone();
                            move || active_tab.get() == tab_id
                        });
                        
                        view! {
                            <Button
                                size=Size::Sm
                                intent=if is_active.get() { Intent::Primary } else { Intent::Secondary }
                                on_click=Callback::new({
                                    let tab_id = tab_id.clone();
                                    move |_| set_active_tab.set(tab_id.clone())
                                })
                            >
                                {label}
                            </Button>
                        }
                    }).collect_view()}
                </nav>
            </Card>
        </div>
    }
}