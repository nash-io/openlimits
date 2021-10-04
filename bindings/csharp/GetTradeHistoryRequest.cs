namespace OpenLimits
{
    public struct GetTradeHistoryRequest
    {
        public readonly string market;
        public readonly string orderId;
        public readonly Paginator paginator;

        public GetTradeHistoryRequest(string market, string orderId, Paginator paginator)
        {
            this.market = market;
            this.orderId = orderId;
            this.paginator = paginator;
        }

        public GetTradeHistoryRequest(string market, string orderId)
        {
            this.market = market;
            this.orderId = orderId;
            this.paginator = null;
        }
    }
}