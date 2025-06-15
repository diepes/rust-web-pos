use std::sync::{Arc, Mutex};
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::web::data::{AppState, Product, Order};

pub mod web;

/// Handler to get the list of available products.
pub async fn get_products(State(state): State<Arc<AppState>>) -> Json<Vec<Product>> {
    let products = state.products.lock().unwrap().clone();
    Json(products)
}

/// Handler to create a new order.
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(order): Json<Order>,
) -> (StatusCode, Json<Order>) {
    println!("Received new order: {:?}", order);
    let mut orders = state.orders.lock().unwrap();
    orders.push(order);
    let created_order = orders.last().unwrap().clone();
    (StatusCode::CREATED, Json(created_order))
}

/// Helper function to populate our initial list of products.
pub fn get_initial_products() -> Vec<Product> {
    vec![
        Product { id: 1, name: "Classic Burger".to_string(), price: 15.50 },
        Product { id: 2, name: "Cheese Burger".to_string(), price: 17.50 },
        Product { id: 3, name: "Fries".to_string(), price: 6.00 },
        Product { id: 4, name: "Soda".to_string(), price: 4.50 },
    ]
}