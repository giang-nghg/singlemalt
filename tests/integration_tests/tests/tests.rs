mod common;

#[cfg(test)]
mod tests {
    use crate::common::server;
    use singlemalt::server::approximate_set_service::singlemalt_proto::approximate_set_service_client::ApproximateSetServiceClient;
    use singlemalt::server::approximate_set_service::singlemalt_proto::InsertRequest;
    use singlemalt::server::approximate_set_service::singlemalt_proto::approximate_set_service_server::ApproximateSetService;

    #[test]
    fn insert_an_element() {
        let server = server();
        let request = tonic::Request::new(InsertRequest {
            value: "Tonic".into(),
        });

        server.insert(request);
    }
}
