use openlimits::exchange::coinbase::Coinbase;
use openlimits::exchange::coinbase::CoinbaseParameters;
use openlimits::prelude::*;

#[tokio::main]
async fn main() {
    let coinbase = Coinbase::new(CoinbaseParameters::prod())
                        .await
                        .expect("Couldn't create coinbase client");

    let order_book = coinbase.order_book(&OrderBookRequest {market_pair: "BTC-EUR".to_string()})
                        .await
                        .expect("Couldn't get order book");

    println!("{:?}", order_book);
}