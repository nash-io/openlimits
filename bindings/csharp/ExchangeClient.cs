
namespace OpenLimits
{
    
    using System;
    using System.Threading;
    using System.Collections.Generic;
    using System.Runtime.InteropServices;
    public class ExchangeClient
    {
        static HashSet<ExchangeClient> _clients = new HashSet<ExchangeClient>();
        private void handleResult(FFIResult result) {
            string message = "Unknown error";
            if (result.message.ToInt64() != 0) {
                message = CString.ToString(result.message);
                FreeString(result.message);
            }
            switch(result.tag) {
                case ResultTag.Ok: return;
                case ResultTag.InvalidArgument:
                    throw new ArgumentException(message);
                case ResultTag.BinanceError:
                    throw new BinanceError(message);
                case ResultTag.CoinbaseError:
                    throw new CoinbaseError(message);
                case ResultTag.NashProtocolError:
                    throw new NashProtocolError(message);
                case ResultTag.MissingImplementation:
                    throw new MissingImplementation(message);
                case ResultTag.AssetNotFound:
                    throw new AssetNotFound(message);
                case ResultTag.NoApiKeySet:
                    throw new NoApiKeySet(message);
                case ResultTag.InternalServerError:
                    throw new InternalServerError(message);
                case ResultTag.ServiceUnavailable:
                    throw new ServiceUnavailable(message);
                case ResultTag.Unauthorized:
                    throw new Unauthorized(message);
                case ResultTag.SymbolNotFound:
                    throw new SymbolNotFound(message);
                case ResultTag.SocketError:
                    throw new SocketError(message);
                case ResultTag.GetTimestampFailed:
                    throw new GetTimestampFailed(message);
                case ResultTag.ReqError:
                    throw new ReqError(message);
                case ResultTag.InvalidHeaderError:
                    throw new InvalidHeaderError(message);
                case ResultTag.InvalidPayloadSignature:
                    throw new InvalidPayloadSignature(message);
                case ResultTag.IoError:
                    throw new IoError(message);
                case ResultTag.PoisonError:
                    throw new PoisonError(message);
                case ResultTag.JsonError:
                    throw new JsonError(message);
                case ResultTag.ParseFloatError:
                    throw new ParseFloatError(message);
                case ResultTag.UrlParserError:
                    throw new UrlParserError(message);
                case ResultTag.Tungstenite:
                    throw new Tungstenite(message);
                case ResultTag.TimestampError:
                    throw new TimestampError(message);
                case ResultTag.UnkownResponse:
                    throw new UnkownResponse(message);
                case ResultTag.NotParsableResponse:
                    throw new NotParsableResponse(message);
                case ResultTag.MissingParameter:
                    throw new MissingParameter(message);     
                case ResultTag.WebSocketMessageNotSupported:
                    throw new WebSocketMessageNotSupported(message);            }
        }
        /// Used by rust to write data directly to C# thus avoiding changing ownership
        private FFITrade[] subTradesBuff = new FFITrade[1024];
        private FFIAskBid[] subAsksBuff = new FFIAskBid[1024];
        private FFIAskBid[] subBidsBuff = new FFIAskBid[1024];

        // Callbacks from rust into C#. Some callbacks come in a "private" and public version.
        // Some objects, especially those containing strings or array of objects will be serialized into a
        // C# version after arriving. Strings exchanged from rust to C# must be freed manually. So it is important not to expose
        // The internals
        public delegate void OnError();
        public delegate void OnPing();
        public delegate void OnDisconnect();
        public delegate void OnOrderbook(OrderbookResponse orderbook);
        unsafe private delegate void OnOrderbookFFI(ulong bidActualValueLen, ulong askActualValueLen, IntPtr market, ulong lastUpdateId, ulong updateId);
        public delegate void OnTrades(TradesResponse trades);
        private delegate void OnTradesFFI(ulong bidActualValueLen, IntPtr market);
        private OnError onErrorCb;
        private List<OnError> onErrorCbs = new List<OnError>();

        private OnPing onPingCb;
        private List<OnPing> onPingCbs = new List<OnPing>();
        private OnOrderbookFFI onOrderbookCb;
        private OnTradesFFI onTradesCb;

