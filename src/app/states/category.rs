use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Category;
use crate::backend::category::get_categories;




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

        // The derived signal approach means no Effects needed here!
        // The WebSocket state handles message parsing with derived signals
        // The category state can simply expose a derived signal that combines
        // the base categories with real-time updates

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