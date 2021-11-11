namespace OpenLimits
{
    using System.Runtime.InteropServices;
    using System;
    using System.Globalization;


    [StructLayout(LayoutKind.Sequential)]
    internal struct FFITrade {
        public readonly IntPtr id;
        public readonly IntPtr buyerOrderId;
        public readonly IntPtr sellerOrderId;
        public readonly IntPtr marketPair;

        public readonly IntPtr price;
        public readonly IntPtr qty;
        public readonly IntPtr fees;
        public readonly Side side;
        public readonly Liquidity liquidity;
        public readonly ulong createdAt;

        public void Dispose() {
            ExchangeClient.FreeString(id);
            ExchangeClient.FreeString(buyerOrderId);
            ExchangeClient.FreeString(sellerOrderId);
            ExchangeClient.FreeString(marketPair);
            ExchangeClient.FreeString(price);
            ExchangeClient.FreeString(qty);
            ExchangeClient.FreeString(fees);
        }

        public Trade ToTrade() {
            return new Trade(
                CString.ToString(this.id),
                CString.ToString(this.buyerOrderId),
                CString.ToString(this.sellerOrderId),
                CString.ToString(this.marketPair),
                CString.ToString(this.price),
                CString.ToString(this.qty),
                CString.ToString(this.fees),
                this.side,
                this.liquidity,
                this.createdAt
            );
        }
    }

    public struct Trade {
        public readonly string id;
        public readonly string buyerOrderId;
        public readonly string sellerOrderId;
        public readonly string marketPair;
        public readonly decimal price;
        public readonly decimal qty;
        public readonly decimal? fees;

        public readonly Side side;
        public readonly Liquidity liquidity;
        public readonly ulong createdAt;

        public Trade(string id, string buyerOrderId, string sellerOrderId, string marketPair, string price, string qty, string fees, Side side, Liquidity liquidity, ulong createdAt)
        {
            this.id = id;
            this.buyerOrderId = buyerOrderId;
            this.sellerOrderId = sellerOrderId;
            this.marketPair = marketPair;
            this.price = decimal.Parse(price, System.Globalization.NumberStyles.AllowDecimalPoint, CultureInfo.InvariantCulture);
            this.qty = decimal.Parse(qty, System.Globalization.NumberStyles.AllowDecimalPoint, CultureInfo.InvariantCulture);
            this.fees = fees == null ?  default(decimal?): decimal.Parse(fees, System.Globalization.NumberStyles.AllowDecimalPoint, CultureInfo.InvariantCulture);
            this.side = side;
            this.liquidity = liquidity;
            this.createdAt = createdAt;
        }

        public override string ToString()
        {
            return "Trade{" +
                "id='" + id + '\'' +
                ", buyer_order_id='" + buyerOrderId + '\'' +
                ", seller_order_id='" + sellerOrderId + '\'' +
                ", market_pair='" + marketPair + '\'' +
                ", price=" + price +
                ", qty=" + qty +
                ", fees=" + fees +
                ", side='" + side + '\'' +
                ", liquidity='" + liquidity + '\'' +
                ", createdAt=" + createdAt +
                '}';
        }
    }
}