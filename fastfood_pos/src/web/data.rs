use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Represents a single product that can be sold.
#[derive(Serialize, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

/// Represents a single item within an order.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderItem {
    pub product_id: u32,
    pub quantity: u32,
}

/// Represents a customer's order.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Order {
    pub items: Vec<OrderItem>,
    pub total: f64,
}

/// Holds the application's shared state.
pub struct AppState {
    pub products: Mutex<Vec<Product>>,
    pub orders: Mutex<Vec<Order>>,
}