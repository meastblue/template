use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Task {
    pub async fn list(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, completed, created_at, updated_at, deleted_at
            FROM tasks
            WHERE deleted_at IS NULL
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, completed, created_at, updated_at, deleted_at
            FROM tasks
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET deleted_at = now()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, title, description, completed, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn destroy(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(())
    }

    pub async fn restore(pool: &PgPool, id: Uuid) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET deleted_at = NULL
            WHERE id = $1 AND deleted_at IS NOT NULL
            RETURNING id, title, description, completed, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

impl NewTask {
    pub async fn create(pool: &PgPool, data: NewTask) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (id, title, description, completed, created_at, updated_at)
            VALUES ($1, $2, $3, false, now(), now())
            RETURNING id, title, description, completed, created_at, updated_at, deleted_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(data.title)
        .bind(data.description)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl UpdateTask {
    pub async fn update(pool: &PgPool, id: Uuid, data: UpdateTask) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                completed = COALESCE($4, completed),
                updated_at = now()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, title, description, completed, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .bind(data.title)
        .bind(data.description)
        .bind(data.completed)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }
}
