using System.Diagnostics;
using System.Runtime.InteropServices;
namespace OpenLimits
{
    public enum NashEnvironment {
        Sandbox,
        Production
    }
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public struct NashClientConfig
    {
        public readonly string apikey;
        public readonly string secret;
        public readonly string affiliateCode;

        public readonly ulong clientId;
        public readonly NashEnvironment environment;
        public readonly ulong timeout;

        private NashClientConfig(string apikey, string secret, ulong clientId, NashEnvironment environment, ulong timeout, string affiliateCode)
        {
            this.apikey = apikey;
            this.secret = secret;
            this.clientId = clientId;
            this.environment = environment;
            this.timeout = timeout;
            this.affiliateCode = affiliateCode;
        }

        static public NashClientConfig Authenticated(string apikey, string secret, ulong clientId, NashEnvironment environment, ulong timeout) {
            return new NashClientConfig(apikey, secret, clientId, environment, timeout, null);
        }

        static public NashClientConfig Authenticated(string apikey, string secret, string affiliateCode, ulong clientId, NashEnvironment environment, ulong timeout) {
            return new NashClientConfig(apikey, secret, clientId, environment, timeout, affiliateCode);
        }

        static public NashClientConfig Unauthenticated(ulong clientId, NashEnvironment environment, ulong timeout) {
            return new NashClientConfig(null, null, clientId, environment, timeout, null);
        }
    }
}