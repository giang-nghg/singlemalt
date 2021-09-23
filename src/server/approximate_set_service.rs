pub mod singlemalt_proto {
    tonic::include_proto!("singlemalt_proto");
}

use tonic::{Request, Response, Status};

use singlemalt_proto::approximate_set_service_server::ApproximateSetService;
use singlemalt_proto::{InsertReply, InsertRequest};

#[derive(Debug, Default)]
pub struct CuckooFilterService {}

#[tonic::async_trait]
impl ApproximateSetService for CuckooFilterService {
    async fn insert(
        &self,
        request: Request<InsertRequest>,
    ) -> Result<Response<InsertReply>, Status> {
        let reply = singlemalt_proto::InsertReply {
            message: format!("{}", request.into_inner().value).into(),
        };

        Ok(Response::new(reply))
    }
}
