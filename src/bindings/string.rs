use ligen::marshalling::MarshalFrom;

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
