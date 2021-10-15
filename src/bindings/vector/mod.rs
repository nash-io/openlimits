use ligen_macro::inner_ligen;
use ligen::marshalling::MarshalFrom;

inner_ligen! {
    ffi(
        Vec(name = "FFIVector")
    ),
    csharp(
        ffi(
            Vec(
                name = "FFIVector"
            ),
            FFIVector(
                name = "FFIVector"
            )
        ),
        marshal(
            Vec(
                name = "List"
            ),
            FFIVector(
                name = "List",
                methods = "src/bindings/vector/vector.methods.cs",
                generics = "<T> where T: unmanaged"
            )
        ),
    )
}

#[repr(C)]
pub struct FFIVector<T> {
    pub pointer: *mut T,
    pub length: u64
}

impl<T: Clone> MarshalFrom<FFIVector<T>> for Vec<T> {
    fn marshal_from(from: FFIVector<T>) -> Self {
        let length = from.length as usize;
        unsafe {
            std::slice::from_raw_parts(from.pointer, length).to_vec()
        }
    }
}

impl<T> MarshalFrom<Vec<T>> for FFIVector<T> {
    fn marshal_from(mut from: Vec<T>) -> Self {
        let pointer = from.as_mut_ptr();
        let length = from.len() as u64;
        // FIXME: Memory leak.
        std::mem::forget(from);
        Self { pointer, length }
    }
}
