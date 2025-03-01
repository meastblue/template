use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use sqlx::PgPool;

use super::handler::TaskHandler;

pub struct TaskRoute;

impl TaskRoute {
    pub fn init(pool: PgPool) -> Router {
        Router::new()
            .route("/", get(TaskHandler::list).post(TaskHandler::create))
            .route(
                "/{id}",
                get(TaskHandler::get)
                    .put(TaskHandler::update)
                    .patch(TaskHandler::update)
                    .delete(TaskHandler::delete),
            )
            .route("/{id}/restore", patch(TaskHandler::restore))
            .route("/{id}/destroy", delete(TaskHandler::destroy))
            .with_state(pool)
    }
}
