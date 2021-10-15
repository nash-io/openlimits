use ligen::marshalling::{MarshalFrom, MarshalInto};
use ligen_macro::inner_ligen;
use rust_decimal::Decimal;

inner_ligen! {
    FFIString(ignore = true),
    ffi(
        String(name = "FFIString")
    ),
    csharp(
        marshal(
            FFIString(
                name = "string",
                MarshalAs = "UnmanagedType.LPStr"
            ),
            String(
                name = "string",
                MarshalAs = "UnmanagedType.LPStr"
            )
        ),
    )
}

#[repr(C)]
pub struct FFIString {
    pub pointer: *mut i8
}

impl MarshalFrom<FFIString> for String {
    fn marshal_from(value: FFIString) -> Self {
        #[allow(unsafe_code)]
        unsafe {
            if value.pointer.is_null() {
                "".into()
            } else {
                let cstr = std::ffi::CStr::from_ptr(value.pointer);
                cstr.to_string_lossy().to_string()
            }
        }
    }
}

impl MarshalFrom<String> for FFIString {
    fn marshal_from(value: String) -> Self {
        let mut value = value;
        let pointer = value.as_mut_ptr() as *mut i8;
        // FIXME: Memory leak.
        Box::into_raw(Box::new(value));
        Self { pointer }
    }
}

impl MarshalFrom<Decimal> for FFIString {
    fn marshal_from(value: Decimal) -> Self {
        value.to_string().marshal_into()
    }
}
