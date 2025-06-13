use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

// The main entry point for our application
#[tokio::main]
async fn main() {
    // --- 1. Set up application state ---
    // This is where we'll store our data, like the list of products and orders.
    // Arc and Mutex are used to safely share this data across different threads.
    let app_state = Arc::new(AppState {
        products: Mutex::new(get_initial_products()),
        orders: Mutex::new(Vec::new()),
    });

    // --- 2. Define CORS policy ---
    // This allows our frontend (running on a different origin) to talk to our backend.
    let cors = CorsLayer::new()
        .allow_origin(Any) // In production, you'd restrict this to your frontend's domain
        .allow_methods(Any)
        .allow_headers(Any);

    // --- 3. Define our application's routes ---
    // These are the URLs our API will respond to.
    let app = Router::new()
        // A GET request to /api/products will call the `get_products` function.
        .route("/api/products", get(get_products))
        // A POST request to /api/orders will call the `create_order` function.
        .route("/api/orders", post(create_order))
        // Serve static files from the `public` directory.
        .nest_service("/", tower_http::services::ServeDir::new("public"))
        // Add the application state and CORS policy to our router.
        .with_state(app_state)
        .layer(cors);

    // --- 4. Start the server ---
    // We bind our server to port 3000 on all available network interfaces.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// --- Data Structures ---
// These `structs` define the shape of our data.

/// Represents a single product that can be sold.
#[derive(Serialize, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

/// Represents a single item within an order.
#[derive(Deserialize, Serialize, Debug, Clone)]
struct OrderItem {
    product_id: u32,
    quantity: u32,
}

/// Represents a customer's order.
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Order {
    items: Vec<OrderItem>,
    total: f64,
}

/// Holds the application's shared state.
struct AppState {
    products: Mutex<Vec<Product>>,
    orders: Mutex<Vec<Order>>,
}

// --- API Handler Functions ---
// These functions are called when a request hits one of our routes.

/// Handler to get the list of available products.
async fn get_products(State(state): State<Arc<AppState>>) -> Json<Vec<Product>> {
    // We lock the mutex to safely access the products vector and clone it.
    let products = state.products.lock().unwrap().clone();
    // We return the products as JSON.
    Json(products)
}

/// Handler to create a new order.
async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(order): Json<Order>,
) -> (StatusCode, Json<Order>) {
    println!("Received new order: {:?}", order);
    // We lock the orders mutex and push the new order into our list.
    let mut orders = state.orders.lock().unwrap();
    orders.push(order); // The `order` is moved here
    // Retrieve the just-added order to return it.
    let created_order = orders.last().unwrap().clone();

    // We return a 201 Created status code and the created order as JSON.
    (StatusCode::CREATED, Json(created_order))
}

/// Helper function to populate our initial list of products.
fn get_initial_products() -> Vec<Product> {
    vec![
        Product { id: 1, name: "Classic Burger".to_string(), price: 15.50 },
        Product { id: 2, name: "Cheese Burger".to_string(), price: 17.50 },
        Product { id: 3, name: "Fries".to_string(), price: 6.00 },
        Product { id: 4, name: "Soda".to_string(), price: 4.50 },
    ]
}

// In the `create_order` function, the `order` variable is of type `Order`.
// When we do `orders.push(order)`, the ownership of the value is moved into the `orders` vector.
// This means we can't use the `order` variable again after that line.
// To solve this, we can clone the order *before* moving it, or we can retrieve it from the vector after pushing.
// The updated code now retrieves the last added order to return it.
// Additionally, for robust error handling, `unwrap()` should be replaced with proper error handling in a real application.
// For this simple example, it's fine.
// The `OrderItem` and `Order` structs now also derive `Serialize` so they can be returned as JSON.
