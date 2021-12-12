        public static implicit operator string(FFIString from) {
            unsafe {
                return Marshal.PtrToStringUTF8(from.GetPointer());
            }
        }

        public static implicit operator FFIString(string from) {
            unsafe {
                return FFIString.New(Marshal.StringToHGlobalAnsi(from));
            }
        }

        public static implicit operator IntPtr(FFIString from) {
            unsafe {
                return from.GetPointer();
            }
        }
