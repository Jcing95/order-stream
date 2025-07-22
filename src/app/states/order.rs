use leptos::prelude::*;
use crate::common::types::Product;

#[derive(Debug, Clone)]
pub struct OrderItem {
    pub product_id: String,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

impl OrderItem {
    pub fn total(&self) -> f64 {
        self.price * self.quantity as f64
    }
    
    pub fn from_product(product: Product) -> Self {
        Self {
            product_id: product.id,
            name: product.name,
            price: product.price,
            quantity: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderState {
    items: ReadSignal<Vec<OrderItem>>,
    set_items: WriteSignal<Vec<OrderItem>>,
}

impl OrderState {
    pub fn new() -> Self {
        let (items, set_items) = signal(Vec::new());
        
        Self {
            items,
            set_items,
        }
    }
    
    /// Get the order items signal for reactive UI
    pub fn get_items(&self) -> ReadSignal<Vec<OrderItem>> {
        self.items
    }
    
    /// Add a product to the order (or increase quantity if already present)
    pub fn add_product(&self, product: Product) {
        if !product.active {
            return; // Don't add inactive products
        }
        
        self.set_items.update(|items| {
            if let Some(existing_item) = items.iter_mut().find(|item| item.product_id == product.id) {
                existing_item.quantity += 1;
            } else {
                items.push(OrderItem::from_product(product));
            }
        });
    }
    
    /// Increase quantity of an item
    pub fn increase_quantity(&self, product_id: &str) {
        self.set_items.update(|items| {
            if let Some(item) = items.iter_mut().find(|item| item.product_id == product_id) {
                item.quantity += 1;
            }
        });
    }
    
    /// Decrease quantity of an item (removes if quantity becomes 0)
    pub fn decrease_quantity(&self, product_id: &str) {
        self.set_items.update(|items| {
            if let Some(item) = items.iter_mut().find(|item| item.product_id == product_id) {
                if item.quantity > 1 {
                    item.quantity -= 1;
                } else {
                    items.retain(|i| i.product_id != product_id);
                }
            }
        });
    }
    
    /// Remove an item completely from the order
    pub fn remove_item(&self, product_id: &str) {
        self.set_items.update(|items| {
            items.retain(|item| item.product_id != product_id);
        });
    }
    
    /// Clear all items from the order
    pub fn clear(&self) {
        self.set_items.set(Vec::new());
    }
    
    /// Get the total price of all items
    pub fn total_price(&self) -> f64 {
        self.items.get_untracked().iter().map(|item| item.total()).sum()
    }
    
    /// Get the total number of items
    pub fn total_items(&self) -> u32 {
        self.items.get_untracked().iter().map(|item| item.quantity).sum()
    }
}

pub fn provide() -> OrderState {
    let order_state = OrderState::new();
    provide_context(order_state.clone());
    order_state
}

pub fn get() -> OrderState {
    expect_context::<OrderState>()
}