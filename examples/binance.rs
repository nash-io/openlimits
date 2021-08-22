use openlimits::exchange::binance::Binance;
use openlimits::exchange::binance::BinanceParameters;
use openlimits::prelude::*;

#[tokio::main]
async fn main() {
    let binance = Binance::new(BinanceParameters::prod())
                        .await
                        .expect("Couldn't create openlimits-binance client");

    let order_book = binance.order_book(&OrderBookRequest {market_pair: "BTCEUR".to_string()})
                        .await
                        .expect("Couldn't get order book");

    println!("{:?}", order_book);
}