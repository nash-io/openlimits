// use {hashbrown::HashMap, std::error::Error};
// use chrono::DateTime;

use std::error::Error;
use async_trait::async_trait;

//use crate::client::Client;

// pub struc Exchange {
//     client: Client
//     // socket
//     // list of subsribed trades
//     // ...
// }

#[async_trait]
pub trait Exchange {
    async fn ping() -> Result<String, Box<dyn Error>>;
    // async fn trade_history(&mut self, CurrencyPair, DateTime, DateTime) -> Result<Vec<Trades>, Box<dyn Error>>; 
    // async fn get_markets(&mut self) -> Result<Vec<Market>, Box<dyn Error>>;
    // async fn subscribe_trades(&mut self, Callback) -> Result<Vec<Market>, Box<dyn Error>>; 
}
