use openlimits::coinbase::Coinbase;
use openlimits::exchange::Exchange;
use openlimits::model::{GetPriceTickerRequest, OrderBookRequest};

#[tokio::test]
async fn order_book() {
    let exchange = Coinbase::new(true);
    let req = OrderBookRequest {
        symbol: "ETH-BTC".to_string(),
    };
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = Coinbase::new(true);
    let req = GetPriceTickerRequest {
        symbol: "ETH-BTC".to_string(),
    };
    let resp = exchange.get_price_ticker(&req).await.unwrap();
    println!("{:?}", resp);
}
