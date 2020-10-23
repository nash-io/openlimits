use super::websocket::{Subscription, OpenLimitsWebsocketMessage};
use super::{OrderBookResponse, Trade, AskBid, Side, Liquidity};

use pyo3::prelude::{FromPyObject, PyObject, ToPyObject, Python};
use pyo3::types::PyDict;
use pyo3::exceptions::PyException;

impl<'a> FromPyObject<'a> for Subscription {
    fn extract(pyobj: &'a pyo3::PyAny) -> Result<Self, pyo3::PyErr> {
        // we will simulate an enum in Python via dictionary keys
        if let Ok(trade) = pyobj.get_item("trade") {
            Ok(Subscription::Trade(trade.extract()?))
        }
        else if let Ok(orderbook) = pyobj.get_item("orderbook") {
            let orderbook_args: (String, i64) = orderbook.extract()?;
            Ok(Subscription::OrderBook(orderbook_args.0, orderbook_args.1))
        } else {
            Err(PyException::new_err("Not a valid input subscription"))
        }
     }
}

impl ToPyObject for OpenLimitsWebsocketMessage {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        match self {
            Self::Ping => {
                // empty dict to represent null
                dict.set_item("ping", PyDict::new(py)).unwrap();
            },
            Self::OrderBook(resp) => {
                dict.set_item("orderbook", resp).unwrap();
            },
            Self::Trades(resp) => {
                dict.set_item("trades", resp).unwrap();
            }
        }
        dict.into()
    }
}

// Responses 

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

impl ToPyObject for AskBid {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict.set_item("price", self.price.to_string()).unwrap();
        inner_dict.set_item("qty", self.qty.to_string()).unwrap();
        dict.set_item("ask_or_bid", inner_dict).unwrap();
        dict.into()
    }
}

impl ToPyObject for Trade<String, String> {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict.set_item("liquidity", self.liquidity.clone()).unwrap();
        inner_dict.set_item("market_pair", self.market_pair.clone()).unwrap();
        inner_dict.set_item("price", self.price.to_string()).unwrap();
        inner_dict.set_item("qty", self.qty.to_string()).unwrap();
        inner_dict.set_item("order_id", self.order_id.clone()).unwrap();
        inner_dict.set_item("side", self.side.clone()).unwrap();
        inner_dict.set_item("created_at", self.created_at).unwrap();
        inner_dict.set_item("fees", self.fees.map(|fee| fee.to_string())).unwrap();
        inner_dict.set_item("id", self.id.clone()).unwrap();
        dict.set_item("trade", inner_dict).unwrap();
        dict.into()
    }
}

impl ToPyObject for Side {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let to_string = match self {
            Self::Buy => {
                "buy"
            },
            Self::Sell => {
                "sell"
            }
        };
        dict.set_item("side", to_string).unwrap();
        dict.into()
    }
}

impl ToPyObject for Liquidity {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let to_string = match self {
            Self::Maker => {
                "maker"
            },
            Self::Taker => {
                "taker"
            }
        };
        dict.set_item("liquidity", to_string).unwrap();
        dict.into()
    }
}