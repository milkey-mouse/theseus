#![deny(unused_results)]

use super::{ffi, BorrowedRef};

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

// TODO: automate generation of these based on first field of each type

pub unsafe trait InstanceOf<T> {}

unsafe impl<T> InstanceOf<T> for T {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyASCIIObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyBaseExceptionObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyByteArrayObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyByteArrayObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyBytesObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyBytesObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyCFunctionObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyCMethodObject {}
unsafe impl InstanceOf<ffi::PyCFunctionObject> for ffi::PyCMethodObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyCodeObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyCodeObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyCompactUnicodeObject {}
unsafe impl InstanceOf<ffi::PyASCIIObject> for ffi::PyCompactUnicodeObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyComplexObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyDateTime_Date {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyDateTime_DateTime {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyDateTime_Delta {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyDateTime_Time {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyDescrObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyDictObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyFloatObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyFrameObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyFunctionObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyGenObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyGetSetDescrObject {}
unsafe impl InstanceOf<ffi::PyDescrObject> for ffi::PyGetSetDescrObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyHeapTypeObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyHeapTypeObject {}
unsafe impl InstanceOf<ffi::PyTypeObject> for ffi::PyHeapTypeObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyImportErrorObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyListObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyListObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyLongObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyMemberDescrObject {}
unsafe impl InstanceOf<ffi::PyDescrObject> for ffi::PyMemberDescrObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyMethodDescrObject {}
unsafe impl InstanceOf<ffi::PyDescrObject> for ffi::PyMethodDescrObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyModuleDef {}
unsafe impl InstanceOf<ffi::PyModuleDef_Base> for ffi::PyModuleDef {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyModuleDef_Base {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyOSErrorObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PySetObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PySliceObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyStopIterationObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PySyntaxErrorObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PySystemExitObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyTupleObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyTupleObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyTypeObject {}
unsafe impl InstanceOf<ffi::PyVarObject> for ffi::PyTypeObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyUnicodeErrorObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyUnicodeObject {}
unsafe impl InstanceOf<ffi::PyASCIIObject> for ffi::PyUnicodeObject {}
unsafe impl InstanceOf<ffi::PyCompactUnicodeObject> for ffi::PyUnicodeObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyVarObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::PyWrapperDescrObject {}
unsafe impl InstanceOf<ffi::PyDescrObject> for ffi::PyWrapperDescrObject {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::_PyDateTime_BaseDateTime {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::_PyDateTime_BaseTime {}

unsafe impl InstanceOf<ffi::PyObject> for ffi::_PyWeakReference {}

pub unsafe trait PyObjectWrapper: Copy {
    const NAME: &'static str;

    type Wrapped: InstanceOf<ffi::PyObject>;

