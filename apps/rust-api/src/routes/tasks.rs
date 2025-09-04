use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{CreateTaskRequest, Task};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<RwLock<Vec<Task>>>,
}

pub async fn list_tasks(State(state): State<Arc<AppState>>) -> Json<Vec<Task>> {
    let tasks = state.tasks.read().await;
    Json(tasks.clone())
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
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

    {
        let mut tasks = state.tasks.write().await;
        tasks.push(task.clone());
    }

    Ok(Json(task))
}
