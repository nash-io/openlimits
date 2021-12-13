using System;

namespace Example
{
    using Openlimits;
    using System.Threading;
	using System.Collections.Generic;
    using System.Runtime.InteropServices;
    class Program
    {
        static public void Main(string[] args)
        {
            Console.WriteLine("String: " + Client.GetName());
            Console.WriteLine("Appended string: " + Client.AppendString("estringue"));
//            CoinbaseParameters parameters = new CoinbaseParameters(Environment.Production, "a", "b", "c");
//            Client client = Client.Coinbase(parameters);
//            Console.WriteLine(Decimal.Parse(askBid.qty));
//            Console.WriteLine(Decimal.Parse(askBid.qty));
//            Console.WriteLine(askBid.price);
//            var list = new List<ulong>();
//            list.Add(1);
//            list.Add(2);
//            list.Add(3);
//            Console.WriteLine(client.Sum(list));
//            var result = client.Mul(list, 2);
//            foreach (var value in result) {
//                Console.WriteLine(value);
//            }

//            Test.Display(person);
//            NashClientConfig config = NashClientConfig.Unauthenticated(0, NashEnvironment.Production, 1000);
//            Console.WriteLine(config.environment);
//            var client = new ExchangeClient(config);
//
//            client.SubscribeToDisconnect(() => {
//                Console.WriteLine("Disconnected");
//            });
//            foreach(var market in client.ReceivePairs()) {
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