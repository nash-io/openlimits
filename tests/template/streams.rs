use futures::stream::StreamExt;

use openlimits::model::websocket::Subscription;
use openlimits::exchange::traits::stream::ExchangeWs;

pub async fn orderbook(ws: &impl ExchangeWs) {
    let s = ws
        .create_stream(&[Subscription::OrderBookUpdates("bnbbtc".to_string())])
        .await;

    let ob = s.expect("Couldn't create stream.").next().await;

    print!("{:?}", ob);
}

pub async fn trades(ws: &impl ExchangeWs) {
    let s = ws
        .create_stream(&[Subscription::Trades("bnbbtc".to_string())])
        .await;

    let trades = s.expect("Couldn't create stream.").next().await;

    print!("{:?}", trades);
}
