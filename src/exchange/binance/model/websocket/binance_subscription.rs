/// Used for subscriptions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinanceSubscription {
    /// Listen key
    UserData(String),   
    /// Symbol         
    AggregateTrade(String),      
    /// Symbol
    Trade(String),  
    /// (Symbol, Interval)             
    Candlestick(String, String), 
    /// Symbol
    MiniTicker(String),          
    MiniTickerAll,
    /// Symbol
    Ticker(String), 
    TickerAll,
    /// (Symbol, Depth)
    OrderBook(String, i64), 
    /// (Symbol, Interval)
    Depth(String, Option<u16>), 
}