use axum::Router;
use sqlx::PgPool;

use crate::tasks::route::TaskRoute;

use super::middlewares::cors_middleware;

pub fn config_routes(pool: PgPool) -> Router {
    let routes = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, world!" }))
        .nest("/tasks", TaskRoute::init(pool.clone()));

    Router::new().layer(cors_middleware()).nest("/api", routes)
}
