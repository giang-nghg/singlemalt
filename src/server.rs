use tonic::{transport::Server, Request, Response, Status};

use singlemalt_proto::approximate_set_service_server::{ApproximateSetService, ApproximateSetServiceServer};
use singlemalt_proto::{InsertReply, InsertRequest};

pub mod singlemalt_proto {
    tonic::include_proto!("singlemalt_proto");
}

#[derive(Debug, Default)]
pub struct CuckooFilterService {}

#[tonic::async_trait]
impl ApproximateSetService for CuckooFilterService {
    async fn insert(
        &self,
        request: Request<InsertRequest>,
    ) -> Result<Response<InsertReply>, Status> {
        let reply = singlemalt_proto::InsertReply {
            message: format!("Hello {}!", request.into_inner().value).into(),
        };

        Ok(Response::new(reply))
    }
}

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