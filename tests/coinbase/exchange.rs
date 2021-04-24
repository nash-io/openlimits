use openlimits::exchange::coinbase::{Coinbase, CoinbaseParameters};
use openlimits::OpenLimits;
use openlimits::prelude::*;

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
