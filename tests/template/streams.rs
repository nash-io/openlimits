use futures::stream::StreamExt;

use openlimits::model::websocket::Subscription;
use openlimits::exchange::traits::stream::ExchangeWs;
use openlimits_exchange::model::market_pair::MarketPair;
use openlimits::exchange::model::currency::Currency;

pub async fn orderbook(ws: &impl ExchangeWs) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let s = ws
        .create_stream(&[Subscription::OrderBookUpdates(market_pair)])
        .await;

    let ob = s.expect("Couldn't create stream.").next().await;

    print!("{:?}", ob);
}

pub async fn trades(ws: &impl ExchangeWs) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let mut s = ws
        .create_stream(&[Subscription::Trades(market_pair)])
        .await
        .expect("Couldn't create stream.");

    let trades = s.next().await;
    print!("{:?}", trades);
    let trades = s.next().await;
    print!("{:?}", trades);
}
