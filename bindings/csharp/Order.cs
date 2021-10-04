namespace OpenLimits
{
    using System;
    using System.Globalization;

    internal struct FFIOrder
    {
        public readonly IntPtr id;
        public readonly IntPtr marketPair;
        public readonly IntPtr clientOrderId;
        public readonly ulong createdAt;
        public readonly OrderType orderType;
        public readonly Side side;
        public readonly OrderStatus status;
        public readonly IntPtr size;
        public readonly IntPtr price;
        public readonly IntPtr remaining;

        public void Dispose() {
            ExchangeClient.FreeString(id);
            ExchangeClient.FreeString(marketPair);
            ExchangeClient.FreeString(size);
            ExchangeClient.FreeString(price);
            ExchangeClient.FreeString(remaining);
        }

        public Order ToOrder() {
            return new Order(
                CString.ToString(this.id),
                CString.ToString(this.marketPair),
                CString.ToString(this.clientOrderId),
                this.createdAt,
                this.orderType,
                this.side,
                this.status,
                CString.ToString(this.size),
                CString.ToString(this.price),
                CString.ToString(this.remaining)
            );
        }
    }

    public struct Order
    {
        public readonly string id;
        public readonly string marketPair;
        public readonly string clientOrderId;
        public readonly ulong createdAt;
        public readonly OrderType orderType;
        public readonly Side side;
        public readonly OrderStatus status;
        public readonly decimal size;
        public readonly decimal? price;
        public readonly decimal? remaining;

        public Order(string id, string marketPair, string clientOrderId, ulong createdAt, OrderType orderType, Side side, OrderStatus status, string size, string price, string remaining)
        {
            this.id = id;
            this.marketPair = marketPair;
            this.clientOrderId = clientOrderId;
            this.createdAt = createdAt;
            this.orderType = orderType;
            this.side = side;
            this.status = status;
            
            this.size = decimal.Parse(size, System.Globalization.NumberStyles.AllowDecimalPoint, CultureInfo.InvariantCulture);
            this.price = price == null ? default(decimal?) : decimal.Parse(price, System.Globalization.NumberStyles.AllowDecimalPoint, CultureInfo.InvariantCulture);
            this.remaining = remaining == null ? default(decimal?) : decimal.Parse(remaining, System.Globalization.NumberStyles.AllowDecimalPoint, CultureInfo.InvariantCulture);
        }

        public override bool Equals(object obj)
        {
            return base.Equals(obj);
        }

        public override int GetHashCode()
        {
            return base.GetHashCode();
        }

        public override string ToString()
        {
            return "Order{" +
                "id='" + id + '\'' +
                ", market='" + marketPair + '\'' +
                ", clientOrderId='" + clientOrderId + '\'' +
                ", createdAt=" + createdAt +
                ", orderType='" + orderType + '\'' +
                ", side='" + side + '\'' +
                ", status='" + status + '\'' +
                ", size='" + size + '\'' +
                ", price='" + price + '\'' +
                ", remaining='" + remaining + '\'' +
                '}';
        }
    }
}