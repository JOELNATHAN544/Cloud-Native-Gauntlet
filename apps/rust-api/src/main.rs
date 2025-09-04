use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::FromRef;

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

    // Auth state (Keycloak OIDC)
    let issuer = "http://10.72.220.223:8080/realms/gauntlet".to_string();
    let jwks_url = format!("{}/protocol/openid-connect/certs", issuer);
    let auth_state = Arc::new(auth::AuthState::new(auth::OidcConfig {
        issuer: issuer.clone(),
        client_id: "rust-api".to_string(),
        jwks_url,
    }));

    #[derive(Clone)]
    struct AppCtx {
        tasks: Arc<routes::tasks::AppState>,
        auth: Arc<auth::AuthState>,
    }

    impl FromRef<AppCtx> for Arc<routes::tasks::AppState> {
        fn from_ref(input: &AppCtx) -> Self {
            input.tasks.clone()
        }
    }

    impl FromRef<AppCtx> for Arc<auth::AuthState> {
        fn from_ref(input: &AppCtx) -> Self {
            input.auth.clone()
        }
    }

    let ctx = AppCtx { tasks: state.clone(), auth: auth_state.clone() };

    // Build our application with routes and state
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/tasks", get(routes::tasks::list_tasks))
        .route("/api/tasks", post(routes::tasks::create_task))
        .route("/api/auth/login", post(routes::auth::login))
        .with_state(auth_state)
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
