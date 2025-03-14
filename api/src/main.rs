use app::server::Server;

mod app;
mod tasks;

#[tokio::main]
async fn main() {
    let server = Server::new().await;
    server.run().await.unwrap();
}
