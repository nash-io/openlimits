use openlimits::{
    binance::{client::websocket::BinanceWebsocket, model::websocket::BinanceSubscription},
    exchange_ws::ExchangeWs,
};
use std::sync::mpsc::sync_channel;

#[tokio::test]
async fn aggregate_trade() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::AggregateTrade("bnbbtc".to_string());

    websocket
        .subscribe(sub, move |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test(core_threads = 2)]
async fn candlestick() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Candlestick("bnbbtc".to_string(), "1m".to_string());

    websocket
        .subscribe(sub, move |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test]
async fn depth() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Depth("bnbbtc".to_string(), Some(1));

    websocket
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test]
async fn mini_ticker() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::MiniTicker("bnbbtc".to_string());

    websocket
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test]
async fn mini_ticker_all() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::MiniTickerAll;

    websocket
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test(core_threads = 2)]
async fn order_book() {
    let (tx, rx) = sync_channel(0);
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::OrderBook("bnbbtc".to_string(), 10);

    websocket
        .subscribe(sub, move |m| {
            println!("{:?}", m);
            tx.send(()).unwrap();
        })
        .await
        .unwrap();

    rx.recv().unwrap();
}

#[tokio::test]
async fn ticker() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Ticker("bnbbtc".to_string());

    websocket
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test]
async fn ticker_all() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::TickerAll;

    websocket
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test]
async fn trade() {
    let websocket = BinanceWebsocket::new();
    let sub = BinanceSubscription::Trade("bnbbtc".to_string());

    websocket
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}
