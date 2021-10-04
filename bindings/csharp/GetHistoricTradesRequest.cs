namespace OpenLimits
{
    using System.Runtime.InteropServices;
    [StructLayout(LayoutKind.Sequential, Pack=1)]
    public struct GetHistoricTradesRequest
    {
        public readonly string market;
        public readonly Paginator paginator;

        public GetHistoricTradesRequest(string market, Paginator paginator)
        {
            this.market = market;
            this.paginator = paginator;
        }

        public GetHistoricTradesRequest(string market)
        {
            this.market = market;
            this.paginator = null;
        }
    }
}