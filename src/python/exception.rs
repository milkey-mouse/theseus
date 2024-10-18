#![deny(unused_results)]
use std::{
    any::Any,
    ffi::{c_int, c_uint},
    mem::{self, MaybeUninit},
    panic,
    ptr::{self, NonNull},
};

use pyo3_ffi::PyErr_SetObject;

use crate::send_cell::SendCell;

use super::{
    ffi, pyvarobject_head_init, BaseException, BorrowedRef, InstanceOf, PyObjectWrapper, StrongRef,
    Tuple, Type,
};

pub trait PyResult: Sized {
    type Output;

    unsafe fn ok_or_exception(self) -> Result<Self::Output, BorrowedRef<BaseException>>;

    unsafe fn unwrap_or_exception(self) -> Self::Output {
        unsafe {
            self.ok_or_exception()
                .unwrap_or_else(|exception| unsafe { Exception::throw(exception) })
        }
    }
}

impl<T> PyResult for *mut T {
    type Output = NonNull<T>;

    unsafe fn ok_or_exception(self) -> Result<Self::Output, BorrowedRef<BaseException>> {
        match NonNull::new(self) {
            Some(ptr) => Ok(ptr),
            None => Err(BaseException::current().unwrap()),
        }
    }
}

impl PyResult for c_int {
    type Output = ();

    unsafe fn ok_or_exception(self) -> Result<Self::Output, BorrowedRef<BaseException>> {
        match self {
            0 => Ok(()),
            _ => Err(BaseException::current().unwrap()),
        }
    }
}

impl PyResult for c_uint {
    type Output = ();

    unsafe fn ok_or_exception(self) -> Result<Self::Output, BorrowedRef<BaseException>> {
        match self {
            0 => Ok(()),
            _ => Err(BaseException::current().unwrap()),
        }
    }
}

impl<T> PyResult for Option<T> {
    type Output = T;

    unsafe fn ok_or_exception(self) -> Result<Self::Output, BorrowedRef<BaseException>> {
        match self {
            Some(value) => Ok(value),
            None => Err(BaseException::current().unwrap()),
        }
    }
}

impl PyResult for () {
    type Output = ();

    unsafe fn ok_or_exception(self) -> Result<Self::Output, BorrowedRef<BaseException>> {
        match BaseException::current() {
            Some(exception) => Err(exception),
            None => Ok(()),
        }
    }
}

pub struct Exception(SendCell<StrongRef<BaseException>>);

impl Exception {
    pub(super) unsafe fn catch_unwind<T, F: FnOnce() -> *mut T + panic::UnwindSafe>(
        &self,
        f: F,
    ) -> *mut T {
        match panic::catch_unwind(f) {
            Ok(value) => value,
            Err(payload) => unsafe {
                PyErr_SetObject(
                    PanicException::py_type().as_ptr(),
                    PanicException::new(Some(payload)).as_ptr(),
                );
                ptr::null_mut()
            },
        }
    }

    pub(super) unsafe fn throw(exception: BorrowedRef<BaseException>) -> ! {
        unsafe {
            if let Some(mut exception) = exception.cast_dyn::<PanicException>() {
                let payload = exception.take().unwrap();
                ffi::PyErr_Clear();
                panic::resume_unwind(payload)
            } else {
                // TODO: make these exceptions print correctly
                panic::panic_any(Exception(SendCell::new(StrongRef::from(exception))))
            }
        }
    }

    pub(super) unsafe fn handle_unchecked() -> ! {
        unsafe { Self::throw(BaseException::current().unwrap()) }
    }

    pub(super) unsafe fn handle<T: PyResult>(maybe_exception: T) -> T::Output {
        unsafe {
            maybe_exception
                .ok_or_exception()
                .unwrap_or_else(|exception| unsafe { Self::throw(exception) })
        }
    }
}

fn _pyexc_baseexception() -> *mut ffi::PyTypeObject {
    unsafe { ffi::PyExc_BaseException as *mut ffi::PyTypeObject }
}

static mut PANIC_EXCEPTION_TYPE: MaybeUninit<ffi::PyTypeObject> = MaybeUninit::zeroed();

