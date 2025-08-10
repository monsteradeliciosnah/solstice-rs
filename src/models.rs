use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct NewTask {
    pub title: String
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TaskPatch {
    pub title: Option<String>,
    pub completed: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub message: String
}
