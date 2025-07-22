use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Product;
use crate::backend::product::get_products;
use crate::common::resource_type::Message;
use crate::app::states::websocket;

#[derive(Debug, Clone)]
pub struct ProductState {
    products: ReadSignal<Vec<Product>>,
    set_products: WriteSignal<Vec<Product>>,
}

impl ProductState {
    pub fn new() -> Self {
        let (products, set_products) = signal(Vec::new());
        
        // Load products once on initialization using Effect
        Effect::new({
            let set_products = set_products;
            move |_| {
                spawn_local(async move {
                    match get_products().await {
                        Ok(prods) => set_products.set(prods),
                        Err(_) => {}, // Keep empty vec on error
                    }
                });
            }
        });

        let product_state = Self {
            products,
            set_products,
        };

        // Connect to websocket updates
        let websocket_state = websocket::get();
        Effect::new({
            let product_state = product_state.clone();
            let websocket_state = websocket_state.clone();
            move |_| {
                if let Some(message) = websocket_state.products.get() {
                    match message {
                        Message::Add(product) => {
                            product_state.add_product(product);
                        }
                        Message::Update(product) => {
                            product_state.update_product(product);
                        }
                        Message::Delete(id) => {
                            product_state.remove_product(&id);
                        }
                    }
                    // Clear the signal after processing to allow new messages to trigger
                    websocket_state.products.set(None);
                }
            }
        });


        product_state
    }

    /// Get the products signal for reactive UI - this is your simple interface
    pub fn get_products(&self) -> ReadSignal<Vec<Product>> {
        self.products
    }

    /// Add a single product (for WebSocket updates when someone creates a product)
    pub fn add_product(&self, product: Product) {
        self.set_products.update(|prods| prods.push(product));
    }
    
    /// Update a single product (for WebSocket updates when someone modifies a product)
    pub fn update_product(&self, updated: Product) {
        leptos::logging::log!("Updating product: {:?}", updated);
        let current_products = self.products.get_untracked();
        let new_products: Vec<Product> = current_products
            .iter()
            .map(|prod| {
                if prod.id == updated.id {
                    leptos::logging::log!("Found product to update: {} -> {:?}", prod.id, updated);
                    updated.clone()
                } else {
                    prod.clone()
                }
            })
            .collect();
        leptos::logging::log!("Setting new products vector with {} items", new_products.len());
        self.set_products.set(new_products);
    }
    
    /// Remove a product (for WebSocket updates when someone deletes a product)
    pub fn remove_product(&self, id: &str) {
        self.set_products.update(|prods| prods.retain(|p| p.id != id));
    }
    
    /// Replace all products (for full refresh if needed)
    pub fn set_products(&self, products: Vec<Product>) {
        self.set_products.set(products);
    }
}

pub fn provide() -> ProductState {
    let product_state = ProductState::new();
    provide_context(product_state.clone());
    product_state
}

pub fn get() -> ProductState {
    expect_context::<ProductState>()
}