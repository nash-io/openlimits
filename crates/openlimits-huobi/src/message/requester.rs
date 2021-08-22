use messaging::prelude::*;
use exchange::message::request::{Request};
use exchange::message::request::Response;
use exchange::message::subscription::SubscriptionResponse;
use crate::{Huobi, HuobiSubscription};
use nash_ws::Message;

#[async_trait]
impl Requester for Huobi {
    type Request = Request;
    type Response = Response;
    type Error = String;
    async fn request(&mut self, request: &Self::Request) -> Result<Self::Response, Self::Error> {
        let (id, mut receiver) = self.register_request().await;
        let request_message = match request {
            Request::Subscription(_subscription) => {
                let sub = format!("market.{}{}.kline.1min", "eth", "btc");
                let request = HuobiSubscription { id, sub };
                serde_json::to_string(&request)
                    .map_err(|error| {
                        format!("{:#?}", error)
                    })?
            }
        };
        self.sender.send(&Message::Text(request_message)).await.expect("Couldn't send message.");
        let response = receiver.next().await;
        response
            .filter(|response| response.status == "ok")
            .map(|_response| {
                Response::Subscription(SubscriptionResponse{})
            })
            .ok_or_else(|| {
                // FIXME: Also implement timeout.
                "Failed to receive resposne".into()
            })
    }
}
