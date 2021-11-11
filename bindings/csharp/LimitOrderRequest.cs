namespace OpenLimits
{
    public struct LimitOrderRequest
    {
        public readonly string price;
        public readonly string size;
        public readonly string market;
        public readonly TimeInForce timeInForce;
        public readonly ulong timeInForceDurationMs;
        public readonly bool postOnly;

        public LimitOrderRequest(string price, string size, string market, TimeInForce timeInForce, ulong timeInForceDurationMs, bool postOnly)
        {
            this.price = price;
            this.size = size;
            this.market = market;
            this.timeInForce = timeInForce;
            this.timeInForceDurationMs = timeInForceDurationMs;
            this.postOnly = postOnly;
        }

        public static LimitOrderRequest immediateOrCancel(string price, string size, string market) {
            return new LimitOrderRequest(price, size, market, TimeInForce.IOC, 0, false);
        }
        public static LimitOrderRequest goodTillCancelled(string price, string size, string market) {
            return new LimitOrderRequest(price, size, market, TimeInForce.GTC, 0, false);
        }
        public static LimitOrderRequest fillOrKill(string price, string size, string market) {
            return new LimitOrderRequest(price, size, market, TimeInForce.FOK, 0, false);
        }
        public static LimitOrderRequest goodTillTIme(string price, string size, string market, ulong timeInForceDurationMs) {
            return new LimitOrderRequest(price, size, market, TimeInForce.GTT, timeInForceDurationMs, false);
        }

        public static LimitOrderRequest immediateOrCancelPostOnly(string price, string size, string market) {
            return new LimitOrderRequest(price, size, market, TimeInForce.IOC, 0, true);
        }
        public static LimitOrderRequest goodTillCancelledPostOnly(string price, string size, string market) {
            return new LimitOrderRequest(price, size, market, TimeInForce.GTC, 0, true);
        }
        public static LimitOrderRequest fillOrKillPostOnly(string price, string size, string market) {
            return new LimitOrderRequest(price, size, market, TimeInForce.FOK, 0, true);
        }
        public static LimitOrderRequest goodTillTImePostOnly(string price, string size, string market, ulong timeInForceDurationMs) {
            return new LimitOrderRequest(price, size, market, TimeInForce.GTT, timeInForceDurationMs, true);
        }
    }
}