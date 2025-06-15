use axum::{
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::{get_products, create_order, get_initial_products};
// Import AppState from the web::data module
use crate::web::data::AppState;

pub fn setup_web_app() -> Router {
    // --- 1. Set up application state ---
    let app_state = Arc::new(AppState {
        products: Mutex::new(get_initial_products()),
        orders: Mutex::new(Vec::new()),
    });

    // --- 2. Define CORS policy ---
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // --- 3. Define our application's routes ---
    let app = Router::new()
        .route("/api/products", get(get_products))
        .route("/api/orders", post(create_order))
        .nest_service("/", tower_http::services::ServeDir::new("public"))
        .with_state(app_state)
        .layer(cors);

    app
}
