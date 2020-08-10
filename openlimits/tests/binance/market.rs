use openlimits::binance::Binance;
use openlimits::exchange::Exchange;
use openlimits::model::{GetPriceTickerRequest, OrderBookRequest};

#[tokio::test]
async fn order_book() {
    let exchange = Binance::new(true);
    let req = OrderBookRequest {
        symbol: "BNBBTC".to_string(),
    };
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = Binance::new(true);
    let req = GetPriceTickerRequest {
        symbol: "BNBBTC".to_string(),
    };
    let resp = exchange.get_price_ticker(&req).await.unwrap();
    println!("{:?}", resp);
}
