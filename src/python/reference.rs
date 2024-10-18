#![deny(unused_results)]
use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use super::{
    ffi,
    object::{InstanceOf, PyObjectWrapper},
    Object, Type,
};

pub struct BorrowedRef<T: PyObjectWrapper>(NonNull<T::Wrapped>, PhantomData<T>);

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

impl<T: PyObjectWrapper> BorrowedRef<T> {
    pub unsafe fn as_ref(&self) -> &T::Wrapped {
        unsafe { self.0.cast().as_ref() }
    }

    pub unsafe fn as_mut(&mut self) -> &mut T::Wrapped {
        unsafe { self.0.cast().as_mut() }
    }

    pub fn as_ptr<U>(&self) -> *mut U
    where
        T::Wrapped: InstanceOf<U>,
    {
        self.0.cast().as_ptr()
    }

    pub unsafe fn is_type(&self, _type: BorrowedRef<Type>) -> bool {
        unsafe {
            let tp = _type.as_ptr() as *mut ffi::PyTypeObject;
            ffi::PyObject_TypeCheck(self.as_ptr(), tp) != 0
        }
    }

    pub unsafe fn cast_unchecked<U: PyObjectWrapper>(&self) -> BorrowedRef<U> {
        BorrowedRef(self.0.cast(), PhantomData)
    }

    pub fn cast_super<U: PyObjectWrapper>(&self) -> BorrowedRef<U>
    where
        T::Wrapped: InstanceOf<U::Wrapped>,
    {
        unsafe { self.cast_unchecked() }
    }

    pub unsafe fn cast_dyn<U: PyObjectWrapper>(&self) -> Option<BorrowedRef<U>> {
        unsafe {
            match self.is_type(U::py_type()) {
                true => Some(self.cast_unchecked()),
                false => None,
            }
        }
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
