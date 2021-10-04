using System;

namespace Example
{
    using OpenLimits;
    using System.Threading;
    class Program
    {
//        static public void PrintBook(OrderbookResponse orderbook) {
//            Console.WriteLine("New orderbook orders in " + orderbook.market);
//            Console.WriteLine("asks");
//            foreach(var ask in orderbook.asks) {
//                Console.WriteLine(ask);
//            }
//
//            Console.WriteLine("bids");
//            foreach(var bid in orderbook.bids) {
//                Console.WriteLine(bid);
//            }
//        }

        static public void Main(string[] args)
        {
            Console.WriteLine("Init");
            Test.Hello();
            Test.SaySomething();
            NashClientConfig config = NashClientConfig.Unauthenticated(0, NashEnvironment.Production, 1000);
            Console.WriteLine(config.environment);
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