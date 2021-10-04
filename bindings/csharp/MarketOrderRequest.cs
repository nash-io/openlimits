namespace OpenLimits
{
    public struct MarketOrderRequest
    {
        public readonly string size;
        public readonly string market;

        public MarketOrderRequest(string size, string market)
        {
            this.size = size;
            this.market = market;
        }
    }
}