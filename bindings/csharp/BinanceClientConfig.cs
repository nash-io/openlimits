
namespace OpenLimits
{
    using System.Runtime.InteropServices;
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public struct BinanceClientConfig
    {
        public readonly string apikey;
        public readonly string secret;
        public readonly bool sandbox;

        private BinanceClientConfig(string apikey, string secret, bool sandbox)
        {
            this.apikey = apikey;
            this.secret = secret;
            this.sandbox = sandbox;
        }

        public static BinanceClientConfig Authenticated(string apikey, string secret, bool sandbox) {
            return new BinanceClientConfig(apikey, secret, sandbox);
        }

        public static BinanceClientConfig Unauthenticated(bool sandbox) {
            return new BinanceClientConfig(null, null, sandbox);
        }
    }
}