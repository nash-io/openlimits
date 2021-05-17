pub use nash_native_client::{Client, Environment};
use nash_protocol::protocol::subscriptions::SubscriptionResponse;

#[derive(Debug)]
pub struct SubscriptionResponseWrapper(pub SubscriptionResponse);

impl Clone for SubscriptionResponseWrapper {
    fn clone(&self) -> Self {
        match &self.0 {
            SubscriptionResponse::Orderbook(o) => {
                SubscriptionResponseWrapper(SubscriptionResponse::Orderbook(o.clone()))
            }
            SubscriptionResponse::Trades(t) => {
                SubscriptionResponseWrapper(SubscriptionResponse::Trades(t.clone()))
            }
            SubscriptionResponse::Ticker(ticker) => {
                SubscriptionResponseWrapper(SubscriptionResponse::Ticker(ticker.clone()))
            }
            SubscriptionResponse::AccountBalances(balances) => {
                SubscriptionResponseWrapper(SubscriptionResponse::AccountBalances(balances.clone()))
            }
            SubscriptionResponse::AccountOrders(orders) => {
                SubscriptionResponseWrapper(SubscriptionResponse::AccountOrders(orders.clone()))
            }
            SubscriptionResponse::AccountTrades(trades) => {
                SubscriptionResponseWrapper(SubscriptionResponse::AccountTrades(trades.clone()))
            }
        }
    }
}