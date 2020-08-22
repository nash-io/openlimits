use openlimits::coinbase::{
    client::websocket::CoinbaseWebsocket,
    model::websocket::{ChannelType, Subscription},
};
use futures::StreamExt;

#[tokio::test]
async fn aggregate_trade() {
    let mut websocket = CoinbaseWebsocket::new("wss://ws-feed.pro.coinbase.com");

    let sub = Subscription {
        channels: vec![ChannelType::Level2],
        product_ids: vec!["BTC-USD".to_string()],
    };

    websocket.subscribe(sub).await.unwrap();

    println!("{:?}", websocket.next().await);
    println!("{:?}", websocket.next().await);
}
