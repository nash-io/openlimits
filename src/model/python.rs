use super::websocket::{OpenLimitsWebSocketMessage, Subscription};
use crate::any_exchange::InitAnyExchange;
use crate::binance::{BinanceCredentials, BinanceParameters};
use crate::coinbase::{CoinbaseCredentials, CoinbaseParameters};
use crate::model::{Interval, Paginator, TimeInForce};
use crate::nash::{Environment, NashCredentials, NashParameters};
use pyo3::exceptions::PyException;
use pyo3::prelude::{FromPyObject, IntoPy, PyObject, PyResult, Python, ToPyObject};
use pyo3::types::PyDict;
use std::time::Duration;

// Python to Rust...

impl<'a> FromPyObject<'a> for TimeInForce {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let maybe_interval: PyResult<Interval> = ob.get_item("time_in_force")?.extract();
        if let Ok(interval) = maybe_interval {
            Ok(TimeInForce::GoodTillTime(interval.into()))
        } else {
            let value: String = ob.get_item("time_in_force")?.extract()?;
            match &value[..] {
                "good_til_cancelled" => Ok(TimeInForce::GoodTillCancelled),
                "immediate_or_cancelled" => Ok(TimeInForce::ImmediateOrCancelled),
                "fill_or_kill" => Ok(TimeInForce::FillOrKill),
                _ => Err(PyException::new_err("Invalid time in force")),
            }
        }
    }
}

impl<'a> FromPyObject<'a> for InitAnyExchange {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        // unfortunately can't do the if let on the extract() since it needs type annotations
        if let Ok(nash) = ob.extract() {
            return Ok(InitAnyExchange::Nash(nash));
        }
        let maybe_binance: PyResult<BinanceParameters> = ob.extract();
        if let Ok(binance) = maybe_binance {
            return Ok(InitAnyExchange::Binance(binance));
        }
        let maybe_coinbase: PyResult<CoinbaseParameters> = ob.extract();
        if let Ok(coinbase) = maybe_coinbase {
            return Ok(InitAnyExchange::Coinbase(coinbase));
        }
        Err(PyException::new_err(
            "invalid exchange initialization params",
        ))
    }
}

impl<'a> FromPyObject<'a> for BinanceCredentials {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let py_dict = ob.get_item("binance_credentials")?.downcast::<PyDict>()?;
        let api_key: String = py_dict
            .get_item("api_key")
            .ok_or(PyException::new_err(
                "secret not included in binance credentials",
            ))?
            .extract()?;
        let api_secret: String = py_dict
            .get_item("api_secret")
            .ok_or(PyException::new_err(
                "session not included in binance credentials",
            ))?
            .extract()?;
        Ok(BinanceCredentials {
            api_key,
            api_secret,
        })
    }
}

impl<'a> FromPyObject<'a> for BinanceParameters {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let py_dict = ob.get_item("binance")?.downcast::<PyDict>()?;
        let credentials: Option<BinanceCredentials> = py_dict
            .get_item("credentials")
            .ok_or(PyException::new_err(
                "credentials not included in binance params",
            ))?
            .extract()?;
        let sandbox: bool = py_dict
            .get_item("sandbox")
            .ok_or(PyException::new_err(
                "session not included in binance credentials",
            ))?
            .extract()?;
        Ok(BinanceParameters {
            sandbox,
            credentials,
        })
    }
}

impl<'a> FromPyObject<'a> for CoinbaseCredentials {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let py_dict = ob.get_item("coinbase_credentials")?.downcast::<PyDict>()?;
        let api_key: String = py_dict
            .get_item("api_key")
            .ok_or(PyException::new_err(
                "secret not included in coinbase credentials",
            ))?
            .extract()?;
        let api_secret: String = py_dict
            .get_item("api_secret")
            .ok_or(PyException::new_err(
                "session not included in coinbase credentials",
            ))?
            .extract()?;
        let passphrase: String = py_dict
            .get_item("passphrase")
            .ok_or(PyException::new_err(
                "passphrase not included in coinbase credentials",
            ))?
            .extract()?;
        Ok(CoinbaseCredentials {
            api_key,
            api_secret,
            passphrase,
        })
    }
}

