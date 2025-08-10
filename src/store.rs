use crate::models::{Task, NewTask, TaskPatch};
use serde_json::json;
use sqlx::{SqlitePool, Row};
use time::OffsetDateTime;
use uuid::Uuid;
use thiserror::Error;
use std::sync::Arc;
use axum::http::StatusCode;

#[derive(Clone)]
pub struct Store {
    pool: Arc<SqlitePool>
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("not found")]
    NotFound,
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

impl Store {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(url).await?;
        Ok(Self { pool: Arc::new(pool) })
    }

    pub async fn migrate(&self) -> Result<(), StoreError> {
        sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS tasks(
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"#).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Task>, StoreError> {
        let rows = sqlx::query("SELECT id, title, completed, created_at, updated_at FROM tasks ORDER BY created_at DESC")
            .fetch_all(&*self.pool).await?;
        let items = rows.into_iter().map(|r| Task {
            id: Uuid::parse_str(r.get::<String,_>("id").as_str()).unwrap(),
            title: r.get::<String,_>("title"),
            completed: r.get::<i64,_>("completed") != 0,
            created_at: OffsetDateTime::parse(&r.get::<String,_>("created_at"), &time::format_description::well_known::Rfc3339).unwrap(),
            updated_at: OffsetDateTime::parse(&r.get::<String,_>("updated_at"), &time::format_description::well_known::Rfc3339).unwrap(),
        }).collect();
        Ok(items)
    }

    pub async fn create(&self, new: NewTask) -> Result<Task, StoreError> {
        let now = OffsetDateTime::now_utc();
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO tasks(id, title, completed, created_at, updated_at) VALUES(?, ?, 0, ?, ?)")
            .bind(id.to_string())
            .bind(new.title.clone())
            .bind(now.format(&time::format_description::well_known::Rfc3339).unwrap())
            .bind(now.format(&time::format_description::well_known::Rfc3339).unwrap())
            .execute(&*self.pool).await?;
        Ok(Task { id, title: new.title, completed: false, created_at: now, updated_at: now })
    }

    pub async fn get(&self, id: Uuid) -> Result<Task, StoreError> {
        let row = sqlx::query("SELECT id, title, completed, created_at, updated_at FROM tasks WHERE id=?")
            .bind(id.to_string())
            .fetch_optional(&*self.pool).await?;
        match row {
            None => Err(StoreError::NotFound),
            Some(r) => Ok(Task{
                id,
                title: r.get::<String,_>("title"),
                completed: r.get::<i64,_>("completed") != 0,
                created_at: OffsetDateTime::parse(&r.get::<String,_>("created_at"), &time::format_description::well_known::Rfc3339).unwrap(),
                updated_at: OffsetDateTime::parse(&r.get::<String,_>("updated_at"), &time::format_description::well_known::Rfc3339).unwrap(),
            })
        }
    }

    pub async fn patch(&self, id: Uuid, patch: TaskPatch) -> Result<Task, StoreError> {
        let mut task = self.get(id).await?;
        if let Some(t) = patch.title { task.title = t; }
        if let Some(c) = patch.completed { task.completed = c; }
        task.updated_at = OffsetDateTime::now_utc();
        sqlx::query("UPDATE tasks SET title=?, completed=?, updated_at=? WHERE id=?")
            .bind(&task.title)
            .bind(if task.completed {1} else {0})
            .bind(task.updated_at.format(&time::format_description::well_known::Rfc3339).unwrap())
            .bind(id.to_string())
            .execute(&*self.pool).await?;
        Ok(task)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), StoreError> {
        let res = sqlx::query("DELETE FROM tasks WHERE id=?").bind(id.to_string()).execute(&*self.pool).await?;
        if res.rows_affected()==0 { return Err(StoreError::NotFound); }
        Ok(())
    }
}
