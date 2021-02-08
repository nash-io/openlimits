use crate::MarketPair;

#[derive(Debug)]
pub enum Request {
    Subscription(Subscription)
}

#[derive(Debug)]
pub enum Response {
    Subscription(SubscriptionResponse)
}

#[derive(Debug)]
pub struct SubscriptionResponse {

}

#[derive(Debug)]
pub enum Subscription {
    OrderBook(MarketPair)
}
