use leptos::prelude::*;

use crate::{
    backend::product::CreateProduct,
    app::{
        states::category,
        components::atoms::icons,
    },
};

#[component]
pub fn CreateProduct() -> impl IntoView {
    let create_action = ServerAction::<CreateProduct>::new();
    let category_state = category::get();
    let categories = category_state.get_categories();

    // Handle successful product creation
    Effect::new(move |_| {
        if let Some(Ok(_product)) = create_action.value().get() {
            // Product created successfully - could add success notification here
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
                        <label for="name" class="block text-sm font-medium text-text mb-2">"Product Name"</label>
                        <input
                            id="name"
                            name="req[name]"
                            type="text"
                            required
                            class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                            placeholder="Enter product name"
                        />
                    </div>
                    
                    <div>
                        <label for="category_id" class="block text-sm font-medium text-text mb-2">"Category"</label>
                        <select
                            id="category_id"
                            name="req[category_id]"
                            required
                            class="relative block w-full px-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                        >
                            <option value="">"Select a category"</option>
                            <For
                                each=move || categories.get()
                                key=|category| category.id.clone()
                                children=move |category| {
                                    view! {
                                        <option value={category.id.clone()}>{category.name}</option>
                                    }
                                }
                            />
                        </select>
                    </div>
                    
                    <div>
                        <label for="price" class="block text-sm font-medium text-text mb-2">"Price (€)"</label>
                        <div class="relative">
                            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <span class="text-text-muted sm:text-sm">"€"</span>
                            </div>
                            <input
                                id="price"
                                name="req[price]"
                                type="number"
                                step="0.01"
                                min="0"
                                required
                                class="relative block w-full pl-8 pr-3 py-2 border border-border bg-surface placeholder-text-muted text-text rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                                placeholder="9.99"
                            />
                        </div>
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
                            fallback=|| view! { "Create Product" }
                        >
                            <span class="flex items-center">
                                <icons::Spinner attr:class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"/>
                                "Creating product..."
                            </span>
                        </Show>
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}