use openlimits::coinbase::{Coinbase, CoinbaseParameters};
use openlimits::exchange::OpenLimits;
use openlimits::exchange_info::ExchangeInfoRetrieval;
use openlimits::exchange::Exchange;

#[tokio::test]
async fn retrieve_pairs() {
    // let exchange = init().await;
    let exchange = Coinbase::new(CoinbaseParameters::prod()).await.expect("Couldn't create exchange.");
    let result = exchange.retrieve_pairs().await.expect("Couldn't retrieve pairs");
    println!("{:#?}", result);
}

async fn init() -> Coinbase {
    let parameters = CoinbaseParameters {
        credentials: None,
        sandbox: true,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}
