use crate::model::websocket::Subscription;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoinbaseSubscription {
    Heartbeat(String),
    Status,
    // Ticker(String),
    Level2(String),
    // User,
    // Matches,
    // FullChannel
}

impl From<Subscription> for CoinbaseSubscription {
    fn from(subscription: Subscription) -> Self {
        match subscription {
            Subscription::OrderBookUpdates(symbol) => CoinbaseSubscription::Level2(symbol),
            _ => unimplemented!(),
        }
    }
}