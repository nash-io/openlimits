        public static implicit operator string(FFIString from) {
            unsafe {
                return Marshal.PtrToStringUTF8(from.Pointer());
            }
        }

        public static implicit operator FFIString(string from) {
            unsafe {
                return new FFIString(Marshal.StringToHGlobalAnsi(from));
                // FIXME: Memory leak https://docs.microsoft.com/pt-br/dotnet/api/system.runtime.interopservices.marshal.freehglobal?view=net-5.0
            }
        }