use leptos::prelude::*;

use crate::backend::category::CreateCategory;

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
            <div>
                <h3 class="mt-6 text-center text-2xl font-bold text-text">
                    "Create a Category"
                </h3>
            </div>
            
            <ActionForm 
                action=create_action
                attr:class="mt-8 space-y-6"
            >
                <div class="space-y-4">
                    <div>
                        <label for="name" class="sr-only">"Category Name"</label>
                        <input
                            id="name"
                            name="req[name]"
                            type="text"
                            required
                            class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                            placeholder="Category name"
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

                <Show when=move || create_action.value().get().as_ref().map(|result| result.is_ok()).unwrap_or(false)>
                    <div class="bg-green-50 border border-green-200 rounded-md p-4">
                        <div class="flex">
                            <div class="ml-3">
                                <h3 class="text-sm font-medium text-green-800">
                                    "Category created successfully!"
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
                                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                "Creating category..."
                            </span>
                        </Show>
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}