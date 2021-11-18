use futures::stream::StreamExt;

use openlimits::model::websocket::{Subscription, WebSocketResponse, OpenLimitsWebSocketMessage};
use openlimits::exchange::traits::stream::ExchangeWs;
use openlimits_exchange::model::market_pair::MarketPair;
use openlimits::exchange::model::currency::Currency;
use tokio::time::timeout;
use std::time::Duration;

pub async fn orderbook(ws: &impl ExchangeWs) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let mut stream = ws
        .create_stream(&[Subscription::OrderBookUpdates(market_pair)])
        .await
        .expect("Failed to create stream.");

    for _ in 0..2 {
        let message_timeout = timeout(Duration::new(2, 0), stream.next()).await;
        if let Ok(message) = message_timeout {
            let message = message
                .expect("Failed to stream trades.")
                .expect("Stream error.");
            match message {
                WebSocketResponse::Generic(OpenLimitsWebSocketMessage::OrderBook(orderbook)) => {
                    println!("{:#?}", orderbook);
                },
                _ => panic!("Incorrect message: {:#?}", message)
            }
        }
    }
}

pub async fn trades(ws: &impl ExchangeWs) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let mut stream = ws
        .create_stream(&[Subscription::Trades(market_pair)])
        .await
        .expect("Couldn't create stream.");

    for _ in 0..2 {
        let message_timeout = timeout(Duration::new(2, 0), stream.next()).await;
        if let Ok(message) = message_timeout {
            let message = message
                .expect("Failed to stream trades.")
                .expect("Stream error.");
            match message {
                WebSocketResponse::Generic(OpenLimitsWebSocketMessage::Trades(trades)) => {
                    println!("{:#?}", trades);
                },
                _ => panic!("Incorrect message: {:#?}", message)
            }
        }
    }
}