    fn py_type() -> BorrowedRef<Type>;
}

macro_rules! _impl_py_type {
    ($tag:ident, $ty:expr) => {
        _impl_py_type!($tag, $ty, ffi::PyObject);
    };
    ($tag:ident, $ty:expr, $obj:path) => {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $tag;

        unsafe impl crate::python::object::PyObjectWrapper for $tag {
            const NAME: &'static str = stringify!($tag);

            type Wrapped = $obj;

            fn py_type() -> crate::python::BorrowedRef<crate::python::object::Type> {
                let _ty: *mut crate::python::ffi::PyTypeObject = $ty;
                unsafe {
                    let ptr = core::ptr::NonNull::new_unchecked(_ty);
                    crate::python::BorrowedRef::from(ptr)
                }
            }
        }
    };
}

// TODO: add Wrapped types more specific than ffi::PyObject where possible/needed

use _impl_py_type as impl_py_type;

impl_py_type!(ArithmeticError, unsafe {
    ffi::PyExc_ArithmeticError as *mut ffi::PyTypeObject
});
impl_py_type!(AssertionError, unsafe {
    ffi::PyExc_AssertionError as *mut ffi::PyTypeObject
});
impl_py_type!(AsyncGen, &raw mut ffi::PyAsyncGen_Type);
impl_py_type!(AttributeError, unsafe {
    ffi::PyExc_AttributeError as *mut ffi::PyTypeObject
});
impl_py_type!(BaseException, unsafe {
    ffi::PyExc_BaseException as *mut ffi::PyTypeObject
});
impl_py_type!(BaseExceptionGroup, unsafe {
    ffi::PyExc_BaseExceptionGroup as *mut ffi::PyTypeObject
});
impl_py_type!(BlockingIOError, unsafe {
    ffi::PyExc_BlockingIOError as *mut ffi::PyTypeObject
});
impl_py_type!(Bool, &raw mut ffi::PyBool_Type);
impl_py_type!(BrokenPipeError, unsafe {
    ffi::PyExc_BrokenPipeError as *mut ffi::PyTypeObject
});
impl_py_type!(BufferError, unsafe {
    ffi::PyExc_BufferError as *mut ffi::PyTypeObject
});
impl_py_type!(ByteArray, &raw mut ffi::PyByteArray_Type);
impl_py_type!(ByteArrayIter, &raw mut ffi::PyByteArrayIter_Type);
impl_py_type!(Bytes, &raw mut ffi::PyBytes_Type);
impl_py_type!(BytesIter, &raw mut ffi::PyBytesIter_Type);
impl_py_type!(BytesWarning, unsafe {
    ffi::PyExc_BytesWarning as *mut ffi::PyTypeObject
});
impl_py_type!(CFunction, &raw mut ffi::PyCFunction_Type);
impl_py_type!(CMethod, &raw mut ffi::PyCMethod_Type);
impl_py_type!(CallIter, &raw mut ffi::PyCallIter_Type);
impl_py_type!(Capsule, &raw mut ffi::PyCapsule_Type);
impl_py_type!(ChildProcessError, unsafe {
    ffi::PyExc_ChildProcessError as *mut ffi::PyTypeObject
});
impl_py_type!(ClassMethodDescr, &raw mut ffi::PyClassMethodDescr_Type);
impl_py_type!(Code, &raw mut ffi::PyCode_Type);
impl_py_type!(Complex, &raw mut ffi::PyComplex_Type);
impl_py_type!(ConnectionAbortedError, unsafe {
    ffi::PyExc_ConnectionAbortedError as *mut ffi::PyTypeObject
});
impl_py_type!(ConnectionError, unsafe {
    ffi::PyExc_ConnectionError as *mut ffi::PyTypeObject
});
impl_py_type!(ConnectionRefusedError, unsafe {
    ffi::PyExc_ConnectionRefusedError as *mut ffi::PyTypeObject
});
impl_py_type!(ConnectionResetError, unsafe {
    ffi::PyExc_ConnectionResetError as *mut ffi::PyTypeObject
});
impl_py_type!(Context, &raw mut ffi::PyContext_Type);
impl_py_type!(ContextToken, &raw mut ffi::PyContextToken_Type);
impl_py_type!(ContextVar, &raw mut ffi::PyContextVar_Type);
impl_py_type!(Coro, &raw mut ffi::PyCoro_Type);
impl_py_type!(CoroWrapper, &raw mut ffi::_PyCoroWrapper_Type);
impl_py_type!(DeprecationWarning, unsafe {
    ffi::PyExc_DeprecationWarning as *mut ffi::PyTypeObject
});
impl_py_type!(Dict, &raw mut ffi::PyDict_Type);
impl_py_type!(DictItems, &raw mut ffi::PyDictItems_Type);
impl_py_type!(DictIterItem, &raw mut ffi::PyDictIterItem_Type);
impl_py_type!(DictIterKey, &raw mut ffi::PyDictIterKey_Type);
impl_py_type!(DictIterValue, &raw mut ffi::PyDictIterValue_Type);
impl_py_type!(DictKeys, &raw mut ffi::PyDictKeys_Type);
impl_py_type!(DictProxy, &raw mut ffi::PyDictProxy_Type);
impl_py_type!(DictRevIterItem, &raw mut ffi::PyDictRevIterItem_Type);
impl_py_type!(DictRevIterKey, &raw mut ffi::PyDictRevIterKey_Type);
impl_py_type!(DictRevIterValue, &raw mut ffi::PyDictRevIterValue_Type);
impl_py_type!(DictValues, &raw mut ffi::PyDictValues_Type);
impl_py_type!(EOFError, unsafe {
    ffi::PyExc_EOFError as *mut ffi::PyTypeObject
});
impl_py_type!(Ellipsis, &raw mut ffi::PyEllipsis_Type);
impl_py_type!(EncodingWarning, unsafe {
    ffi::PyExc_EncodingWarning as *mut ffi::PyTypeObject
});
impl_py_type!(Enum, &raw mut ffi::PyEnum_Type);
impl_py_type!(EnvironmentError, unsafe {
    ffi::PyExc_EnvironmentError as *mut ffi::PyTypeObject
});
impl_py_type!(Exception, unsafe {
    ffi::PyExc_Exception as *mut ffi::PyTypeObject
});
impl_py_type!(FileExistsError, unsafe {
    ffi::PyExc_FileExistsError as *mut ffi::PyTypeObject
});
impl_py_type!(FileNotFoundError, unsafe {
    ffi::PyExc_FileNotFoundError as *mut ffi::PyTypeObject
});
impl_py_type!(Filter, &raw mut ffi::PyFilter_Type);
impl_py_type!(Float, &raw mut ffi::PyFloat_Type);
impl_py_type!(FloatingPointError, unsafe {
    ffi::PyExc_FloatingPointError as *mut ffi::PyTypeObject
});
impl_py_type!(Frame, &raw mut ffi::PyFrame_Type);
impl_py_type!(FrozenSet, &raw mut ffi::PyFrozenSet_Type);
impl_py_type!(Function, &raw mut ffi::PyFunction_Type);
impl_py_type!(FutureWarning, unsafe {
    ffi::PyExc_FutureWarning as *mut ffi::PyTypeObject
});
impl_py_type!(Gen, &raw mut ffi::PyGen_Type);
impl_py_type!(GeneratorExit, unsafe {
    ffi::PyExc_GeneratorExit as *mut ffi::PyTypeObject
});
impl_py_type!(GetSetDescr, &raw mut ffi::PyGetSetDescr_Type);
impl_py_type!(IOError, unsafe {
    ffi::PyExc_IOError as *mut ffi::PyTypeObject
});
impl_py_type!(ImportError, unsafe {
    ffi::PyExc_ImportError as *mut ffi::PyTypeObject
});
impl_py_type!(ImportWarning, unsafe {
    ffi::PyExc_ImportWarning as *mut ffi::PyTypeObject
});
impl_py_type!(IndentationError, unsafe {
    ffi::PyExc_IndentationError as *mut ffi::PyTypeObject
});
impl_py_type!(IndexError, unsafe {
    ffi::PyExc_IndexError as *mut ffi::PyTypeObject
});
impl_py_type!(InterruptedError, unsafe {
    ffi::PyExc_InterruptedError as *mut ffi::PyTypeObject
});
impl_py_type!(IsADirectoryError, unsafe {
    ffi::PyExc_IsADirectoryError as *mut ffi::PyTypeObject
});
impl_py_type!(KeyError, unsafe {
    ffi::PyExc_KeyError as *mut ffi::PyTypeObject
});
impl_py_type!(KeyboardInterrupt, unsafe {
    ffi::PyExc_KeyboardInterrupt as *mut ffi::PyTypeObject
});
impl_py_type!(List, &raw mut ffi::PyList_Type);
impl_py_type!(ListIter, &raw mut ffi::PyListIter_Type);
impl_py_type!(ListRevIter, &raw mut ffi::PyListRevIter_Type);
impl_py_type!(Long, &raw mut ffi::PyLong_Type);
impl_py_type!(LongRangeIter, &raw mut ffi::PyLongRangeIter_Type);
impl_py_type!(LookupError, unsafe {
    ffi::PyExc_LookupError as *mut ffi::PyTypeObject
});
impl_py_type!(ManagedBuffer, &raw mut ffi::_PyManagedBuffer_Type);
impl_py_type!(Map, &raw mut ffi::PyMap_Type);
impl_py_type!(MemberDescr, &raw mut ffi::PyMemberDescr_Type);
impl_py_type!(MemoryError, unsafe {
    ffi::PyExc_MemoryError as *mut ffi::PyTypeObject
});
impl_py_type!(MemoryView, &raw mut ffi::PyMemoryView_Type);
impl_py_type!(MethodDescr, &raw mut ffi::PyMethodDescr_Type);
impl_py_type!(MethodWrapper, &raw mut ffi::_PyMethodWrapper_Type);
impl_py_type!(Module, &raw mut ffi::PyModule_Type);
impl_py_type!(ModuleDef, &raw mut ffi::PyModuleDef_Type);
impl_py_type!(ModuleNotFoundError, unsafe {
    ffi::PyExc_ModuleNotFoundError as *mut ffi::PyTypeObject
});
impl_py_type!(NameError, unsafe {
    ffi::PyExc_NameError as *mut ffi::PyTypeObject
});
impl_py_type!(None, &raw mut ffi::_PyNone_Type);
impl_py_type!(NotADirectoryError, unsafe {
    ffi::PyExc_NotADirectoryError as *mut ffi::PyTypeObject
});
impl_py_type!(NotImplemented, &raw mut ffi::_PyNotImplemented_Type);
impl_py_type!(NotImplementedError, unsafe {
    ffi::PyExc_NotImplementedError as *mut ffi::PyTypeObject
});
impl_py_type!(OSError, unsafe {
    ffi::PyExc_OSError as *mut ffi::PyTypeObject
});
impl_py_type!(Object, &raw mut ffi::PyBaseObject_Type);
impl_py_type!(OverflowError, unsafe {
    ffi::PyExc_OverflowError as *mut ffi::PyTypeObject
});
impl_py_type!(PendingDeprecationWarning, unsafe {
    ffi::PyExc_PendingDeprecationWarning as *mut ffi::PyTypeObject
});
impl_py_type!(PermissionError, unsafe {
    ffi::PyExc_PermissionError as *mut ffi::PyTypeObject
});
impl_py_type!(ProcessLookupError, unsafe {
    ffi::PyExc_ProcessLookupError as *mut ffi::PyTypeObject
});
impl_py_type!(Property, &raw mut ffi::PyProperty_Type);
impl_py_type!(Range, &raw mut ffi::PyRange_Type);
impl_py_type!(RangeIter, &raw mut ffi::PyRangeIter_Type);
impl_py_type!(RecursionError, unsafe {
    ffi::PyExc_RecursionError as *mut ffi::PyTypeObject
});
impl_py_type!(RecursionErrorInst, unsafe {
    ffi::PyExc_RecursionErrorInst as *mut ffi::PyTypeObject
});
impl_py_type!(ReferenceError, unsafe {
    ffi::PyExc_ReferenceError as *mut ffi::PyTypeObject
});
impl_py_type!(ResourceWarning, unsafe {
    ffi::PyExc_ResourceWarning as *mut ffi::PyTypeObject
});
impl_py_type!(Reversed, &raw mut ffi::PyReversed_Type);
impl_py_type!(RuntimeError, unsafe {
    ffi::PyExc_RuntimeError as *mut ffi::PyTypeObject
});
impl_py_type!(RuntimeWarning, unsafe {
    ffi::PyExc_RuntimeWarning as *mut ffi::PyTypeObject
});
impl_py_type!(SeqIter, &raw mut ffi::PySeqIter_Type);
impl_py_type!(Set, &raw mut ffi::PySet_Type);
impl_py_type!(SetIter, &raw mut ffi::PySetIter_Type);
impl_py_type!(Slice, &raw mut ffi::PySlice_Type);
impl_py_type!(StopAsyncIteration, unsafe {
    ffi::PyExc_StopAsyncIteration as *mut ffi::PyTypeObject
});
impl_py_type!(StopIteration, unsafe {
    ffi::PyExc_StopIteration as *mut ffi::PyTypeObject
});
impl_py_type!(Super, &raw mut ffi::PySuper_Type);
impl_py_type!(SyntaxError, unsafe {
    ffi::PyExc_SyntaxError as *mut ffi::PyTypeObject
});
impl_py_type!(SyntaxWarning, unsafe {
    ffi::PyExc_SyntaxWarning as *mut ffi::PyTypeObject
});
impl_py_type!(SystemError, unsafe {
    ffi::PyExc_SystemError as *mut ffi::PyTypeObject
});
impl_py_type!(SystemExit, unsafe {
    ffi::PyExc_SystemExit as *mut ffi::PyTypeObject
});
impl_py_type!(TabError, unsafe {
    ffi::PyExc_TabError as *mut ffi::PyTypeObject
});
impl_py_type!(TimeoutError, unsafe {
    ffi::PyExc_TimeoutError as *mut ffi::PyTypeObject
});
impl_py_type!(TraceBack, &raw mut ffi::PyTraceBack_Type);
impl_py_type!(Tuple, &raw mut ffi::PyTuple_Type);
impl_py_type!(TupleIter, &raw mut ffi::PyTupleIter_Type);
impl_py_type!(Type, &raw mut ffi::PyType_Type, ffi::PyTypeObject);
impl_py_type!(TypeError, unsafe {
    ffi::PyExc_TypeError as *mut ffi::PyTypeObject
});
impl_py_type!(UnboundLocalError, unsafe {
    ffi::PyExc_UnboundLocalError as *mut ffi::PyTypeObject
});
impl_py_type!(Unicode, &raw mut ffi::PyUnicode_Type);
impl_py_type!(UnicodeDecodeError, unsafe {
    ffi::PyExc_UnicodeDecodeError as *mut ffi::PyTypeObject
});
impl_py_type!(UnicodeEncodeError, unsafe {
    ffi::PyExc_UnicodeEncodeError as *mut ffi::PyTypeObject
});
impl_py_type!(UnicodeError, unsafe {
    ffi::PyExc_UnicodeError as *mut ffi::PyTypeObject
});
impl_py_type!(UnicodeIter, &raw mut ffi::PyUnicodeIter_Type);
impl_py_type!(UnicodeTranslateError, unsafe {
    ffi::PyExc_UnicodeTranslateError as *mut ffi::PyTypeObject
});
impl_py_type!(UnicodeWarning, unsafe {
    ffi::PyExc_UnicodeWarning as *mut ffi::PyTypeObject
});
impl_py_type!(UserWarning, unsafe {
    ffi::PyExc_UserWarning as *mut ffi::PyTypeObject
});
impl_py_type!(ValueError, unsafe {
    ffi::PyExc_ValueError as *mut ffi::PyTypeObject
});
impl_py_type!(Warning, unsafe {
    ffi::PyExc_Warning as *mut ffi::PyTypeObject
});
impl_py_type!(
    WeakrefCallableProxy,
    &raw mut ffi::_PyWeakref_CallableProxyType
);
impl_py_type!(WeakrefProxy, &raw mut ffi::_PyWeakref_ProxyType);
impl_py_type!(WeakrefRef, &raw mut ffi::_PyWeakref_RefType);
impl_py_type!(WrapperDescr, &raw mut ffi::PyWrapperDescr_Type);
impl_py_type!(ZeroDivisionError, unsafe {
    ffi::PyExc_ZeroDivisionError as *mut ffi::PyTypeObject
});
impl_py_type!(Zip, &raw mut ffi::PyZip_Type);
