namespace OpenLimits
{
    
    using System;
    using System.Runtime.InteropServices;

    // Used to store a result from rust to C#
    [StructLayout(LayoutKind.Sequential)]
    internal struct FFIResult {
        public ResultTag tag;
        public IntPtr message;
    }

    // Contains the different error types
    internal enum ResultTag {
        Ok,
        InvalidArgument,
        BinanceError,
        CoinbaseError,
        NashProtocolError,
        MissingImplementation,
        AssetNotFound,
        NoApiKeySet,
        InternalServerError,
        ServiceUnavailable,
        Unauthorized,
        SymbolNotFound,
        SocketError,
        GetTimestampFailed,
        ReqError,
        InvalidHeaderError,
        InvalidPayloadSignature,
        IoError,
        PoisonError,
        JsonError,
        ParseFloatError,
        UrlParserError,
        Tungstenite,
        TimestampError,
        UnkownResponse,
        NotParsableResponse,
        MissingParameter,
        WebSocketMessageNotSupported,

        InitializeException,
        SubscribeException,
        NoMarketPair
    }

    public class OpenLimitsError: Exception {
        public OpenLimitsError(string message): base(message) { }

    }

    public class BinanceError : OpenLimitsError {
        public BinanceError(string message): base(message) { }
    };
    public class CoinbaseError : OpenLimitsError {
        public CoinbaseError(string message): base(message) { }
    };
    public class NashProtocolError : OpenLimitsError {
        public NashProtocolError(string message): base(message) { }
    };
    public class MissingImplementation : OpenLimitsError {
        public MissingImplementation(string message): base(message) { }
    };
    public class AssetNotFound : OpenLimitsError {
        public AssetNotFound(string message): base(message) { }
    };
    public class NoApiKeySet : OpenLimitsError {
        public NoApiKeySet(string message): base(message) { }
    };
    public class InternalServerError : OpenLimitsError {
        public InternalServerError(string message): base(message) { }
    };
    public class ServiceUnavailable : OpenLimitsError {
        public ServiceUnavailable(string message): base(message) { }
    };
    public class Unauthorized : OpenLimitsError {
        public Unauthorized(string message): base(message) { }
    };
    public class SymbolNotFound : OpenLimitsError {
        public SymbolNotFound(string message): base(message) { }
    };
    public class SocketError : OpenLimitsError {
        public SocketError(string message): base(message) { }
    };
    public class GetTimestampFailed : OpenLimitsError {
        public GetTimestampFailed(string message): base(message) { }
    };
    public class ReqError : OpenLimitsError {
        public ReqError(string message): base(message) { }
    };
    public class InvalidHeaderError : OpenLimitsError {
        public InvalidHeaderError(string message): base(message) { }
    };
    public class InvalidPayloadSignature : OpenLimitsError {
        public InvalidPayloadSignature(string message): base(message) { }
    };
    public class IoError : OpenLimitsError {
        public IoError(string message): base(message) { }
    };
    public class PoisonError : OpenLimitsError {
        public PoisonError(string message): base(message) { }
    };
    public class JsonError : OpenLimitsError {
        public JsonError(string message): base(message) { }
    };
    public class ParseFloatError : OpenLimitsError {
        public ParseFloatError(string message): base(message) { }
    };
    public class UrlParserError : OpenLimitsError {
        public UrlParserError(string message): base(message) { }
    };
    public class Tungstenite : OpenLimitsError {
        public Tungstenite(string message): base(message) { }
    };
    public class TimestampError : OpenLimitsError {
        public TimestampError(string message): base(message) { }
    };
    public class UnkownResponse : OpenLimitsError {
        public UnkownResponse(string message): base(message) { }
    };
    public class NotParsableResponse : OpenLimitsError {
        public NotParsableResponse(string message): base(message) { }
    };
    public class MissingParameter : OpenLimitsError {
        public MissingParameter(string message): base(message) { }
    };
    public class WebSocketMessageNotSupported : OpenLimitsError {
        public WebSocketMessageNotSupported(string message): base(message) { }
    };

    public class InitializeException : OpenLimitsError {
        public InitializeException(string message): base(message) { }
    };
    public class SubscribeException : OpenLimitsError {
        public SubscribeException(string message): base(message) { }
    };
    public class NoMarketPair : OpenLimitsError {
        public NoMarketPair(string message): base(message) { }
    };
}