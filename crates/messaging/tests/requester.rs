use cross_test::prelude::*;
use messaging::prelude::*;

cross_test_configure!();

pub struct CustomRequester;

#[derive(Debug, PartialEq)]
pub struct CustomRequest {
    valid: bool
}

#[derive(Debug, PartialEq)]
pub struct CustomResponse {
    content: String
}

#[async_trait]
impl Requester for CustomRequester {
    type Request = CustomRequest;
    type Response = CustomResponse;
    type Error = String;
    async fn request(&mut self, request: &Self::Request) -> Result<Self::Response, Self::Error> {
        if request.valid {
            Ok(CustomResponse {
                content: "Valid request.".into()
            })
        } else {
            Err("Invalid request.".into())
        }
    }
}

#[cross_test]
async fn requester() {
    let mut requester = CustomRequester;
    assert_eq!(requester.request(&CustomRequest { valid: true  }).await, Ok(CustomResponse { content: "Valid request.".into() }));
    assert_eq!(requester.request(&CustomRequest { valid: false }).await, Err("Invalid request.".into()));
}