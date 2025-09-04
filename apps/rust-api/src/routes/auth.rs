use axum::{http::StatusCode, Json};
use uuid::Uuid;

use crate::models::{LoginRequest, LoginResponse, User};


pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    // Mock authentication - will be replaced with database lookup
    if payload.username == "admin" && payload.password == "password" {
        let user = User {
            id: Uuid::new_v4(),
            username: payload.username,
            email: "admin@example.com".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        Ok(Json(LoginResponse { token: "not-implemented".to_string(), user }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
