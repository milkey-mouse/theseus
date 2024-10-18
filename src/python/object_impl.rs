use std::{
    ffi::c_char,
    ptr::{self, NonNull},
};

use super::*;

impl BaseException {
    pub fn current() -> Option<BorrowedRef<Self>> {
        NonNull::new(unsafe { ffi::PyErr_Occurred() }).map(BorrowedRef::from)
    }
}

impl Module {
    pub fn import(name: &BorrowedRef<Unicode>) -> StrongRef<Self> {
        unsafe { StrongRef::from(Exception::handle(ffi::PyImport_Import(name.as_ptr()))) }
    }
}

impl Tuple {
    pub fn new(size: ffi::Py_ssize_t) -> StrongRef<Self> {
        unsafe { StrongRef::from(Exception::handle(ffi::PyTuple_New(size))) }
    }
}

impl Unicode {
    pub fn new(string: impl AsRef<[u8]>) -> StrongRef<Self> {
        let string = string.as_ref();
        unsafe {
            StrongRef::from(Exception::handle(ffi::PyUnicode_DecodeUTF8(
                string.as_ptr() as *const c_char,
                ffi::Py_ssize_t::try_from(string.len())
                    .expect("string larger than ffi::Py_ssize_t::MAX"),
                ptr::null(),
            )))
        }
    }
}
