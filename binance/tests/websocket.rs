use binance::client::websocket::BinanceWebsocket;
use binance::model::websocket::Subscription;
use futures::StreamExt;

#[tokio::test]
async fn aggregate_trade() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::AggregateTrade("bnbbtc".to_string());

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn candlestick() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::Candlestick("bnbbtc".to_string(), "1m".to_string());

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn depth() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::Depth("bnbbtc".to_string(), Some(1));

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn mini_ticker() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::MiniTicker("bnbbtc".to_string());

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn mini_ticker_all() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::MiniTickerAll;

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn order_book() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::OrderBook("bnbbtc".to_string(), 5);

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn ticker() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::Ticker("bnbbtc".to_string());

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn ticker_all() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::TickerAll;

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
}

#[tokio::test]
async fn trade() {
    let mut websocket = BinanceWebsocket::new();
    let sub = Subscription::Trade("bnbbtc".to_string());

    websocket.subscribe(sub).await.unwrap();
    println!("{:?}", websocket.next().await);
}
