use openlimits::{
    binance::{model::KlineParams, Binance, BinanceParameters},
    exchange::Exchange,
};

#[tokio::test]
async fn get_depth() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_depth("BNBBTC", 50)
        .await
        .expect("Couldn't get depth.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_prices() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_all_prices()
        .await
        .expect("Couldn't get all prices.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_price("BNBBTC")
        .await
        .expect("Couldn't get price.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_book_tickers() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_all_book_tickers()
        .await
        .expect("Couldn't get all book tickers.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_book_ticker() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_book_ticker("BNBBTC")
        .await
        .expect("Couldn't get book ticker.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_24h_price_stats() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_24h_price_stats("BNBBTC")
        .await
        .expect("Couldn't get 24h price stats.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_klines() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let params = KlineParams {
        symbol: String::from("BNBBTC"),
        interval: String::from("5m"),
        paginator: None,
    };
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_klines(&params)
        .await
        .expect("Couldn't get klines.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_24h_price_stats_all() {
    let exchange = Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_24h_price_stats_all()
        .await
        .expect("Couldn't get 24h price stats all.");
    println!("{:?}", resp);
}
