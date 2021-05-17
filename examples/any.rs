use openlimits::exchange::any::AnyExchange;
use openlimits::exchange::any::InitAnyExchange;
use openlimits::exchange::binance::BinanceParameters;
use openlimits::prelude::*;

#[tokio::main]
async fn main() {
    // Binance, Coinbase and Nash availables
    let binance = AnyExchange::new(InitAnyExchange::Binance(BinanceParameters::prod()))
                    .await
                    .expect("Couldn't create binance client");

    let order_book = binance.order_book(&OrderBookRequest {market_pair: "BTCEUR".to_string()})
                    .await
                    .expect("Couldn't get order book");

    println!("{:?}", order_book);
}