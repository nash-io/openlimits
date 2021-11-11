        public static implicit operator FFIVector<T>(List<T> from) {
            var array = from.ToArray();
            ulong length = (ulong) from.Count;
            GCHandle handle = GCHandle.Alloc(array, GCHandleType.Pinned);
            try {
                IntPtr pointer = handle.AddrOfPinnedObject();
                return new FFIVector<T>(pointer, length);
            } finally {
                // FIXME: Memory leak? It seems to be garbage collected.
                // if (handle.IsAllocated)
                //    handle.Free();
            }
        }

        public static implicit operator List<T>(FFIVector<T> from) {
            unsafe {
                T[] array = new T[from.length];
                fixed (T* apointer = array) {
                    long length = (long) from.length * (long) sizeof(T);
                    Buffer.MemoryCopy((void*) from.pointer, (void*) apointer, length, length);
                    return new List<T>(array);
                }
            }
        }