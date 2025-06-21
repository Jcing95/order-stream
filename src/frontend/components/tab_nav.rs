use leptos::prelude::*;

#[component]
pub fn TabNav(
    active_tab: ReadSignal<String>,
    set_active_tab: WriteSignal<String>,
    tabs: Vec<(&'static str, &'static str)>, // (id, label)
) -> impl IntoView {
    view! {
        <div class="mb-8">
            <div class="bg-gray-100 dark:bg-gray-800 p-1 rounded-xl shadow-inner">
                <nav class="flex space-x-1">
                    {tabs.into_iter().map(|(id, label)| {
                        let tab_id = id.to_string();
                        view! {
                            <button
                                class={
                                    let tab_id = tab_id.clone();
                                    move || {
                                        if active_tab.get() == tab_id {
                                            format!("bg-white dark:bg-gray-700 text-blue-600 dark:text-blue-400 shadow-lg whitespace-nowrap px-6 py-3 rounded-lg font-semibold text-sm transition-all duration-200 transform scale-[1.02]")
                                        } else {
                                            format!("text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-white/50 dark:hover:bg-gray-700/50 whitespace-nowrap px-6 py-3 rounded-lg font-medium text-sm transition-all duration-200 hover:scale-[1.01]")
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