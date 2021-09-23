mod fixtures;

#[macro_use]
mod macros;

use crate::fixtures::test_client;
use singlemalt::server::approximate_set_service::singlemalt_proto::InsertRequest;
use proptest::prelude::*;

proptest! {
    #[test]
    fn insert_an_element(s in "\\PC*") {
        async fn test(s: &str) {
            let mut client = test_client().await;

            let request = tonic::Request::new(InsertRequest {
                value: s.into(),
            });

            let response = client.insert(request).await.unwrap().into_inner();

            assert_eq!(s, response.message);
        }
        await_test!(test(&s))
    }
}
