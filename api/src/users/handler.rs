use axum::{Json, extract::State};
use sqlx::PgPool;

use super::entity::{NewUser, User};

pub struct UserHandler;

impl UserHandler {
    pub async fn create(
        State(pool): State<PgPool>,
        Json(new_user): Json<NewUser>,
    ) -> Result<Json<User>, String> {
        let user = NewUser::create(&pool, new_user)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Json(user))
    }
}
