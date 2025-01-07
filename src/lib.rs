pub mod fons;
use std::ffi;

unsafe extern "C" {
    pub fn sfons_flush(s: *mut fons::FONScontext);
}

unsafe extern "C" {
    pub fn sfons_create_context(width: ffi::c_int, height: ffi::c_int) -> *mut fons::FONScontext;
}

unsafe extern "C" {
    pub fn fonsAddFontMem(
        s: *mut fons::FONScontext,
        name: *mut ffi::c_char,
        data: * mut ffi::c_uchar,
        data_len: ffi::c_int,
        free_data: ffi::c_int,
    ) -> ffi::c_int;
}

