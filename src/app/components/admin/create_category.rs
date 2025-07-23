use leptos::prelude::*;

use crate::{
    backend::category::CreateCategory,
    app::components::atoms::icons,
};

#[component]
pub fn CreateCategory() -> impl IntoView {
    let create_action = ServerAction::<CreateCategory>::new();

    // Handle successful category creation
    Effect::new(move |_| {
        if let Some(Ok(_category)) = create_action.value().get() {
            // Category created successfully - could add success notification here
        }
    });

    view! {
        <div class="max-w-md w-full space-y-8">
            <ActionForm
                action=create_action
                attr:class="mt-8 space-y-6"
            >
                <div class="space-y-4">
                    <div>
                        <label for="name" class="block text-sm font-medium text-text mb-2">"Category Name"</label>
                        <input
                            id="name"
                            name="req[name]"
                            type="text"
                            required
                            class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                            placeholder="Enter category name"
                        />
                    </div>
                </div>

                <Show when=move || create_action.value().get().as_ref().map(|result| result.is_err()).unwrap_or(false)>
                    <div class="bg-red-50 border border-red-200 rounded-md p-4">
                        <div class="flex">
                            <div class="ml-3">
                                <h3 class="text-sm font-medium text-red-800">
                                    {move || {
                                        create_action.value().get()
                                            .and_then(|result| result.err())
                                            .map(|err| err.to_string())
                                            .unwrap_or_else(|| "An error occurred".to_string())
                                    }}
                                </h3>
                            </div>
                        </div>
                    </div>
                </Show>

                <div>
                    <button
                        type="submit"
                        disabled=move || create_action.pending().get()
                        class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <Show
                            when=move || create_action.pending().get()
                            fallback=|| view! { "Create Category" }
                        >
                            <span class="flex items-center">
                                <icons::Spinner attr:class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"/>
                                "Creating category..."
                            </span>
                        </Show>
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}
