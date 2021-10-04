using System;
using System.Runtime.InteropServices;

namespace OpenLimits {
    public class Test {
        public static void Hello() {
            Console.WriteLine("Hello");
        }

        const string NativeLib = "libopenlimits_sharp";

        [DllImport(NativeLib, EntryPoint = "say_something", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe public static extern void SaySomething();

    }
}