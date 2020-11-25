use openlimits::{
    binance::{client::websocket::BinanceWebsocket, model::websocket::BinanceSubscription},
    exchange_ws::ExchangeWs,
};
use std::sync::mpsc::sync_channel;

async fn test_subscription_callback(websocket: BinanceWebsocket, sub: BinanceSubscription) {
    let (tx, rx) = sync_channel(0);

    websocket.subscribe(sub, move |m| {
        m.as_ref().expect("Couldn't get response.");
        tx.send(()).expect("Couldn't send sync message.");
    }).await.expect("Couldn't subscribe.");

    rx.recv().expect("Couldn't receive sync message.");
}

#[tokio::test(core_threads = 2)]
async fn aggregate_trade() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::AggregateTrade("bnbbtc".to_string());
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn candlestick() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Candlestick("bnbbtc".to_string(), "1m".to_string());
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn depth() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Depth("bnbbtc".to_string(), Some(1));
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn mini_ticker() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::MiniTicker("bnbbtc".to_string());
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn mini_ticker_all() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::MiniTickerAll;
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn order_book() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::OrderBook("bnbbtc".to_string(), 10);
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn ticker() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Ticker("bnbbtc".to_string());
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn ticker_all() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::TickerAll;
    test_subscription_callback(websocket, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn trade() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Trade("bnbbtc".to_string());
    test_subscription_callback(websocket, sub).await;
}
