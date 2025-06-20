use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::{Category, Item, Order, CreateCategoryRequest, CreateItemRequest};
use crate::backend::services::{
    categories::{get_categories, create_category, delete_category},
    items::{get_items, create_item},
    orders::{get_orders, create_order, delete_order},
};

#[derive(Clone, Copy)]
pub struct AdminState {
    pub categories: RwSignal<Vec<Category>>,
    pub items: RwSignal<Vec<Item>>,
    pub orders: RwSignal<Vec<Order>>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl AdminState {
    pub fn new() -> Self {
        Self {
            categories: RwSignal::new(Vec::new()),
            items: RwSignal::new(Vec::new()),
            orders: RwSignal::new(Vec::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
        }
    }

    pub async fn load_all(&self) {
        self.loading.set(true);
        self.error.set(None);
        
        // Load categories first (needed for items)
        match get_categories().await {
            Ok(fetched_categories) => {
                self.categories.set(fetched_categories);
            }
            Err(err) => {
                self.error.set(Some(format!("Failed to load categories: {}", err)));
                self.loading.set(false);
                return;
            }
        }
        
        // Load items
        match get_items().await {
            Ok(fetched_items) => {
                self.items.set(fetched_items);
            }
            Err(err) => {
                self.error.set(Some(format!("Failed to load items: {}", err)));
            }
        }
        
        // Load orders
        match get_orders().await {
            Ok(fetched_orders) => {
                self.orders.set(fetched_orders);
            }
            Err(err) => {
                self.error.set(Some(format!("Failed to load orders: {}", err)));
            }
        }
        
        self.loading.set(false);
    }

    // Category operations with backend calls
    pub fn create_category(&self, request: CreateCategoryRequest) {
        let state = self.clone();
        spawn_local(async move {
            state.loading.set(true);
            state.error.set(None);
            
            match create_category(request).await {
                Ok(new_category) => {
                    state.categories.update(|categories| categories.push(new_category));
                }
                Err(err) => {
                    state.error.set(Some(format!("Failed to create category: {}", err)));
                }
            }
            
            state.loading.set(false);
        });
    }

    pub fn delete_category(&self, category_id: String) {
        let state = self.clone();
        spawn_local(async move {
            state.loading.set(true);
            state.error.set(None);
            
            match delete_category(category_id.clone()).await {
                Ok(_) => {
                    state.categories.update(|categories| {
                        categories.retain(|c| c.id != category_id);
                    });
                }
                Err(err) => {
                    state.error.set(Some(format!("Failed to delete category: {}", err)));
                }
            }
            
            state.loading.set(false);
        });
    }

    // Item operations with backend calls
    pub fn create_item(&self, request: CreateItemRequest) {
        let state = self.clone();
        spawn_local(async move {
            state.loading.set(true);
            state.error.set(None);
            
            match create_item(request).await {
                Ok(new_item) => {
                    state.items.update(|items| items.push(new_item));
                }
                Err(err) => {
                    state.error.set(Some(format!("Failed to create item: {}", err)));
                }
            }
            
            state.loading.set(false);
        });
    }

    // Order operations with backend calls
    pub fn create_order(&self) {
        let state = self.clone();
        spawn_local(async move {
            state.loading.set(true);
            state.error.set(None);
            
            match create_order().await {
                Ok(new_order) => {
                    state.orders.update(|orders| orders.push(new_order));
                }
                Err(err) => {
                    state.error.set(Some(format!("Failed to create order: {}", err)));
                }
            }
            
            state.loading.set(false);
        });
    }

    pub fn delete_order(&self, order_id: String) {
        let state = self.clone();
        spawn_local(async move {
            state.loading.set(true);
            state.error.set(None);
            
            match delete_order(order_id.clone()).await {
                Ok(_) => {
                    state.orders.update(|orders| {
                        orders.retain(|o| o.id != order_id);
                    });
                }
                Err(err) => {
                    state.error.set(Some(format!("Failed to delete order: {}", err)));
                }
            }
            
            state.loading.set(false);
        });
    }
}