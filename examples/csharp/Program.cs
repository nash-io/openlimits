using System;

namespace Example
{
    using Openlimits;
    using System.Threading;
    class Program
    {
        static public void Main(string[] args)
        {
//            Console.WriteLine("Init");
//            Test.Hello();
//            var test = new Test(64, Environment.Sandbox);
//            Console.WriteLine(test.value);
//            Console.WriteLine(test.environment);
//            var test2 = Test.Create(32);
//            Console.WriteLine(test2.value);
//            Console.WriteLine(test2.environment);
            CoinbaseParameters parameters = new CoinbaseParameters(Environment.Production, "a", "b", "c");
            Client client = Client.Coinbase(parameters);
            Client.OrderBook(client, "BTC-EUR");
//            var person = new Person("Danilo", "Guanabara");
//            Test.Display(person);
//            NashClientConfig config = NashClientConfig.Unauthenticated(0, NashEnvironment.Production, 1000);
//            Console.WriteLine(config.environment);
//            var client = new ExchangeClient(config);
//
//            client.SubscribeToDisconnect(() => {
//                Console.WriteLine("Disconnected");
//            });
//            foreach(var market in client.R qeceivePairs()) {
//                client.SubscribeToOrderbook(market.symbol, PrintBook);
//            }
//
//            GC.Collect();
//            GC.WaitForPendingFinalizers();

            // Noia markets only available in NashEnvironment.Production
            // Console.WriteLine("Listening to the noia markets");
            // client.SubscribeToOrderbook("noia_usdc", PrintBook);
            // client.SubscribeToOrderbook("noia_btc", PrintBook);
        }
    }
}