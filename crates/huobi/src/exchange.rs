use async_trait::async_trait;

mod symbol;
pub use symbol::*;
use crate::{Request, Response, Subscription};
use futures::stream::Stream;
use std::pin::Pin;

#[async_trait]
pub trait Exchange: Subscriber {
    type InitializationParameters;

    fn endpoint_url(environment: Environment) -> &'static str;
    async fn new(parameters: Self::InitializationParameters) -> Result<Self> where Self: Sized;
}

#[async_trait]
pub trait Requester {
    async fn request(&mut self, request: &Request) -> Result<Response>;
}

#[async_trait]
pub trait Subscriber: Requester {
    async fn subscribe(&mut self, subscription: Subscription) -> Result<SubscriptionStream>;
}

pub enum Environment {
    Production,
    Sandbox
}

#[derive(Debug)]
pub enum SubscriptionUpdate {
    Tick
}

pub type SubscriptionStream = Pin<Box<dyn Stream<Item = SubscriptionUpdate>>>;

#[derive(Debug)]
pub struct MarketPair(pub Symbol, pub Symbol);

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;