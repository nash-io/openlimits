use cross_test::prelude::*;
use messaging::prelude::*;

cross_test_configure!();

use futures::task::{Context, Poll};
pub use futures::Stream;
pub use std::pin::Pin;

pub struct CustomSubscriber;

#[async_trait]
impl Subscriber for CustomSubscriber {
    type SubscriptionRequest = CustomSubscriptionRequest;
    type Publication = CustomPublication;
    type Error = String;
    async fn subscribe(&mut self, subscription: &Self::SubscriptionRequest) -> Result<Subscription<Self::Publication>, Self::Error> {
        if subscription.valid {
            Ok(Box::pin(CustomSubscription { channel: subscription.channel }))
        } else {
            Err("Invalid subscription.".into())
        }
    }
}

pub struct CustomSubscriptionRequest {
    valid: bool,
    channel: usize
}

#[derive(Debug, PartialEq)]
pub struct CustomSubscription {
    channel: usize
}

impl Stream for CustomSubscription {
    type Item = CustomPublication;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(CustomPublication { channel: self.channel, content: "Valid publication.".into() }))
    }
}

#[derive(Debug, PartialEq)]
pub struct CustomPublication {
    channel: usize,
    content: String
}

#[cross_test]
async fn subscriber() {
    let mut subscriber = CustomSubscriber;

    assert!(subscriber.subscribe(&CustomSubscriptionRequest { valid: false, channel: 1 }).await.is_err());

    let mut subscription = subscriber.subscribe(&CustomSubscriptionRequest { valid: true, channel: 1 }).await.expect("Failed to subscribe.");

    let publication = subscription.next().await.expect("Couldn't get any publication.");
    assert_eq!(publication, CustomPublication { channel: 1, content: "Valid publication.".into() });
}
