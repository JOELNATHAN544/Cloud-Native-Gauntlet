use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use tokio::sync::RwLock;

mod auth;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS configuration
    let cors = CorsLayer::permissive();

    // Shared in-memory state for tasks
    let state = Arc::new(routes::tasks::AppState {
        tasks: Arc::new(RwLock::new(vec![
            models::Task {
                id: uuid::Uuid::new_v4(),
                title: "Complete Cloud-Native Gauntlet".to_string(),
                description: Some("Finish the 12-day challenge".to_string()),
                completed: false,
                user_id: uuid::Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            models::Task {
                id: uuid::Uuid::new_v4(),
                title: "Deploy to K3s".to_string(),
                description: Some("Get the cluster running".to_string()),
                completed: true,
                user_id: uuid::Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ])),
    });

    // Build our application with routes and state
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/tasks", get(routes::tasks::list_tasks))
        .route("/api/tasks", post(routes::tasks::create_task))
        .route("/api/auth/login", post(routes::auth::login))
        .with_state(state)
        .layer(cors);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
