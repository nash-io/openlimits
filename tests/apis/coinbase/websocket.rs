use futures::StreamExt;
use openlimits::coinbase::{
    client::websocket::CoinbaseWebsocket,
    model::websocket::{ChannelType, Subscription},
};

#[tokio::test]
async fn aggregate_trade() {
    let mut websocket = CoinbaseWebsocket::new("wss://ws-feed.pro.coinbase.com");

    let sub = Subscription {
        channels: vec![ChannelType::Level2],
        product_ids: vec!["BTC-USD".to_string()],
    };

    websocket.subscribe(sub).await.expect("Couldn't subscribe.");

    websocket
        .next()
        .await
        .expect("Couldn't get next.")
        .expect("Couldn't get WebSocket message.");
    websocket
        .next()
        .await
        .expect("Couldn't get next.")
        .expect("Couldn't get WebSocket message.");
}
