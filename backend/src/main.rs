use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod storage;
use crate::{error::ApiError, storage::Storage};

struct AppState {
    storage: Storage,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let host = std::env::var("BACKEND_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("BACKEND_PORT").unwrap_or("30303".to_string());
    let db_conn_str = std::env::var("DATABASE_URL").unwrap();
    let storage = Storage::new(&db_conn_str).await.unwrap();

    let state = AppState { storage };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/tasks", get(get_tasks))
        .route("/task", post(add_task))
        .route(
            "/task/{id}",
            get(get_task).put(update_task).delete(delete_task),
        )
        .fallback(not_found)
        .with_state(shared_state);

    let address = format!("{host}:{port}");
    let listener = TcpListener::bind(&address).await.unwrap();
    info!("Starting server on http://{host}:{port}");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    #[serde(skip_deserializing)]
    id: i32,
    title: String,
    description: Option<String>,
}

#[axum::debug_handler]
async fn get_tasks(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Task>>, ApiError> {
    debug!("Get tasks called");
    let tasks = state.storage.query_tasks().await?;
    Ok(Json(tasks))
}

#[axum::debug_handler]
async fn add_task(
    State(state): State<Arc<AppState>>,
    Json(task): Json<Task>,
) -> Result<Json<serde_json::Value>, ApiError> {
    debug!("Add task called: {task:?}");
    let id = state.storage.insert_task(&task).await?;
    Ok(Json(serde_json::json!({"id": id})))
}

async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Task>, ApiError> {
    debug!("Get task called: {id}");
    let task = state.storage.query_task(id).await?;
    if let Some(task) = task {
        Ok(Json(task))
    } else {
        Err(ApiError::new_not_found("task not found".to_string()))
    }
}

async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(task): Json<Task>,
) -> Result<Json<serde_json::Value>, ApiError> {
    debug!("Update task called: {id} {task:?}");
    if let Some(id) = state.storage.update_task(id, &task).await? {
        Ok(Json(serde_json::json!({"id": id})))
    } else {
        Err(ApiError::new_not_found("task not found".to_string()))
    }
}

async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Task>, ApiError> {
    debug!("Delete task called: {id}");
    if let Some(task) = state.storage.delete_task(id).await? {
        Ok(Json(task))
    } else {
        Err(ApiError::new_not_found("task not found".to_string()))
    }
}

async fn not_found() -> impl IntoResponse {
    ApiError::new_not_found("unexpected route".to_string())
}
