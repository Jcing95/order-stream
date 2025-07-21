use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Category;
use crate::backend::category::get_categories;

#[cfg(feature = "hydrate")]
use crate::app::states::websocket::{self, Message};



#[derive(Debug, Clone)]
pub struct CategoryState {
    categories: ReadSignal<Vec<Category>>,
    set_categories: WriteSignal<Vec<Category>>,
}

impl CategoryState {
    pub fn new() -> Self {
        let (categories, set_categories) = signal(Vec::new());
        
        // Load categories once on initialization using Effect
        Effect::new({
            let set_categories = set_categories;
            move |_| {
                spawn_local(async move {
                    match get_categories().await {
                        Ok(cats) => set_categories.set(cats),
                        Err(_) => {}, // Keep empty vec on error
                    }
                });
            }
        });

        // Subscribe to WebSocket updates (client-side only)
        #[cfg(feature = "hydrate")]
        {
            let set_categories_ws = set_categories;
            Effect::new(move |_| {
                let ws_state = websocket::get();
                let categories_signal = ws_state.categories();
                
                // Handle incoming WebSocket messages for categories
                Effect::new(move |_| {
                    if let Some(msg) = categories_signal.get() {
                        match msg {
                            Message::Add(category) => {
                                set_categories_ws.update(|cats| cats.push(category));
                            },
                            Message::Update(updated_category) => {
                                set_categories_ws.update(|cats| {
                                    if let Some(cat) = cats.iter_mut().find(|c| c.id == updated_category.id) {
                                        *cat = updated_category;
                                    }
                                });
                            },
                            Message::Delete(category_id) => {
                                set_categories_ws.update(|cats| cats.retain(|c| c.id != category_id));
                            }
                        }
                    }
                });
            });
        }

        Self {
            categories,
            set_categories,
        }
    }

    /// Get the categories signal for reactive UI - this is your simple interface
    pub fn get_categories(&self) -> ReadSignal<Vec<Category>> {
        self.categories
    }

    /// Add a single category (for WebSocket updates when someone creates a category)
    pub fn add_category(&self, category: Category) {
        self.set_categories.update(|cats| cats.push(category));
    }
    
    /// Update a single category (for WebSocket updates when someone modifies a category)
    pub fn update_category(&self, updated: Category) {
        self.set_categories.update(|cats| {
            if let Some(cat) = cats.iter_mut().find(|c| c.id == updated.id) {
                *cat = updated;
            }
        });
    }
    
    /// Remove a category (for WebSocket updates when someone deletes a category)
    pub fn remove_category(&self, id: &str) {
        self.set_categories.update(|cats| cats.retain(|c| c.id != id));
    }
    
    /// Replace all categories (for full refresh if needed)
    pub fn set_categories(&self, categories: Vec<Category>) {
        self.set_categories.set(categories);
    }
}

pub fn provide() -> CategoryState {
    let category_state = CategoryState::new();
    provide_context(category_state.clone());
    category_state
}

pub fn get() -> CategoryState {
    expect_context::<CategoryState>()
}