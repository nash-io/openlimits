namespace OpenLimits
{
    using System.Runtime.InteropServices;
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public struct Candle
    {
        public readonly ulong time;
        public readonly double low;
        public readonly double high;
        public readonly double open;
        public readonly double close;
        public readonly double volume;

        public Candle(ulong time, double low, double high, double open, double close, double volume)
        {
            this.time = time;
            this.low = low;
            this.high = high;
            this.open = open;
            this.close = close;
            this.volume = volume;
        }
    }
}