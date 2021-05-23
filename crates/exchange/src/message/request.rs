use crate::message::subscription::{Subscription, SubscriptionResponse};

#[derive(Debug)]
pub enum Request {
    Subscription(Subscription)
}

#[derive(Debug)]
pub enum Response {
    Subscription(SubscriptionResponse)
}
