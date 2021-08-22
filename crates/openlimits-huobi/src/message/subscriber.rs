use messaging::prelude::*;
use crate::Huobi;
use exchange::message::request::Request;

#[async_trait]
impl Subscriber for Huobi {
    type SubscriptionRequest = exchange::message::subscription::Subscription;
    type Publication = exchange::message::subscription::Publication;
    type Error = String;
    async fn subscribe(&mut self, subscription: &Self::SubscriptionRequest) -> Result<Subscription<Self::Publication>, Self::Error> {
        let receiver = self.register_subscription("market.ethbtc.kline.1min".to_string()).await;
        let _response = self.request(&Request::Subscription(subscription.clone())).await?;
        Ok(Box::pin(receiver))
    }
}