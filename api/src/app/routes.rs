use axum::Router;
use sqlx::PgPool;

use crate::users::route::UserRoute;

use super::middlewares::cors_middleware;

pub fn config_routes(pool: PgPool) -> Router {
    let routes = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, world!" }))
        .nest("/users", UserRoute::init(pool));

    Router::new().layer(cors_middleware()).nest("/api", routes)
}
