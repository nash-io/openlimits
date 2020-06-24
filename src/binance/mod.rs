
use crate::exchange::Exchange;
use async_trait::async_trait;

struct Binance {

}
impl Binance {

}

#[async_trait]
impl Exchange for Binance {
    #[tokio::main]
    async fn ping() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://api.binance.com/api/v3/ping")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok("PONG")
    }
}