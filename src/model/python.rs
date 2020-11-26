use super::super::any_exchange::InitAnyExchange;
use super::super::binance::{BinanceCredentials, BinanceParameters};
use super::super::model::{Interval, Paginator, TimeInForce};
use super::super::nash::{Environment, NashCredentials, NashParameters};
use super::websocket::{OpenLimitsWebSocketMessage, Subscription};
use pyo3::exceptions::PyException;
use pyo3::prelude::{FromPyObject, IntoPy, PyObject, PyResult, Python, ToPyObject};
use pyo3::types::PyDict;

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
        let maybe_nash: PyResult<NashParameters> = ob.extract();
        if let Ok(nash) = maybe_nash {
            return Ok(InitAnyExchange::Nash(nash));
        }
        let maybe_binance: PyResult<BinanceParameters> = ob.extract();
        if let Ok(binance) = maybe_binance {
            return Ok(InitAnyExchange::Binance(binance));
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
        Ok(NashParameters {
            affiliate_code,
            credentials,
            client_id,
            environment,
            timeout,
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
                dict.set_item("ping", PyDict::new(py)).unwrap();
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
        dict.set_item("order_type", inner_str).unwrap();
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
            .unwrap();
        dict.set_item("ticker", inner_dict).unwrap();
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
        inner_dict.set_item("low", self.low.to_string()).unwrap();
        inner_dict.set_item("high", self.high.to_string()).unwrap();
        inner_dict
            .set_item("close", self.close.to_string())
            .unwrap();
        inner_dict.set_item("open", self.open.to_string()).unwrap();
        inner_dict.set_item("time", self.time).unwrap();
        inner_dict
            .set_item("volume", self.volume.to_string())
            .unwrap();
        dict.set_item("candle", inner_dict).unwrap();
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
        inner_dict.set_item("quote", self.quote.clone()).unwrap();
        inner_dict
            .set_item("quote_decimal", self.quote_increment.to_string())
            .unwrap();
        inner_dict.set_item("base", self.base.clone()).unwrap();
        inner_dict
            .set_item("base_increment", self.base_increment.to_string())
            .unwrap();
        inner_dict.set_item("symbol", self.symbol.clone()).unwrap();
        dict.set_item("market_pair", inner_dict).unwrap();
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
        inner_dict.set_item("asset", self.asset.clone()).unwrap();
        inner_dict.set_item("free", self.free.to_string()).unwrap();
        inner_dict
            .set_item("total", self.total.to_string())
            .unwrap();
        dict.set_item("balance", inner_dict).unwrap();
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
        dict.set_item("order_canceled", self.id.clone()).unwrap();
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
        inner_dict.set_item("id", self.id.clone()).unwrap();
        inner_dict
            .set_item("market_pair", self.market_pair.clone())
            .unwrap();
        inner_dict
            .set_item("price", self.price.map(|x| x.to_string()))
            .unwrap();
        inner_dict
            .set_item("order_type", self.order_type.clone())
            .unwrap();
        inner_dict
            .set_item(
                "client_order_id",
                self.client_order_id.clone().map(|x| x.to_string()),
            )
            .unwrap();
        inner_dict
            .set_item("created_at", self.created_at.clone())
            .unwrap();
        inner_dict.set_item("side", self.side.clone()).unwrap();
        inner_dict.set_item("size", self.size.to_string()).unwrap();
        inner_dict.set_item("status", self.status.clone()).unwrap();
        dict.set_item("order", inner_dict).unwrap();
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
        inner_dict.set_item("asks", self.asks.clone()).unwrap();
        inner_dict.set_item("bids", self.bids.clone()).unwrap();
        dict.set_item("orderbook", inner_dict).unwrap();
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
            .unwrap();
        inner_dict.set_item("qty", self.qty.to_string()).unwrap();
        dict.set_item("ask_or_bid", inner_dict).unwrap();
        dict.into()
    }
}

impl ToPyObject for Trade {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict
            .set_item("liquidity", self.liquidity.clone())
            .unwrap();
        inner_dict
            .set_item("market_pair", self.market_pair.clone())
            .unwrap();
        inner_dict
            .set_item("price", self.price.to_string())
            .unwrap();
        inner_dict.set_item("qty", self.qty.to_string()).unwrap();
        inner_dict
            .set_item("order_id", self.order_id.to_string())
            .unwrap();
        inner_dict.set_item("side", self.side.clone()).unwrap();
        inner_dict.set_item("created_at", self.created_at).unwrap();
        inner_dict
            .set_item("fees", self.fees.map(|fee| fee.to_string()))
            .unwrap();
        inner_dict.set_item("id", self.id.to_string()).unwrap();
        dict.set_item("trade", inner_dict).unwrap();
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
        dict.set_item("order_status", to_string).unwrap();
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
        dict.set_item("side", to_string).unwrap();
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
        dict.set_item("liquidity", to_string).unwrap();
        dict.into()
    }
}
