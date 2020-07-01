use crate::Result;

pub trait Exchange {
    fn ping(&self) -> Result<String>;
    // async fn trade_history(&mut self, CurrencyPair, DateTime, DateTime) -> Result<Vec<Trades>, Box<dyn Error>>;
    // async fn get_markets(&mut self) -> Result<Vec<Market>, Box<dyn Error>>;
    // async fn subscribe_trades(&mut self, Callback) -> Result<Vec<Market>, Box<dyn Error>>;
}