impl<'a> FromPyObject<'a> for CoinbaseParameters {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let py_dict = ob.get_item("coinbase")?.downcast::<PyDict>()?;
        let credentials: Option<CoinbaseCredentials> = py_dict
            .get_item("credentials")
            .ok_or(PyException::new_err(
                "credentials not included in coinbase params",
            ))?
            .extract()?;
        let sandbox: bool = py_dict
            .get_item("sandbox")
            .ok_or(PyException::new_err(
                "session not included in coinbase credentials",
            ))?
            .extract()?;
        Ok(CoinbaseParameters {
            sandbox,
            credentials,
        })
    }
}

impl<'a> FromPyObject<'a> for NashCredentials {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let py_dict = ob.get_item("nash_credentials")?.downcast::<PyDict>()?;
        let secret: String = py_dict
            .get_item("secret")
            .ok_or(PyException::new_err(
                "secret not included in nash credentials",
            ))?
            .extract()?;
        let session: String = py_dict
            .get_item("session")
            .ok_or(PyException::new_err(
                "session not included in nash credentials",
            ))?
            .extract()?;
        Ok(NashCredentials { secret, session })
    }
}

impl<'a> FromPyObject<'a> for NashParameters {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let py_dict = ob.get_item("nash")?.downcast::<PyDict>()?;
        let credentials: Option<NashCredentials> = py_dict
            .get_item("credentials")
            .ok_or(PyException::new_err(
                "credentials not included in nash params",
            ))?
            .extract()?;
        let client_id: u64 = py_dict
            .get_item("client_id")
            .ok_or(PyException::new_err(
                "session not included in nash credentials",
            ))?
            .extract()?;
        let env_str: String = py_dict
            .get_item("environment")
            .ok_or(PyException::new_err(
                "session not included in nash credentials",
            ))?
            .extract()?;
        let affiliate_code: Option<String> = py_dict
            .get_item("affiliate_code")
            .ok_or(PyException::new_err(
                "affiliate_code not included in nash params",
            ))?
            .extract()?;
        let environment = match &env_str[..] {
            "sandbox" => Ok(Environment::Sandbox),
            "production" => Ok(Environment::Production),
            _ => Err(PyException::new_err("not a valid nash environment")),
        }?;
        let timeout: u64 = py_dict
            .get_item("timeout")
            .ok_or(PyException::new_err(
                "timeout not included in nash credentials",
            ))?
            .extract()?;
        let timeout = Duration::from_millis(timeout);
        let sign_states_loop_interval: Option<u64> = py_dict
            .get_item("timeout")
            .ok_or(PyException::new_err(
                "sign states loop interval not included in nash credentials",
            ))?
            .extract()?;
        Ok(NashParameters {
            affiliate_code,
            credentials,
            client_id,
            environment,
            timeout,
            sign_states_loop_interval,
        })
    }
}

impl<'a> FromPyObject<'a> for Interval {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let interval_str: String = ob.extract()?;
        match &interval_str[..] {
            "1m" => Ok(Interval::OneMinute),
            "3m" => Ok(Interval::ThreeMinutes),
            "5m" => Ok(Interval::FiveMinutes),
            "15m" => Ok(Interval::FifteenMinutes),
            "30m" => Ok(Interval::ThirtyMinutes),
            "1h" => Ok(Interval::OneHour),
            "2h" => Ok(Interval::TwoHours),
            "4h" => Ok(Interval::FourHours),
            "6h" => Ok(Interval::SixHours),
            "8h" => Ok(Interval::EightHours),
            "12h" => Ok(Interval::TwelveHours),
            "1d" => Ok(Interval::OneDay),
            "3d" => Ok(Interval::ThreeDays),
            "1w" => Ok(Interval::OneWeek),
            "1mo" => Ok(Interval::OneMonth),
            _ => Err(PyException::new_err("Interval value not supported")),
        }
    }
}

impl<'a> FromPyObject<'a> for Paginator {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let page_values = ob.get_item("paginator")?.downcast::<PyDict>()?;
        let start_time: Option<u64> = match page_values.get_item("start_time") {
            Some(value) => value.extract()?,
            None => None,
        };
        let end_time: Option<u64> = match page_values.get_item("end_time") {
            Some(value) => value.extract()?,
            None => None,
        };
        let limit: Option<u64> = match page_values.get_item("limit") {
            Some(value) => value.extract()?,
            None => None,
        };
        let after: Option<String> = match page_values.get_item("after") {
            Some(value) => value.extract()?,
            None => None,
        };

