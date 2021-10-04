namespace OpenLimits
{
    using System.Runtime.InteropServices;
    using System;
    using System.Text;

    public class CString {
        public static string ToString(IntPtr handle) {
            if (handle.ToInt64() == 0) {
                return null;
            }
            int len = 0;
            while (Marshal.ReadByte(handle,len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }
    }
}