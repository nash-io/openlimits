use async_trait::async_trait;

use std::pin::Pin;
use futures::stream::Stream;

pub type Subscription<T> = Pin<Box<dyn Stream<Item = T>>>;

#[async_trait]
pub trait Subscriber {
    type SubscriptionRequest;
    type Publication;
    type Error;
    async fn subscribe(&mut self, subscription: &Self::SubscriptionRequest) -> Result<Subscription<Self::Publication>, Self::Error>;
}