        let before: Option<String> = match page_values.get_item("before") {
            Some(value) => value.extract()?,
            None => None,
        };
        Ok(Paginator {
            start_time,
            end_time,
            after,
            before,
            limit,
        })
    }
}

impl<'a> FromPyObject<'a> for Subscription {
    fn extract(pyobj: &'a pyo3::PyAny) -> PyResult<Self> {
        // we will simulate an enum in Python via dictionary keys
        if let Ok(trade) = pyobj.get_item("trade") {
            Ok(Subscription::Trades(trade.extract()?))
        } else if let Ok(orderbook) = pyobj.get_item("orderbook") {
            let orderbook_args: (String, i64) = orderbook.extract()?;
            Ok(Subscription::OrderBookUpdates(orderbook_args.0))
        } else {
            Err(PyException::new_err("Not a supported input subscription"))
        }
    }
}

impl ToPyObject for OpenLimitsWebSocketMessage {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            OpenLimitsWebSocketMessage::Ping => {
                // empty dict to represent null
                let dict = PyDict::new(py);
                dict.set_item("ping", PyDict::new(py))
                    .expect("Couldn't set ping.");
                dict.into()
            }
            OpenLimitsWebSocketMessage::OrderBook(resp) => resp.to_object(py),
            OpenLimitsWebSocketMessage::OrderBookDiff(resp) => resp.to_object(py),
            OpenLimitsWebSocketMessage::Trades(resp) => resp.to_object(py),
        }
    }
}

impl IntoPy<PyObject> for OpenLimitsWebSocketMessage {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

// Rust to Python... (Responses)

use super::super::exchange_info::MarketPair;
use super::{
    AskBid, Balance, Candle, Liquidity, Order, OrderBookResponse, OrderCanceled, OrderStatus,
    OrderType, Side, Ticker, Trade,
};

impl ToPyObject for OrderType {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_str = match self {
            Self::Limit => "limit",
            Self::Market => "market",
            Self::StopLimit => "stop_limit",
            Self::StopMarket => "stop_market",
            Self::Unknown => "unknown",
        };
        dict.set_item("order_type", inner_str)
            .expect("Couldn't set order_type.");
        dict.into()
    }
}

impl IntoPy<PyObject> for OrderType {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for Ticker {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        // TODO: why does ticker have so few fields?
        inner_dict
            .set_item(
                "price",
                self.price.map_or(String::from("0.0"), |f| f.to_string()),
            )
            .expect("Couldn't set price.");
        dict.set_item("ticker", inner_dict)
            .expect("Couldn't set ticker.");
        dict.into()
    }
}

impl IntoPy<PyObject> for Ticker {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for Candle {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("low", self.low.to_string())
            .expect("Couldn't set low.");
        inner_dict
            .set_item("high", self.high.to_string())
            .expect("Couldn't set high.");
        inner_dict
            .set_item("close", self.close.to_string())
            .expect("Couldn't set close.");
        inner_dict
            .set_item("open", self.open.to_string())
            .expect("Couldn't set open.");
        inner_dict
            .set_item("time", self.time)
            .expect("Couldn't set time.");
        inner_dict
            .set_item("volume", self.volume.to_string())
            .expect("Couldn't set volume.");
        dict.set_item("candle", inner_dict)
            .expect("Couldn't set candle.");
        dict.into()
    }
}

impl IntoPy<PyObject> for Candle {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for MarketPair {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("quote", self.quote.clone())
            .expect("Couldn't set quote.");
        inner_dict
            .set_item("quote_decimal", self.quote_increment.to_string())
            .expect("Couldn't set quote_decimal.");
        inner_dict
            .set_item("base", self.base.clone())
            .expect("Couldn't set base.");
        inner_dict
            .set_item("base_increment", self.base_increment.to_string())
            .expect("Couldn't set base_increment.");
        inner_dict
            .set_item("symbol", self.symbol.clone())
            .expect("Couldn't set symbol.");
        dict.set_item("market_pair", inner_dict)
            .expect("Couldn't set market_pair.");
        dict.into()
    }
}

impl IntoPy<PyObject> for MarketPair {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for Balance {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("asset", self.asset.clone())
            .expect("Couldn't set asset.");
        inner_dict
            .set_item("free", self.free.to_string())
            .expect("Couldn't set free.");
        inner_dict
            .set_item("total", self.total.to_string())
            .expect("Couldn't set total.");
        dict.set_item("balance", inner_dict)
            .expect("Couldn't set balance.");
        dict.into()
    }
}

