#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinanceSubscription {
    UserData(String),            // listen key
    AggregateTrade(String),      // symbol
    Trade(String),               // symbol
    Candlestick(String, String), // (symbol, interval)
    MiniTicker(String),          // symbol
    MiniTickerAll,
    Ticker(String), // symbol
    TickerAll,
    OrderBook(String, i64),     // (symbol, depth)
    Depth(String, Option<u16>), // (symbol, interval)
}