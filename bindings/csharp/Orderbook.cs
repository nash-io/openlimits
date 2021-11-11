namespace OpenLimits {
    using System.Collections.Generic;
    public class Orderbook {
        private readonly Dictionary<decimal, decimal> _bids = new Dictionary<decimal, decimal>();
        private readonly Dictionary<decimal, decimal> _asks = new Dictionary<decimal, decimal>();
        
        public void Update(OrderbookResponse changes) {
            foreach(var ask in changes.asks) {
                if (ask.qty == 0){
                    if (_asks.ContainsKey(ask.price)) {
                        _asks.Remove(ask.price);
                    }
                } else {
                    _asks.Add(ask.price, ask.qty);
                }
            }

            foreach(var bid in changes.bids) {
                if (bid.qty == 0){
                    if (_bids.ContainsKey(bid.price)) {
                        _bids.Remove(bid.price);
                    }
                } else {
                    _bids.Add(bid.price, bid.qty);
                }
            }
        }
    }
}