impl IntoPy<PyObject> for Balance {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for OrderCanceled {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("order_canceled", self.id.clone())
            .expect("Couldn't set order_canceled.");
        dict.into()
    }
}

impl IntoPy<PyObject> for OrderCanceled {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for Order {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("id", self.id.clone())
            .expect("Couldn't set id.");
        inner_dict
            .set_item("market_pair", self.market_pair.clone())
            .expect("Couldn't set market_pair.");
        inner_dict
            .set_item("price", self.price.map(|x| x.to_string()))
            .expect("Couldn't set price.");
        inner_dict
            .set_item("order_type", self.order_type.clone())
            .expect("Couldn't set order_type.");
        inner_dict
            .set_item(
                "client_order_id",
                self.client_order_id.clone().map(|x| x.to_string()),
            )
            .expect("Couldn't set client_order_id.");
        inner_dict
            .set_item("created_at", self.created_at.clone())
            .expect("Couldn't set created_at.");
        inner_dict
            .set_item("side", self.side.clone())
            .expect("Couldn't set side.");
        inner_dict
            .set_item("size", self.size.to_string())
            .expect("Couldn't set size.");
        inner_dict
            .set_item("status", self.status.clone())
            .expect("Couldn't set status.");
        dict.set_item("order", inner_dict)
            .expect("Couldn't set order.");
        dict.into()
    }
}

impl IntoPy<PyObject> for Order {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for OrderBookResponse {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("asks", self.asks.clone())
            .expect("Couldn't set asks.");
        inner_dict
            .set_item("bids", self.bids.clone())
            .expect("Couldn't set bids.");
        dict.set_item("orderbook", inner_dict)
            .expect("Couldn't set orderbook.");
        dict.into()
    }
}

impl IntoPy<PyObject> for OrderBookResponse {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for AskBid {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("price", self.price.to_string())
            .expect("Couldn't set price.");
        inner_dict
            .set_item("qty", self.qty.to_string())
            .expect("Couldn't set qty.");
        dict.set_item("ask_or_bid", inner_dict)
            .expect("Couldn't set ask_or_bid.");
        dict.into()
    }
}

impl ToPyObject for Trade {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("liquidity", self.liquidity.clone())
            .expect("Couldn't set liquidity.");
        inner_dict
            .set_item("market_pair", self.market_pair.clone())
            .expect("Couldn't set maret_pair.");
        inner_dict
            .set_item("price", self.price.to_string())
            .expect("Couldn't set price.");
        inner_dict
            .set_item("qty", self.qty.to_string())
            .expect("Couldn't set qty.");
        inner_dict
            .set_item(
                "buyer_order_id",
                self.buyer_order_id.clone().map(|id| id.to_string()),
            )
            .expect("Couldn't set buyer_order_id.");
        inner_dict
            .set_item(
                "seller_order_id",
                self.seller_order_id.clone().map(|id| id.to_string()),
            )
            .expect("Couldn't set seller_order_id.");
        inner_dict
            .set_item("side", self.side.clone())
            .expect("Couldn't set side.");
        inner_dict
            .set_item("created_at", self.created_at)
            .expect("Couldn't set created_at.");
        inner_dict
            .set_item("fees", self.fees.map(|fee| fee.to_string()))
            .expect("Couldn't set fees.");
        inner_dict
            .set_item("id", self.id.to_string())
            .expect("Couldn't set id.");
        dict.set_item("trade", inner_dict)
            .expect("Couldn't set trade.");
        dict.into()
    }
}

impl IntoPy<PyObject> for Trade {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for OrderStatus {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let to_string = match self {
            Self::New => "new",
            Self::PartiallyFilled => "partially_filled",
            Self::Canceled => "canceled",
            Self::Filled => "filled",
            Self::PendingCancel => "pending_cancel",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Open => "open",
            Self::Pending => "pending",
            Self::Active => "active",
        };
        dict.set_item("order_status", to_string)
            .expect("Couldn't set order_status.");
        dict.into()
    }
}

impl ToPyObject for Side {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let to_string = match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        };
        dict.set_item("side", to_string)
            .expect("Couldn't set side.");
        dict.into()
    }
}

impl ToPyObject for Liquidity {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let to_string = match self {
            Self::Maker => "maker",
            Self::Taker => "taker",
        };
        dict.set_item("liquidity", to_string)
            .expect("Couldn't set liquidity.");
        dict.into()
    }
}
