use super::websocket::{OpenLimitsWebsocketMessage, Subscription};

use pyo3::exceptions::PyException;
use pyo3::prelude::{FromPyObject, IntoPy, PyObject, Python, ToPyObject};
use pyo3::types::PyDict;

impl<'a> FromPyObject<'a> for Subscription {
    fn extract(pyobj: &'a pyo3::PyAny) -> Result<Self, pyo3::PyErr> {
        // we will simulate an enum in Python via dictionary keys
        if let Ok(trade) = pyobj.get_item("trade") {
            Ok(Subscription::Trade(trade.extract()?))
        } else if let Ok(orderbook) = pyobj.get_item("orderbook") {
            let orderbook_args: (String, i64) = orderbook.extract()?;
            Ok(Subscription::OrderBook(orderbook_args.0, orderbook_args.1))
        } else {
            Err(PyException::new_err("Not a valid input subscription"))
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

// Responses

use super::{
    AskBid, Balance, Liquidity, Order, OrderBookResponse, OrderCanceled, OrderStatus, Side, Trade,
};

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

impl<T> ToPyObject for OrderCanceled<T>
where
    T: ToString,
{
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("order_canceled", self.id.to_string())
            .unwrap();
        dict.into()
    }
}

impl<T> IntoPy<PyObject> for OrderCanceled<T>
where
    T: ToString,
{
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl<T> ToPyObject for Order<T>
where
    T: ToString,
{
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        let inner_dict = PyDict::new(py);
        inner_dict.set_item("id", self.id.to_string()).unwrap();
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

impl<T> IntoPy<PyObject> for Order<T>
where
    T: ToString,
{
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

impl ToPyObject for Trade<String, String> {
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
            .set_item("order_id", self.order_id.clone())
            .unwrap();
        inner_dict.set_item("side", self.side.clone()).unwrap();
        inner_dict.set_item("created_at", self.created_at).unwrap();
        inner_dict
            .set_item("fees", self.fees.map(|fee| fee.to_string()))
            .unwrap();
        inner_dict.set_item("id", self.id.clone()).unwrap();
        dict.set_item("trade", inner_dict).unwrap();
        dict.into()
    }
}

// New,
// PartiallyFilled,
// Filled,
// Canceled,
// PendingCancel,
// Rejected,
// Expired,
// Open,
// Pending,
// Active,

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
        dict.set_item("status", to_string).unwrap();
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
