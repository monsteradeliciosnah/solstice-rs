use axum::{extract::{Path, State}, Json, response::IntoResponse};
use serde_json::json;
use utoipa::{path, ToSchema};
use uuid::Uuid;
use crate::{store::Store, models::{Task, NewTask, TaskPatch, ApiError}};

#[utoipa::path(
    get,
    path = "/api/v1/tasks",
    responses(
        (status = 200, body = [Task])
    )
)]
pub async fn list_tasks(State(store): State<Store>) -> impl IntoResponse {
    match store.list().await {
        Ok(items) => Json(items).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message":e.to_string()}))).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/tasks",
    request_body = NewTask,
    responses(
        (status = 201, body = Task)
    )
)]
pub async fn create_task(State(store): State<Store>, Json(new): Json<NewTask>) -> impl IntoResponse {
    match store.create(new).await {
        Ok(t) => (axum::http::StatusCode::CREATED, Json(t)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message":e.to_string()}))).into_response()
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/tasks/{id}",
    params(("id" = String, Path, description = "Task ID")),
    responses(
        (status = 200, body = Task),
        (status = 404, body = ApiError)
    )
)]
pub async fn get_task(State(store): State<Store>, Path(id): Path<String>) -> impl IntoResponse {
    let id = Uuid::parse_str(&id).unwrap_or(Uuid::nil());
    match store.get(id).await {
        Ok(t) => Json(t).into_response(),
        Err(_) => (axum::http::StatusCode::NOT_FOUND, Json(json!({"message":"not found"}))).into_response()
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/tasks/{id}",
    request_body = TaskPatch,
    responses((status = 200, body = Task), (status=404, body=ApiError))
)]
pub async fn patch_task(State(store): State<Store>, Path(id): Path<String>, Json(p): Json<TaskPatch>) -> impl IntoResponse {
    let id = Uuid::parse_str(&id).unwrap_or(Uuid::nil());
    match store.patch(id, p).await {
        Ok(t) => Json(t).into_response(),
        Err(_) => (axum::http::StatusCode::NOT_FOUND, Json(json!({"message":"not found"}))).into_response()
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/tasks/{id}",
    responses((status = 204))
)]
pub async fn delete_task(State(store): State<Store>, Path(id): Path<String>) -> impl IntoResponse {
    let id = Uuid::parse_str(&id).unwrap_or(Uuid::nil());
    match store.delete(id).await {
        Ok(()) => axum::http::StatusCode::NO_CONTENT,
        Err(_) => axum::http::StatusCode::NOT_FOUND
    }
}
