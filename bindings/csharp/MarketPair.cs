namespace OpenLimits
{
    using System.Runtime.InteropServices;
    using System;

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public struct FFIMarketPair
    {
        public readonly IntPtr baseSymbol;
        public readonly IntPtr quote;
        public readonly IntPtr symbol;
        public readonly IntPtr baseIncrement;
        public readonly IntPtr quoteIncrement;
        public readonly IntPtr baseMinPrice;
        public readonly IntPtr quoteMinPrice;

        public void Dispose() {
            ExchangeClient.FreeString(baseSymbol);
            ExchangeClient.FreeString(quote);
            ExchangeClient.FreeString(symbol);
            ExchangeClient.FreeString(baseIncrement);
            ExchangeClient.FreeString(quoteIncrement);
            ExchangeClient.FreeString(baseMinPrice);
            ExchangeClient.FreeString(quoteMinPrice);
        }

        public MarketPair ToMarketPair() {
            return new MarketPair(
                CString.ToString(this.baseSymbol),
                CString.ToString(this.quote),
                CString.ToString(this.symbol),
                CString.ToString(this.baseIncrement),
                CString.ToString(this.quoteIncrement),
                CString.ToString(this.baseMinPrice),
                CString.ToString(this.quoteMinPrice)
            );
        }
    }

    public struct MarketPair
    {
        public readonly string baseSymbol;
        public readonly string quote;
        public readonly string symbol;
        public readonly string baseIncrement;
        public readonly string quoteIncrement;
        public readonly string baseMinPrice;
        public readonly string quoteMinPrice;

        public MarketPair(string baseSymbol, string quote, string symbol, string baseIncrement, string quoteIncrement, string baseMinPrice, string quoteMinPrice)
        {
            this.baseSymbol = baseSymbol;
            this.quote = quote;
            this.symbol = symbol;
            this.baseIncrement = baseIncrement;
            this.quoteIncrement = quoteIncrement;
            this.baseMinPrice = baseMinPrice;
            this.quoteMinPrice = quoteMinPrice;
        }
    }
}