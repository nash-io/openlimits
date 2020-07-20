use openlimits::binance::Binance;
use openlimits::exchange::Exchange;
use openlimits::model::OpenLimitOrderRequest;

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: 0.001,
        size: 0.1,
        symbol: "BNBBTC",
    };
    let resp = exchange.limit_buy(req).await.unwrap();
    println!("{:?}", resp);
}

fn init() -> Binance {
    dotenv().ok();
    Binance::with_credential(
        &env::var("BINANCE_API_KEY").unwrap(),
        &env::var("BINANCE_API_SECRET").unwrap(),
        true,
    )
}
