use openlimits::coinbase::Coinbase;
use openlimits::exchange::Exchange;
use openlimits::model::OrderBookRequest;

#[tokio::test]
async fn order_book() {
    let exchange = Coinbase::new(true);
    let req = OrderBookRequest {
        symbol: "ETH-BTC".to_string(),
    };
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}