#[repr(C)]
struct PanicExceptionObject {
    base: ffi::PyBaseExceptionObject,
    payload: Option<Box<dyn Any + Send>>,
}

unsafe impl InstanceOf<ffi::PyObject> for PanicExceptionObject {}
unsafe impl InstanceOf<ffi::PyBaseExceptionObject> for PanicExceptionObject {}

#[derive(Clone, Copy)]
struct PanicException;

unsafe impl PyObjectWrapper for PanicException {
    const NAME: &'static str = "PanicException";

    type Wrapped = PanicExceptionObject;

    fn py_type() -> BorrowedRef<Type> {
        unsafe {
            let ptr = (&raw mut PANIC_EXCEPTION_TYPE) as *mut ffi::PyTypeObject;

            if (&raw mut (*ptr).tp_flags).read() & ffi::Py_TPFLAGS_READY != 0 {
                (&raw mut (*ptr).ob_base).write(pyvarobject_head_init(ptr::null_mut(), 0));
                (&raw mut (*ptr).tp_name).write(c"PanicException".as_ptr());
                (&raw mut (*ptr).tp_basicsize).write(mem::size_of::<PanicExceptionObject>() as _);
                (&raw mut (*ptr).tp_flags).write(
                    // TODO: other flags pls
                    ffi::Py_TPFLAGS_DEFAULT
                        | ffi::Py_TPFLAGS_HAVE_GC
                        | ffi::Py_TPFLAGS_DISALLOW_INSTANTIATION,
                );
                (&raw mut (*ptr).tp_doc).write(c"Rust code panicked".as_ptr());
                (&raw mut (*ptr).tp_dictoffset)
                    .write(mem::offset_of!(PanicExceptionObject, base.dict) as _);
                (&raw mut (*ptr).tp_base).write(_pyexc_baseexception());
                (&raw mut (*ptr).tp_dealloc).write(Some(PanicException::dealloc));

                // GC-related things; we can keep BaseException's impl
                // because no fields we add can contain `ffi::PyObject`s
                (&raw mut (*ptr).tp_traverse).write((*_pyexc_baseexception()).tp_traverse);
                (&raw mut (*ptr).tp_clear).write((*_pyexc_baseexception()).tp_clear);

                // unneeded when ffi::Py_TPFLAGS_DISALLOW_INSTANTIATION is set
                //(&raw mut (*ptr).tp_init).write(_pyexc_baseexception().tp_init);
                //(&raw mut (*ptr).tp_new).write(PanicException::new);

                Exception::handle(ffi::PyType_Ready(ptr));
            }

            BorrowedRef::from(NonNull::new_unchecked(ptr))
        }
    }
}

impl PanicException {
    fn new(payload: Option<Box<dyn Any + Send>>) -> StrongRef<PanicException> {
        unsafe {
            let baseexception_new = (*_pyexc_baseexception()).tp_new.unwrap_unchecked();
            let subtype = Self::py_type().as_ptr();
            let args = Tuple::new(0);
            let kwargs = ptr::null_mut();
            let exception: NonNull<PanicExceptionObject> =
                Exception::handle(baseexception_new(subtype, args.as_ptr(), kwargs)).cast();
            (&raw mut (*exception.as_ptr()).payload).write(payload);
            StrongRef::from(exception)
        }
    }

    extern "C" fn dealloc(ptr: *mut ffi::PyObject) {
        unsafe {
            let baseexception_dealloc = (*_pyexc_baseexception()).tp_dealloc.unwrap_unchecked();
            baseexception_dealloc(ptr);
            ptr::drop_in_place(&raw mut (*(ptr as *mut PanicExceptionObject)).payload);
        }
    }
}

impl BorrowedRef<PanicException> {
    unsafe fn take(&mut self) -> Option<Box<dyn Any + Send>> {
        unsafe { self.as_mut() }.payload.take()
    }
}

/*
if let Some(string) = payload.downcast_ref::<String>() {
    Self::new_err((string.clone(),))
} else if let Some(s) = payload.downcast_ref::<&str>() {
    Self::new_err((s.to_string(),))
} else {
    Self::new_err(("panic from Rust code",))
}
 */
