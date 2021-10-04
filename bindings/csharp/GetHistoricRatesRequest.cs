namespace OpenLimits
{
    using System.Runtime.InteropServices;

    public enum Interval {
        OneMinute,
        ThreeMinutes,
        FiveMinutes,
        FifteenMinutes,
        ThirtyMinutes,
        OneHour,
        TwoHours,
        FourHours,
        SixHours,
        EightHours,
        TwelveHours,
        OneDay,
        ThreeDays,
        OneWeek,
        OneMonth,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GetHistoricRatesRequest
    {
        public readonly string market;
        public readonly Interval interval;
        public readonly Paginator paginator;

        public GetHistoricRatesRequest(string market, Interval interval, Paginator paginator)
        {
            this.market = market;
            this.interval = interval;
            this.paginator = paginator;
        }

        public GetHistoricRatesRequest(string market, Interval interval)
        {
            this.market = market;
            this.interval = interval;
            this.paginator = null;
        }
    }
}