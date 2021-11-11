namespace OpenLimits
{
    using System.Collections.Generic;

    public class TradesResponse
    {
        readonly public string market;
        readonly public IEnumerable<Trade> trades;

        public TradesResponse(string market, IEnumerable<Trade> trades)
        {
            this.market = market;
            this.trades = trades;
        }
    }
}