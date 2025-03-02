use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

use super::entity::{NewTask, Task, UpdateTask};

pub struct TaskHandler {
    pub pool: PgPool,
}

impl TaskHandler {
    pub async fn list(State(pool): State<PgPool>) -> Result<Json<Vec<Task>>, String> {
        let tasks = Task::list(&pool).await.map_err(|e| e.to_string())?;

        Ok(Json(tasks))
    }

    pub async fn get(
        State(pool): State<PgPool>,
        Path(id): Path<String>,
    ) -> Result<Json<Task>, String> {
        let id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
        let task = Task::get(&pool, id).await.map_err(|e| e.to_string())?;

        Ok(Json(task))
    }

    pub async fn create(
        State(pool): State<PgPool>,
        Json(new_task): Json<NewTask>,
    ) -> Result<Json<Task>, String> {
        let task = NewTask::create(&pool, new_task)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Json(task))
    }

    pub async fn update(
        State(pool): State<PgPool>,
        Path(id): Path<String>,
        Json(update_task): Json<UpdateTask>,
    ) -> Result<Json<Task>, String> {
        let id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
        let task = UpdateTask::update(&pool, id, update_task)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Json(task))
    }

    pub async fn delete(
        State(pool): State<PgPool>,
        Path(id): Path<String>,
    ) -> Result<Json<Task>, String> {
        let id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
        let task = Task::delete(&pool, id).await.map_err(|e| e.to_string())?;

        Ok(Json(task))
    }

    pub async fn restore(
        State(pool): State<PgPool>,
        Path(id): Path<String>,
    ) -> Result<Json<Task>, String> {
        let id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
        let task = Task::restore(&pool, id).await.map_err(|e| e.to_string())?;

        Ok(Json(task))
    }

    pub async fn destroy(
        State(pool): State<PgPool>,
        Path(id): Path<String>,
    ) -> Result<Json<()>, String> {
        let id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
        Task::destroy(&pool, id).await.map_err(|e| e.to_string())?;

        Ok(Json(()))
    }
}
