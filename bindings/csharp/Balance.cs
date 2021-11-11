namespace OpenLimits
{
    using System.Runtime.InteropServices;
    using System;

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public struct FFIBalance
    {
        public readonly IntPtr asset;
        public readonly IntPtr total;
        public readonly IntPtr free;

        public void Dispose() {
            ExchangeClient.FreeString(asset);
            ExchangeClient.FreeString(total);
            ExchangeClient.FreeString(free);
        }

        public Balance ToBalance() {
            return new Balance(
                CString.ToString(this.asset),
                CString.ToString(this.total),
                CString.ToString(this.free)
            );
        }
    }

    public struct Balance
    {
        public readonly string asset;
        public readonly string total;
        public readonly string free;

        public Balance(string asset, string total, string free)
        {
            this.asset = asset;
            this.total = total;
            this.free = free;
        }
    }
}