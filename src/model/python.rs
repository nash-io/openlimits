use super::super::model::{Interval, Paginator};
use super::websocket::{OpenLimitsWebsocketMessage, Subscription};

use pyo3::exceptions::PyException;
use pyo3::prelude::{FromPyObject, IntoPy, PyObject, PyResult, Python, ToPyObject};
use pyo3::types::PyDict;

// Python to Rust...

impl<'a> FromPyObject<'a> for Interval {
    fn extract(ob: &'a pyo3::PyAny) -> PyResult<Self> {
        let interval_str: String = ob.get_item("interval")?.extract()?;
        match &interval_str[..] {
            "one_minute" => Ok(Interval::OneMinute),
            "three_minutes" => Ok(Interval::ThreeMinutes),
            "five_minutes" => Ok(Interval::FiveMinutes),
            "fifteen_minutes" => Ok(Interval::FifteenMinutes),
            "thirty_minutes" => Ok(Interval::ThirtyMinutes),
            "one_hour" => Ok(Interval::OneHour),
            "two_hours" => Ok(Interval::TwoHours),
            "four_hours" => Ok(Interval::FourHours),
            "six_hours" => Ok(Interval::SixHours),
            "eight_hours" => Ok(Interval::EightHours),
            "twelve_hours" => Ok(Interval::TwelveHours),
            "one_day" => Ok(Interval::OneDay),
            "three_days" => Ok(Interval::ThreeDays),
            "one_week" => Ok(Interval::OneWeek),
            "one_month" => Ok(Interval::OneMonth),
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
            Ok(Subscription::Trade(trade.extract()?))
        } else if let Ok(orderbook) = pyobj.get_item("orderbook") {
            let orderbook_args: (String, i64) = orderbook.extract()?;
            Ok(Subscription::OrderBook(orderbook_args.0, orderbook_args.1))
        } else {
            Err(PyException::new_err("Not a supported input subscription"))
        }
    }
}

impl ToPyObject for OpenLimitsWebsocketMessage {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Self::Ping => {
                // empty dict to represent null
                let dict = PyDict::new(py);
                dict.set_item("ping", PyDict::new(py)).unwrap();
                dict.into()
            }
            Self::OrderBook(resp) => resp.to_object(py),
            Self::Trades(resp) => resp.to_object(py),
        }
    }
}

impl IntoPy<PyObject> for OpenLimitsWebsocketMessage {
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
            .set_item("price", self.price.to_string())
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
