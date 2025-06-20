use leptos::prelude::*;

#[component]
pub fn TabNav(
    active_tab: ReadSignal<String>,
    set_active_tab: WriteSignal<String>,
    tabs: Vec<(&'static str, &'static str)>, // (id, label)
) -> impl IntoView {
    view! {
        <div class="mb-6">
            <div class="border-b border-gray-200">
                <nav class="-mb-px flex space-x-8">
                    {tabs.into_iter().map(|(id, label)| {
                        let tab_id = id.to_string();
                        view! {
                            <button
                                class={
                                    let tab_id = tab_id.clone();
                                    move || {
                                        if active_tab.get() == tab_id {
                                            "border-indigo-500 text-indigo-600 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm"
                                        } else {
                                            "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm"
                                        }
                                    }
                                }
                                on:click={
                                    let tab_id = tab_id.clone();
                                    move |_| set_active_tab.set(tab_id.clone())
                                }
                            >
                                {label}
                            </button>
                        }
                    }).collect_view()}
                </nav>
            </div>
        </div>
    }
}