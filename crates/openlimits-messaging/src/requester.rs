use async_trait::async_trait;

#[async_trait]
pub trait Requester {
    type Request;
    type Response;
    type Error;
    async fn request(&mut self, request: &Self::Request) -> Result<Self::Response, Self::Error>;
}
