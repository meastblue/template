use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use super::{config::Config, routes::config_routes};

pub struct Server {
    db_pool: PgPool,
}

impl Server {
    pub async fn new() -> Self {
        let config = Config::load();
        let db_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(10))
            .connect(&config.db_url())
            .await
            .expect("Failed to connect to database");

        Self { db_pool }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = config_routes(self.db_pool.clone());
        let config = Config::load();
        let addr: SocketAddr = format!("{}:{}", config.srv_host, config.srv_port)
            .parse()
            .expect("Invalid address format");
        let listener = TcpListener::bind(addr).await?;

        println!("Server running at http://{}", addr);
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}
