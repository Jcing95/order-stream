use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Order Stream"/>
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
                    </div>
                    
                    // Demo counter (keeping original functionality)
                    <div class="flex space-x-2 mt-8">
                        <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                            "+"
                        </button>
                        <button class="rounded px-3 py-2 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                            {value}
                        </button>
                        <button
                            on:click=move |_| set_value.update(|value| *value -= 1)
                            class="rounded px-3 py-2 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                            class:invisible=move || {value.get() < 1}
                        >
                            "-"
                        </button>
                    </div>
                </div>
            </div>
        </main>
    }
}