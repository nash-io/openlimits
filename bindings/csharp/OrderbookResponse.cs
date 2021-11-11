namespace OpenLimits
{
    using System.Collections.Generic;

    public class OrderbookResponse
    {
        readonly public string market;
        readonly public IEnumerable<AskBid> asks;
        readonly public IEnumerable<AskBid> bids;

        readonly public ulong lastUpdateId;
        readonly public ulong updateId;

        public OrderbookResponse(string market, IEnumerable<AskBid> asks, IEnumerable<AskBid> bids, ulong lastUpdateId, ulong updateId)
        {
            this.market = market;
            this.asks = asks;
            this.bids = bids;
            this.lastUpdateId = lastUpdateId;
            this.updateId = updateId;
        }
    }
}