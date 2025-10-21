pub mod middleware;

use crate::middleware::pb::echo_server::EchoServer as EchoService;
use crate::middleware::{EchoServer, check_auth};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let server = EchoServer::default();

    let svc = EchoService::with_interceptor(server, check_auth);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
