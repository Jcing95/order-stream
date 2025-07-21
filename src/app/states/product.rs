use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::common::types::Product;
// Assuming there will be product server functions
// use crate::backend::product::get_products;

#[cfg(feature = "hydrate")]
use crate::app::states::websocket::{self, Message};

#[derive(Debug, Clone)]
pub struct ProductState {
    products: ReadSignal<Vec<Product>>,
    set_products: WriteSignal<Vec<Product>>,
}

impl ProductState {
    pub fn new() -> Self {
        let (products, set_products) = signal(Vec::new());
        
        // Load products once on initialization
        Effect::new({
            let _set_products = set_products;
            move |_| {
                spawn_local(async move {
                    // TODO: Uncomment when get_products server function exists
                    // match get_products().await {
                    //     Ok(prods) => _set_products.set(prods),
                    //     Err(_) => {}, // Keep empty vec on error
                    // }
                });
            }
        });

        // Subscribe to WebSocket updates (client-side only)
        #[cfg(feature = "hydrate")]
        {
            let set_products_ws = set_products;
            Effect::new(move |_| {
                let ws_state = websocket::get();
                let products_signal = ws_state.products();
                
                // Handle incoming WebSocket messages for products
                Effect::new(move |_| {
                    if let Some(msg) = products_signal.get() {
                        match msg {
                            Message::Add(product) => {
                                set_products_ws.update(|prods| prods.push(product));
                            },
                            Message::Update(updated_product) => {
                                set_products_ws.update(|prods| {
                                    if let Some(prod) = prods.iter_mut().find(|p| p.id == updated_product.id) {
                                        *prod = updated_product;
                                    }
                                });
                            },
                            Message::Delete(product_id) => {
                                set_products_ws.update(|prods| prods.retain(|p| p.id != product_id));
                            }
                        }
                    }
                });
            });
        }

        Self {
            products,
            set_products,
        }
    }

    /// Get the products signal for reactive UI
    pub fn get_products(&self) -> ReadSignal<Vec<Product>> {
        self.products
    }

    /// Add a single product (for optimistic updates)
    pub fn add_product(&self, product: Product) {
        self.set_products.update(|prods| prods.push(product));
    }
    
    /// Update a single product (for optimistic updates)
    pub fn update_product(&self, updated: Product) {
        self.set_products.update(|prods| {
            if let Some(prod) = prods.iter_mut().find(|p| p.id == updated.id) {
                *prod = updated;
            }
        });
    }
    
    /// Remove a product (for optimistic updates)
    pub fn remove_product(&self, id: &str) {
        self.set_products.update(|prods| prods.retain(|p| p.id != id));
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