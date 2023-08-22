use tonic::transport::Server;

use crate::server::user_service::user_service_server::UserServiceServer;

pub mod entity;
pub mod service;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:7001".parse()?;
    let provider = server::UserServiceProvider::default();

    Server::builder()
        .add_service(UserServiceServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}
