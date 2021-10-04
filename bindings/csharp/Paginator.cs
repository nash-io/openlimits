namespace OpenLimits
{
    using System.Runtime.InteropServices;
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public class Paginator
    {
        public readonly ulong startTime;
        public readonly ulong endTime;
        public readonly ulong limit;
        public readonly string before;
        public readonly string after;

        public Paginator(ulong startTime, ulong endTime, ulong limit, string before, string after)
        {
            this.startTime = startTime;
            this.endTime = endTime;
            this.limit = limit;
            this.before = before;
            this.after = after;
        }
    }
}