namespace OpenLimits
{
    public struct GetOrderHistoryRequest
    {
        public readonly string market;
        public readonly Paginator paginator;

        public GetOrderHistoryRequest(string market, Paginator paginator)
        {
            this.market = market;
            this.paginator = paginator;
        }

        public GetOrderHistoryRequest(string market)
        {
            this.market = market;
            this.paginator = null;
        }
    }
}