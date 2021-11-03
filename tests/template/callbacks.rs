use std::sync::mpsc::sync_channel;

use openlimits::model::websocket::Subscription;
use openlimits::exchange::traits::stream::ExchangeWs;
use openlimits_exchange::model::market_pair::MarketPair;
use openlimits::model::currency::Currency;

async fn test_subscription_callback(websocket: &impl ExchangeWs, sub: Subscription) {
    let (tx, rx) = sync_channel(0);

    websocket
        .subscribe(sub, move |m| {
            m.as_ref().expect("Failed to get response.");
            tx.send(()).expect("Failed to send sync message.");
        })
        .await
        .expect("Failed to subscribe.");

    rx.recv().expect("Failed to receive sync message.");
}

pub async fn orderbook(ws: &impl ExchangeWs) {
    let market = MarketPair(Currency::ETH, Currency::BTC);
    let sub = Subscription::OrderBookUpdates(market);
    test_subscription_callback(ws, sub).await;
}

pub async fn trades(ws: &impl ExchangeWs) {
    let market = MarketPair(Currency::ETH, Currency::BTC);
    let sub = Subscription::Trades(market);
    test_subscription_callback(ws, sub).await;
}
