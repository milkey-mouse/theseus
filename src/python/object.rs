#![deny(unused_results)]
use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use super::{
    ffi,
    types::{InstanceOf, PyObjectWrapper},
    Object, Type, TypeObject, UnicodeObject,
};

pub const fn pyobject_head_init(_type: *mut ffi::PyTypeObject) -> ffi::PyObject {
    ffi::PyObject {
        ob_refcnt: ffi::PyObjectObRefcnt {
            ob_refcnt: ffi::_Py_IMMORTAL_REFCNT,
        },
        ob_type: _type,
    }
}

pub const fn pyvarobject_head_init(
    r#type: *mut ffi::PyTypeObject,
    size: isize,
) -> ffi::PyVarObject {
    ffi::PyVarObject {
        ob_base: pyobject_head_init(r#type),
        ob_size: size,
    }
}

pub unsafe trait PyObjectWrapper {
    type Inner;
}

pub unsafe trait PyTypeInfo {
    const NAME: &'static str;

    fn type_obj() -> *mut TypeObject;

    fn check(obj: &Object) -> bool; // TODO: copy pyo3 fr fr no cap

    // TODO: other methods
}

#[macro_export]
macro_rules! impl_pyobject_wrapper {
    ($name:ident, $super:path, $ffi_name:path, $type_obj:expr, $check_fn:path) => {
        ::$crate::impl_pyobject_wrapper!($name, $super, $ffi_name);

        unsafe impl ::$crate::python::object::PyTypeInfo for $name {
            const NAME: &'static str = stringify!($name);

            fn type_obj() -> *mut ::$crate::python::types::TypeObject {
                let type_obj: *mut ::$crate::python::ffi::PyTypeObject = $type_obj;
                type_obj as *mut ::$crate::python::types::TypeObject
            }

            fn check(obj: &::$crate::python::object::Object) -> bool {
                // TODO
                unsafe { $check_fn(obj) }
            }
        }
    };
    ($name:ident, $super:path, $ffi_name:path) => {
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $name($super);

        unsafe impl ::$crate::python::object::PyObjectWrapper for $name {
            type Inner = $ffi_name;
        }

        impl ::core::ops::Deref for $name {
            type Target = $super;

            fn deref(&self) -> &Self::Target {
                unsafe { &self.0 }
            }
        }
    };
}

pub struct BorrowedRef<T: PyObjectWrapper>(T);

impl<T: PyObjectWrapper> Clone for BorrowedRef<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T, Subtype> From<NonNull<Subtype>> for BorrowedRef<T>
where
    T: PyObjectWrapper,
    Subtype: InstanceOf<T::Wrapped>,
{
    fn from(inner: NonNull<Subtype>) -> Self {
        Self(inner.cast(), PhantomData)
    }
}

pub struct StrongRef<T: PyObjectWrapper>(BorrowedRef<T>);

impl<T: PyObjectWrapper> Deref for StrongRef<T> {
    type Target = BorrowedRef<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, Subtype> From<NonNull<Subtype>> for StrongRef<T>
where
    T: PyObjectWrapper,
    Subtype: InstanceOf<T::Wrapped>,
{
    fn from(inner: NonNull<Subtype>) -> Self {
        // don't use Self::from; we don't want to increase the refcount
        Self(BorrowedRef::from(inner))
    }
}

impl<T: PyObjectWrapper> From<BorrowedRef<T>> for StrongRef<T> {
    fn from(borrowed: BorrowedRef<T>) -> Self {
        unsafe {
            let new_ref = NonNull::new_unchecked(ffi::Py_NewRef(borrowed.as_ptr()));
            Self(BorrowedRef::<T>::from(new_ref.cast::<T::Wrapped>()))
        }
    }
}

impl<T: PyObjectWrapper> Clone for StrongRef<T> {
    fn clone(&self) -> Self {
        StrongRef::from(self.0.clone())
    }
}

impl<T: PyObjectWrapper> Drop for StrongRef<T> {
    fn drop(&mut self) {
        unsafe { ffi::Py_DecRef(self.as_ptr()) };
    }
}

impl<T: PyObjectWrapper> StrongRef<T> {
    pub unsafe fn cast_unchecked<U: PyObjectWrapper>(self) -> StrongRef<U> {
        StrongRef(unsafe { self.0.cast_unchecked() })
    }

    pub fn cast_super<U: PyObjectWrapper>(self) -> StrongRef<U>
    where
        T::Wrapped: InstanceOf<U::Wrapped>,
    {
        unsafe { self.cast_unchecked() }
    }

    pub fn cast_dyn<U: PyObjectWrapper>(self) -> StrongRef<U> {
        unsafe {
            match self.is_type(U::py_type()) {
                true => self.cast_unchecked(),
                false => panic!("{:?} could not be cast to {:?}", T::NAME, U::NAME),
            }
        }
    }
}

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

impl UnicodeObject {
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
