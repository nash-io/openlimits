use crate::market::MarketPair;

#[derive(Debug, Clone)]
pub enum Subscription {
    Tick(MarketPair)
}

#[derive(Debug, Clone)]
pub struct SubscriptionResponse {}

#[derive(Debug, Clone)]
pub enum Publication {
    Tick
}
