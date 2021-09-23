pub mod approximate_set_service;

pub use approximate_set_service::CuckooFilterService;

use tonic::transport::Server;

use approximate_set_service::singlemalt_proto::approximate_set_service_server::ApproximateSetServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let filter = CuckooFilterService::default();

    Server::builder()
        .add_service(ApproximateSetServiceServer::new(filter))
        .serve(addr)
        .await?;

    Ok(())
}
