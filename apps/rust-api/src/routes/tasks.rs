use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{CreateTaskRequest, Task};
use crate::auth::AuthUser;

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<RwLock<Vec<Task>>>,
}

pub async fn list_tasks(
    AuthUser(_user): AuthUser,
) -> Json<Vec<Task>> {
    // For now, return empty tasks since we don't have access to state
    // This will be fixed when we integrate with the main state
    Json(vec![])
}

pub async fn create_task(
    AuthUser(_user): AuthUser,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<Task>, StatusCode> {
    let task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        completed: false,
        user_id: Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    Ok(Json(task))
}
