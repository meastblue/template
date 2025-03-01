use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

use super::handler::UserHandler;

pub struct UserRoute;

impl UserRoute {
    pub fn init(pool: PgPool) -> Router {
        Router::new()
            .route("/", post(UserHandler::create))
            .route("/", get(|| async { "Hello, user!" }))
            .with_state(pool)
    }
}
