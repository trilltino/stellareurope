use axum::{
    routing::{get, post},
    Router,
};
use backend::{create_pool, handlers};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Create database pool
    let pool = create_pool().await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Build application with routes
    let app = Router::new()
        .route("/api/signup", post(handlers::signup))
        .route("/api/events", post(handlers::create_event))
        .route("/api/events", get(handlers::list_events))
        .route("/health", get(|| async { "OK" }))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await?;
    println!("🌟 ═══════════════════════════════════════════════════════════════");
    println!("🚀 STELLAR EUROPE BACKEND SERVER STARTED");
    println!("📡 Server running on http://127.0.0.1:8081");
    println!("🔗 API Endpoints:");
    println!("   • POST /api/signup - User registration");
    println!("   • POST /api/events - Create events with KPI planning");
    println!("   • GET  /api/events - List events");
    println!("   • GET  /health    - Health check");
    println!("🎯 Ready to track KPIs and manage Stellar community events!");
    println!("🌟 ═══════════════════════════════════════════════════════════════");

    axum::serve(listener, app).await?;

    Ok(())
}