        private Dictionary<string, List<OnOrderbook>> onOrderbookCbs = new Dictionary<string, List<OnOrderbook>>();
        private Dictionary<string, List<OnTrades>> onTradesCbs = new Dictionary<string, List<OnTrades>>();
    
        private OnDisconnect onDisconnectCb;
        private List<OnDisconnect> onDisconnectCbs = new List<OnDisconnect>();


       
        const string NativeLib = "libopenlimits_sharp";

        unsafe private IntPtr _client_handle;
        unsafe private IntPtr _sub_handle;

        [DllImport(NativeLib, EntryPoint = "free_string", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern void FreeStringInternal(IntPtr handle);
        static public void FreeString(IntPtr handle) {
            if (handle.ToInt64() == 0) {
                return;
            }
            FreeStringInternal(handle);
        }
        
        
        [DllImport(NativeLib, EntryPoint = "disconnect", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe internal static extern void Disconnect(IntPtr subhandle);

        [DllImport(NativeLib, EntryPoint = "init_binance", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult InitBinance(BinanceClientConfig config, out IntPtr client);

        [DllImport(NativeLib, EntryPoint = "init_nash", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult InitNash(string apikey, string secret, ulong clientid, NashEnvironment environment, ulong timeout, string affiliateCode, out IntPtr client);
        
        
        [DllImport(NativeLib, EntryPoint = "init_coinbase", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult InitCoinbase(string apikey, string secret, string passphrase, bool sandbox, out IntPtr client);
        
        
        [DllImport(NativeLib, EntryPoint = "init_subscriptions", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult InitCbs(IntPtr client,
            OnError onError, OnPing onPing, OnOrderbookFFI onOrderbook, OnTradesFFI onTrades, OnDisconnect onDisconnect,
            IntPtr bidBuffPtr, UIntPtr bidBufLen,
            IntPtr askBuffPtr, UIntPtr askBufLen,
            IntPtr taskBuffPtr, UIntPtr tradeBufLen,
            out IntPtr subhandle
        );


        [DllImport(NativeLib, EntryPoint = "order_book", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult Orderbook(IntPtr client, string market,
            IntPtr bidBuffPtr, ulong bidBufLen, out ulong bidActualValueLen,
            IntPtr askBuffPtr, ulong AskBufLen, out ulong askActualValueLen,
            out ulong lastUpdateId,
            out ulong updateId
        );

        [DllImport(NativeLib, EntryPoint = "get_price_ticker", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetPriceTicker(IntPtr client, string market, out double price);

        [DllImport(NativeLib, EntryPoint = "get_historic_rates", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetHistoricRates(IntPtr client, string market, Interval interval, Paginator paginator,
            IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen
        );

        [DllImport(NativeLib, EntryPoint = "get_historic_trades", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetHistoricTrades(IntPtr client, string market, Paginator paginator,
            IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen
        );

        [DllImport(NativeLib, EntryPoint = "place_order", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult PlaceOrder(IntPtr client, string market,
            string qty,
            bool limit,
            string price,
            Side side,
            TimeInForce tif,
            ulong tifDuration,
            bool postOnly,
            out FFIOrder order
        );
        
        [DllImport(NativeLib, EntryPoint = "get_all_open_orders", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetAllOpenOrders(IntPtr client,
            IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen
        );

        [DllImport(NativeLib, EntryPoint = "subscribe_orderbook", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult SubscribeToOrderbook(IntPtr client, IntPtr subhandle, string market);

        [DllImport(NativeLib, EntryPoint = "subscribe_trades", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult SubscribeToTrades(IntPtr client, IntPtr subhandle, string market);

        [DllImport(NativeLib, EntryPoint = "get_order_history", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetOrderHistory(IntPtr client,
            string market, Paginator paginator,
            IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen
        );

        [DllImport(NativeLib, EntryPoint = "get_trade_history", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetTradeHistory(IntPtr client,
            string market, string orderId, Paginator paginator,
            IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen
        );

        [DllImport(NativeLib, EntryPoint = "get_account_balances", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetAccountBalances(IntPtr client,
            Paginator paginator,
            IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen
        );


        [DllImport(NativeLib, EntryPoint = "cancel_all_orders", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult CancelAllOrders(IntPtr client, string market, IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen);

        [DllImport(NativeLib, EntryPoint = "cancel_order", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult CancelOrder(IntPtr client,  string orderId, string market);

        
        [DllImport(NativeLib, EntryPoint = "get_order", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult GetOrder(IntPtr client,  string orderId, string market, out FFIOrder result);

        [DllImport(NativeLib, EntryPoint = "receive_pairs", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern FFIResult ReceivePairs(IntPtr client, IntPtr buffPtr, UIntPtr valueBufLen, out UIntPtr actualValueLen);

        private void handleFFIResult(FFIResult result) {
        }
        private void onPingHandler() {
            foreach(var callback in this.onPingCbs) {
                callback();
            }
        }
        private void onErrorHandler() {
            foreach(var callback in this.onErrorCbs) {
                callback();
            }
        }
        unsafe private void onTradesHandler(ulong tradeBuffLen, IntPtr marketStr) {
            var market = CString.ToString(marketStr);
            FreeString(marketStr);
            var tradesList = new List<Trade>();
            
            for (int i = 0 ; i < (int)tradeBuffLen ; i ++) {
                tradesList.Add(subTradesBuff[i].ToTrade());
                subTradesBuff[i].Dispose();
            }

            if (!onTradesCbs.ContainsKey(market)) {
                return;
            }
            var trades = new TradesResponse(market, tradesList);
            this.onTradesCbs.TryGetValue(market, out var callbacks);
            foreach(var callback in callbacks) {
                callback(trades);
            }
        }
        unsafe private void onOrderbookHandler(ulong bidActualValueLen, ulong askActualValueLen, IntPtr marketStr, ulong lastUpdateId, ulong updateId) {
            var market = CString.ToString(marketStr);
            FreeString(marketStr);
           
            var bidsList = new List<AskBid>();
            var asksList = new List<AskBid>();

            
            for (int i = 0 ; i < (int)bidActualValueLen ; i ++) {
                bidsList.Add(subBidsBuff[i].ToAskBid());
                subBidsBuff[i].Dispose();
            }
            
            for (int i = 0 ; i < (int)askActualValueLen ; i ++) {
                asksList.Add(subAsksBuff[i].ToAskBid());
                subAsksBuff[i].Dispose();
            }

            if (!onOrderbookCbs.ContainsKey(market)) {
                return;
            }
            var latestOrderbook = new OrderbookResponse(
                market,
                asksList,
                bidsList,
                lastUpdateId,
                updateId
            );

            this.onOrderbookCbs.TryGetValue(market, out var callbacks);
            foreach(var callback in callbacks) {
                callback(latestOrderbook);
            }
        }
        EventWaitHandle ewh = new EventWaitHandle(false, EventResetMode.ManualReset);
        Thread ewhThreadHandle = null;
        private void onDisconnect() {
            ewh.Set();
            _clients.Remove(this);
            
            foreach(var callback in this.onDisconnectCbs) {
                callback();
            }
        }

        unsafe private IntPtr InitCbs() {
            _clients.Add(this);
            fixed (FFIAskBid* bidBuff = subBidsBuff.AsSpan()) {
                fixed (FFIAskBid* askBuff = subAsksBuff.AsSpan()) {
                    fixed (FFITrade* tradeBuff = subTradesBuff.AsSpan()) {
                        this.onOrderbookCb = this.onOrderbookHandler;
                        this.onTradesCb = this.onTradesHandler;
                        this.onPingCb = this.onPingHandler;
                        this.onErrorCb = this.onErrorHandler;
                        this.onTradesCb = this.onTradesHandler;
                        this.onDisconnectCb = this.onDisconnect;
                        InitCbs(
                            _client_handle,
                            this.onErrorCb,
                            this.onPingCb,
                            this.onOrderbookCb,
                            this.onTradesCb,
                            this.onDisconnectCb,

                            (IntPtr)bidBuff, (UIntPtr)subBidsBuff.Length,
                            (IntPtr)askBuff, (UIntPtr)subAsksBuff.Length,
                            (IntPtr)tradeBuff, (UIntPtr)subTradesBuff.Length,
                            out var handle
                        );
                        return handle;
                    }
                }
            }
        }

        unsafe public ExchangeClient(BinanceClientConfig config) {
            handleResult(
                ExchangeClient.InitBinance(config, out var client_handle)
            );
            
            _client_handle = client_handle;
            _sub_handle = InitCbs();
        }

        unsafe public ExchangeClient(NashClientConfig config) {
            handleResult(
                ExchangeClient.InitNash(config.apikey, config.secret, config.clientId, config.environment, config.timeout, config.affiliateCode, out var client_handle)
            );
            _client_handle = client_handle;
            _sub_handle = InitCbs();
        }
        unsafe public ExchangeClient(CoinbaseClientConfig config) {
            handleResult(
                ExchangeClient.InitCoinbase(config.apikey, config.secret, config.passphrase, config.sandbox, out var client_handle)
            );
            _client_handle = client_handle;
            _sub_handle = InitCbs();
        }

        unsafe public double GetPriceTicker(string market) {
            var result = ExchangeClient.GetPriceTicker(_client_handle, market, out double price);
            return price;
        }
        unsafe public OrderbookResponse Orderbook(string market) {
            var bids = new FFIAskBid[512];
            var asks = new FFIAskBid[512];
            var bidsLen = bids.Length;
            var asksLen = asks.Length;
            var bidsList = new List<AskBid>();
            var asksList = new List<AskBid>();
            ulong lastUpdateId = 0;
            ulong updateId = 0;

            fixed (FFIAskBid* bidBuff = bids.AsSpan()) {
                fixed (FFIAskBid* askBuff = asks.AsSpan()) {
                    handleResult(
                        ExchangeClient.Orderbook(
                            _client_handle,
                            market,
                            (IntPtr)bidBuff, (ulong) bidsLen, out var actualBidsLen,
                            (IntPtr)askBuff, (ulong) asksLen, out var actualAsksLen,
                            out lastUpdateId,
                            out updateId
                        )
                    );
                    for (int i = 0 ; i < Math.Min(bidsLen, (int)actualBidsLen) ; i ++) {
                        bidsList.Add(bids[i].ToAskBid());
                        bids[i].Dispose();
                    }
                    for (int i = 0 ; i < Math.Min(asksLen, (int)actualAsksLen) ; i ++) {
                        asksList.Add(asks[i].ToAskBid());
                        asks[i].Dispose();
                    }
                }
            }

            return new OrderbookResponse(
                market,
                asksList,
                bidsList,
                lastUpdateId,
                updateId
            );
        }

         unsafe public IEnumerable<Candle> GetHistoricRates(GetHistoricRatesRequest req) {
            var limit = req.paginator == null ? 0 : req.paginator.limit;
            var candles = new Candle[Math.Max(limit, 256)];
            var candlesLen = candles.Length;
            var candlesList = new List<Candle>();
            

            fixed (Candle* candleBuff = candles.AsSpan()) {
                handleResult(ExchangeClient.GetHistoricRates(
                    _client_handle,
                    req.market, req.interval, req.paginator,
                    (IntPtr)candleBuff, (UIntPtr)candlesLen, out var actualCandleLen
                ));
                for (int i = 0 ; i < (int)actualCandleLen ; i ++) {
                    candlesList.Add(candles[i]);
                }
            }

            return candlesList;
        }
        unsafe public IEnumerable<Trade> GetHistoricTrades(GetHistoricTradesRequest req) {
            var limit = req.paginator == null ? 0 : req.paginator.limit;
            var trades = new FFITrade[Math.Max(limit, 256)];
            var tradesLen = trades.Length;
            var tradesList = new List<Trade>();
            

            fixed (FFITrade* tradeBuff = trades.AsSpan()) {
                handleResult(ExchangeClient.GetHistoricTrades(
                    _client_handle,
                    req.market,
                    req.paginator,
                    (IntPtr)tradeBuff, (UIntPtr)tradesLen, out var actualTradeLen
                ));
                for (int i = 0 ; i < (int)actualTradeLen ; i ++) {
                    tradesList.Add(trades[i].ToTrade());
                    trades[i].Dispose();
                }
            }

            return tradesList;
        }


        unsafe public Order LimitBuy(LimitOrderRequest request) {
            handleResult(ExchangeClient.PlaceOrder(
                _client_handle,
                request.market,
                request.size,
                true,
                request.price,
                Side.Buy,
                request.timeInForce,
                request.timeInForceDurationMs,
                request.postOnly,
                out FFIOrder ffiOrder
            ));
            var order = ffiOrder.ToOrder();
            ffiOrder.Dispose();
            return order;
        }
        unsafe public Order LimitSell(LimitOrderRequest request) {
            handleResult(ExchangeClient.PlaceOrder(
                _client_handle,
                request.market,
                request.size,
                true,
                request.price,
                Side.Sell,
                request.timeInForce,
                request.timeInForceDurationMs,
                request.postOnly,
                out FFIOrder ffiOrder
            ));
            var order = ffiOrder.ToOrder();
            ffiOrder.Dispose();
            return order;
        }

        unsafe public Order MarketBuy(MarketOrderRequest request) {
            handleResult(ExchangeClient.PlaceOrder(
                _client_handle,
                request.market,
                request.size,
                false,
                null,
                Side.Buy,
                TimeInForce.GTC,
                0,
                false,
                out FFIOrder ffiOrder
            ));
            var order = ffiOrder.ToOrder();
            ffiOrder.Dispose();
            return order;
        }

        unsafe public void CancelOrder(string orderId, string market) {
            handleResult(ExchangeClient.CancelOrder(
                _client_handle,
                orderId,
                market
            ));
        }

        unsafe public Order GetOrder(string orderId, string market) {
            handleResult(ExchangeClient.GetOrder(
                _client_handle,
                orderId,
                market,
                out var result
            ));

            var order = result.ToOrder();
            result.Dispose();
            return order;
        }

        unsafe public void CancelOrder(string orderId) {
            CancelOrder(orderId, null);
        }

        unsafe public Order MarketSell(MarketOrderRequest request) {
            handleResult(ExchangeClient.PlaceOrder(
                _client_handle,
                request.market,
                request.size,
                false,
                null,
                Side.Sell,
                TimeInForce.GTC,
                0,
                false,
                out FFIOrder ffiOrder
            ));
            var order = ffiOrder.ToOrder();
            ffiOrder.Dispose();
            return order;
        }
        unsafe public IEnumerable<Order> GetAllOpenOrders() {
            var orders = new FFIOrder[256];
            var ordersLen = orders.Length;
            var ordersList = new List<Order>();
            

            fixed (FFIOrder* orderBuff = orders.AsSpan()) {
                handleResult(ExchangeClient.GetAllOpenOrders(
                    _client_handle,
                    (IntPtr)orderBuff, (UIntPtr)ordersLen, out var actualCandleLen
                ));
                for (int i = 0 ; i < (int)actualCandleLen ; i ++) {
                    ordersList.Add(orderBuff[i].ToOrder());
                    orderBuff[i].Dispose();
                }
            }

            return ordersList;
        }

        unsafe public IEnumerable<Order> GetOrderHistory(GetOrderHistoryRequest req) {
            var limit = req.paginator == null ? 0 : req.paginator.limit;
            var orders = new FFIOrder[Math.Max(limit, 256)];
            var ordersLen = orders.Length;
            var ordersList = new List<Order>();
            

            fixed (FFIOrder* orderBuff = orders.AsSpan()) {
                handleResult(ExchangeClient.GetOrderHistory(
                    _client_handle,
                    req.market, req.paginator,
                    (IntPtr)orderBuff, (UIntPtr)ordersLen, out var actualCandleLen
                ));
                for (int i = 0 ; i < (int)actualCandleLen ; i ++) {
                    ordersList.Add(orderBuff[i].ToOrder());
                    orderBuff[i].Dispose();
                }
            }

            return ordersList;
        }

        unsafe public IEnumerable<Trade> GetTradeHistory(GetTradeHistoryRequest req) {
            var limit = req.paginator == null ? 0 : req.paginator.limit;
            var trades = new FFITrade[Math.Max(limit, 256)];
            var tradesLen = trades.Length;
            var tradesList = new List<Trade>();
            

            fixed (FFITrade* tradeBuff = trades.AsSpan()) {
                handleResult(ExchangeClient.GetTradeHistory(
                    _client_handle,
                    req.market, req.orderId, req.paginator,
                    (IntPtr)tradeBuff, (UIntPtr)tradesLen, out var actualCandleLen
                ));
                for (int i = 0 ; i < (int)actualCandleLen ; i ++) {
                    tradesList.Add(tradeBuff[i].ToTrade());
                    tradeBuff[i].Dispose();
                }
            }

            return tradesList;
        }
    
        unsafe public IEnumerable<Balance> GetAccountBalances(Paginator paginator) {
            var limit = paginator == null ? 0 : paginator.limit;
            var balances = new FFIBalance[Math.Max(limit, 256)];
            var balancesLen = balances.Length;
            var balancesList = new List<Balance>();
            

            fixed (FFIBalance* balanceBuff = balances.AsSpan()) {
                handleResult(ExchangeClient.GetAccountBalances(
                    _client_handle,
                    paginator,
                    (IntPtr)balanceBuff, (UIntPtr)balancesLen, out var actualCandleLen
                ));
                for (int i = 0 ; i < (int)actualCandleLen ; i ++) {
                    balancesList.Add(balanceBuff[i].ToBalance());
                    balanceBuff[i].Dispose();
                }
            }

            return balancesList;
        }
        public IEnumerable<Balance> GetAccountBalances() {
            return this.GetAccountBalances(null);
        }

        unsafe public IEnumerable<string> CancelAllOrders(string market) {
            var orders = new IntPtr[1024];
            var ordersLen = orders.Length;
            var cancelledOrdersList = new List<String>();
            fixed (IntPtr* orderBuff = orders.AsSpan()) {
                 handleResult(ExchangeClient.CancelAllOrders(
                    _client_handle,
                    market,
                    (IntPtr)orderBuff, (UIntPtr)ordersLen, out var actualLen
                ));
                for (int i = 0 ; i < (int)actualLen ; i ++) {
                    cancelledOrdersList.Add(CString.ToString(orders[i]));
                    ExchangeClient.FreeString(orders[i]);
                }
            }
            return cancelledOrdersList;
        }

        unsafe public IEnumerable<MarketPair> ReceivePairs() {
            var marketPairs = new FFIMarketPair[1024];
            var marketPairsLen = marketPairs.Length;
            var pairs = new List<MarketPair>();
            fixed (FFIMarketPair* buff = marketPairs.AsSpan()) {
                 handleResult(ExchangeClient.ReceivePairs(
                    _client_handle,
                    (IntPtr)buff, (UIntPtr)marketPairsLen, out var actualLen
                ));
                for (int i = 0 ; i < (int)actualLen ; i ++) {
                    pairs.Add(marketPairs[i].ToMarketPair());
                    marketPairs[i].Dispose();
                }
            }
            return pairs;
        }

        private void WaitForEwh() {
            ewh.WaitOne();
        }

        private void SetupEWH() {
            if (ewhThreadHandle != null) {
                return;
            }

            ewhThreadHandle = new Thread(this.WaitForEwh);
            ewhThreadHandle.Start();
        }

        public void Listen(
            OnError onError,
            OnPing onPing
        ) {
            this.onErrorCbs.Add(onError);
            this.onPingCbs.Add(onPing);

            this.SetupEWH();

        }

        unsafe public void SubscribeToOrderbook(string market, OnOrderbook onOrderbook) {
            if (!this.onOrderbookCbs.ContainsKey(market)) {
                this.onOrderbookCbs.Add(market, new List<OnOrderbook>());
            }
            this.onOrderbookCbs.TryGetValue(market, out var callbacks);
            callbacks.Add(onOrderbook);
            handleFFIResult(SubscribeToOrderbook(this._client_handle, this._sub_handle, market));
            this.SetupEWH();
        }
        unsafe public void SubscribeToTrades(string market, OnTrades onTrades) {
            if (!this.onTradesCbs.ContainsKey(market)) {
                this.onTradesCbs.Add(market, new List<OnTrades>());
            }
            this.onTradesCbs.TryGetValue(market, out var callbacks);
            callbacks.Add(onTrades);
            handleFFIResult(SubscribeToTrades(this._client_handle, this._sub_handle, market));
            this.SetupEWH();
        }

        public void SubscribeToDisconnect(OnDisconnect cb) {
            this.onDisconnectCbs.Add(cb);
        }

        unsafe public void Disconnect() {
            Disconnect(_sub_handle);
        }
    }
}
