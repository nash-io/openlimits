use serde_json::json;
use serde_json::Value;
use super::BaseClient;
use crate::{
    exchange::binance::model::{
        BookTickers, KlineParams, KlineSummaries, KlineSummary, OrderBook, PriceStats, Prices,
        SymbolPrice, Ticker,
    },
    errors::OpenLimitsError,
};
use rust_decimal::prelude::Decimal;
use super::shared::Result;

// Market Data endpoints
impl BaseClient {
    // Order book (Default 100; max 100)
    pub async fn get_depth<I>(&self, symbol: &str, limit: I) -> Result<OrderBook>
    where
        I: Into<Option<u64>>,
    {
        let limit = limit.into().unwrap_or(100);
        let params = json! {{"symbol": symbol, "limit": limit}};

        Ok(self.transport.get("/api/v3/depth", Some(&params)).await?)
    }

    // Latest price for ALL symbols.
    pub async fn get_all_prices(&self) -> Result<Prices> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v3/ticker/price", None)
            .await?)
    }

    // Latest price for ONE symbol.
    pub async fn get_price(&self, symbol: &str) -> Result<SymbolPrice> {
        let params = json! {{"symbol": symbol}};

        let price = self
            .transport
            .get("/api/v3/ticker/price", Some(&params))
            .await?;

        Ok(price)
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub async fn get_all_book_tickers(&self) -> Result<BookTickers> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v3/ticker/bookTicker", None)
            .await?)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub async fn get_book_ticker(&self, symbol: &str) -> Result<Ticker> {
        let symbol = symbol.to_string();
        self.get_all_book_tickers().await.and_then(
            move |BookTickers::AllBookTickers(book_tickers)| {
                Ok(book_tickers
                    .into_iter()
                    .find(|obj| obj.symbol == symbol)
                    .ok_or(OpenLimitsError::SymbolNotFound())?)
            },
        )
    }

    // 24hr ticker price change statistics
    pub async fn get_24h_price_stats(&self, symbol: &str) -> Result<PriceStats> {
        let params = json! {{"symbol": symbol}};
        Ok(self
            .transport
            .get("/api/v3/ticker/24hr", Some(&params))
            .await?)
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub async fn get_klines(&self, params: &KlineParams) -> Result<KlineSummaries> {
        self.transport
            .get("/api/v3/klines", Some(params))
            .await
            .map(|data: Vec<Vec<Value>>| {
                KlineSummaries::AllKlineSummaries(
                    data.iter()
                        .map(|row| KlineSummary {
                            open_time: to_i64(&row[0]),
                            open: to_decimal(&row[1]),
                            high: to_decimal(&row[2]),
                            low: to_decimal(&row[3]),
                            close: to_decimal(&row[4]),
                            volume: to_decimal(&row[5]),
                            close_time: to_i64(&row[6]),
                            quote_asset_volume: to_decimal(&row[7]),
                            number_of_trades: to_i64(&row[8]),
                            taker_buy_base_asset_volume: to_decimal(&row[9]),
                            taker_buy_quote_asset_volume: to_decimal(&row[10]),
                        })
                        .collect(),
                )
            })
    }

    // 24hr ticker price change statistics
    pub async fn get_24h_price_stats_all(&self) -> Result<Vec<PriceStats>> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v3/ticker/24hr", None)
            .await?)
    }
}

fn to_i64(v: &Value) -> i64 {
    v.as_i64().expect("Couldn't get JSON Value as i64.")
}

fn to_decimal(v: &Value) -> Decimal {
    v.as_str()
        .expect("Couldn't get JSON Value as str.")
        .parse()
        .expect("Couldn't parse str as Decimal.")
}
