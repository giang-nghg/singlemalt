mod fixtures;

use crate::fixtures::test_client;
use singlemalt::server::approximate_set_service::singlemalt_proto::InsertRequest;

#[tokio::test]
async fn insert_an_element() {
    let mut client = test_client().await;

    let request = tonic::Request::new(InsertRequest {
        value: "Tonic".into(),
    });

    let response = client.insert(request).await.unwrap().into_inner();

    assert_eq!("Tonic", response.message);
}
