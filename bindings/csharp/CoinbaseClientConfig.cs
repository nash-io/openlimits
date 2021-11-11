
namespace OpenLimits
{
    using System.Runtime.InteropServices;
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public struct CoinbaseClientConfig
    {
        public readonly string apikey;
        public readonly string secret;
        public readonly string passphrase;
        public readonly bool sandbox;

        private CoinbaseClientConfig(string apikey, string secret, string passphrase, bool sandbox)
        {
            this.apikey = apikey;
            this.secret = secret;
            this.passphrase = passphrase;
            this.sandbox = sandbox;
        }

        public static CoinbaseClientConfig Authenticated(string apikey, string secret, string passphrase, bool sandbox) {
            return new CoinbaseClientConfig(apikey, secret, passphrase, sandbox);
        }

        public static CoinbaseClientConfig Unauthenticated(bool sandbox) {
            return new CoinbaseClientConfig(null, null, null, sandbox);
        }
    }
}