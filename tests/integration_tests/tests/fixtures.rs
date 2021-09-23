use rstest::*;
use singlemalt::server::CuckooFilterService;
use singlemalt::server::approximate_set_service::singlemalt_proto::approximate_set_service_server::ApproximateSetServiceServer;
use singlemalt::server::approximate_set_service::singlemalt_proto::approximate_set_service_client::ApproximateSetServiceClient;
use tonic::transport::{Server, Channel};
use std::time::Duration;

#[fixture]
pub async fn test_client() -> ApproximateSetServiceClient<Channel> {
    let addr = "[::1]:50051".parse().unwrap();
    let filter = CuckooFilterService::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ApproximateSetServiceServer::new(filter))
            .serve(addr)
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    ApproximateSetServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap()
}
