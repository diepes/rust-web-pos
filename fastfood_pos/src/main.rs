
use fastfood_pos::web::app;

#[tokio::main]
async fn main() {
    let app = app::setup_web_app();

    // --- 4. Start the server ---
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

