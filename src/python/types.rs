#![deny(unused_results)]

use std::cell::UnsafeCell;

use super::{ffi, PyObjectWrapper, PyTypeInfo};
use crate::impl_pyobject_wrapper;

pub struct Object(UnsafeCell<ffi::PyObject>);

unsafe impl PyObjectWrapper for Object {
    type Inner = ffi::PyObject;
}

unsafe impl PyTypeInfo for Object {
    const NAME: &'static str = "Object";

    fn type_obj() -> BorrowedRef<Type> {
        unsafe { BorrowedRef::from(NonNull::new_unchecked(ffi::PyBaseObject_Type)) }
    }

    fn check(obj: &Object) -> bool {
        true // TODO
    }
}

// cargo doc && find target/doc/pyo3_ffi -type f -name '*.html' -exec ./parse_items.py {} + | ./stratify.py

impl_pyobject_wrapper!(ASCIIObject, Object, ffi::PyASCIIObject);
impl_pyobject_wrapper!(
    CompactUnicodeObject,
    ASCIIObject,
    ffi::PyCompactUnicodeObject
);
impl_pyobject_wrapper!(
    UnicodeObject,
    CompactUnicodeObject,
    ffi::PyUnicodeObject,
    &raw mut ffi::PyUnicode_Type,
    ffi::PyUnicode_Check
);

impl_pyobject_wrapper!(AsyncGenObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(BaseExceptionObject, Object, ffi::PyBaseExceptionObject);
impl_pyobject_wrapper!(
    ArithmeticErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    AssertionErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    AttributeErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    BaseExceptionGroupObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    BlockingIOErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    BrokenPipeErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    BufferErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    BytesWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ChildProcessErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ConnectionAbortedErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ConnectionErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ConnectionRefusedErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ConnectionResetErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    DeprecationWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    EOFErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    EncodingWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    EnvironmentErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ExceptionObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    FileExistsErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    FileNotFoundErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    FloatingPointErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    FutureWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    GeneratorExitObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    IOErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ImportWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    IndentationErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    IndexErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    InterruptedErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    IsADirectoryErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    KeyErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    KeyboardInterruptObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    LookupErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    MemoryErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ModuleNotFoundErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    NameErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    NotADirectoryErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    NotImplementedErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    OverflowErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    PendingDeprecationWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    PermissionErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ProcessLookupErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    RecursionErrorInstObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    RecursionErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ReferenceErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ResourceWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    RuntimeErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    RuntimeWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    StopAsyncIterationObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    SyntaxWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    SystemErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    TabErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    TimeoutErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    TypeErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    UnboundLocalErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    UnicodeDecodeErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    UnicodeEncodeErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    UnicodeTranslateErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    UnicodeWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    UserWarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ValueErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    WarningObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);
impl_pyobject_wrapper!(
    ZeroDivisionErrorObject,
    BaseExceptionObject,
    ffi::PyBaseExceptionObject
);

impl_pyobject_wrapper!(BaseObjectObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    BoolObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyBool_Type,
    ffi::PyBool_Check
);

impl_pyobject_wrapper!(ByteArrayIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(BytesIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    CFunctionObject,
    Object,
    ffi::PyCFunctionObject,
    &raw mut ffi::PyCFunction_Type,
    ffi::PyCFunction_Check
);
impl_pyobject_wrapper!(
    CMethodObject,
    CFunctionObject,
    ffi::PyCMethodObject,
    &raw mut ffi::PyCMethod_Type,
    ffi::PyCMethod_Check
);

impl_pyobject_wrapper!(
    CallIterObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyCallIter_Type,
    ffi::PyCallIter_Check
);

impl_pyobject_wrapper!(CapsuleObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(ClassMethodDescrObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    ComplexObject,
    Object,
    ffi::PyComplexObject,
    &raw mut ffi::PyComplex_Type,
    ffi::PyComplex_Check
);

impl_pyobject_wrapper!(ContextObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(ContextTokenObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(ContextVarObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(CoroObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(DateTime_Date, Object, ffi::PyDateTime_Date);

impl_pyobject_wrapper!(DateTime_DateTime, Object, ffi::PyDateTime_DateTime);

impl_pyobject_wrapper!(DateTime_Delta, Object, ffi::PyDateTime_Delta);

impl_pyobject_wrapper!(DateTime_Time, Object, ffi::PyDateTime_Time);

impl_pyobject_wrapper!(DescrObject, Object, ffi::PyDescrObject);
impl_pyobject_wrapper!(GetSetDescrObject, DescrObject, ffi::PyGetSetDescrObject);
impl_pyobject_wrapper!(MemberDescrObject, DescrObject, ffi::PyMemberDescrObject);
impl_pyobject_wrapper!(MethodDescrObject, DescrObject, ffi::PyMethodDescrObject);
impl_pyobject_wrapper!(WrapperDescrObject, DescrObject, ffi::PyWrapperDescrObject);

impl_pyobject_wrapper!(
    DictItemsObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyDictItems_Type,
    ffi::PyDictItems_Check
);

impl_pyobject_wrapper!(DictIterItemObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(DictIterKeyObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(DictIterValueObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    DictKeysObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyDictKeys_Type,
    ffi::PyDictKeys_Check
);

impl_pyobject_wrapper!(
    DictObject,
    Object,
    ffi::PyDictObject,
    &raw mut ffi::PyDict_Type,
    ffi::PyDict_Check
);

impl_pyobject_wrapper!(DictProxyObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(DictRevIterItemObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(DictRevIterKeyObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(DictRevIterValueObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    DictValuesObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyDictValues_Type,
    ffi::PyDictValues_Check
);

impl_pyobject_wrapper!(EllipsisObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(EnumObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(FilterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    FloatObject,
    Object,
    ffi::PyFloatObject,
    &raw mut ffi::PyFloat_Type,
    ffi::PyFloat_Check
);

impl_pyobject_wrapper!(
    FrameObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyFrame_Type,
    ffi::PyFrame_Check
);

impl_pyobject_wrapper!(FrozenSetObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    FunctionObject,
    Object,
    ffi::PyFunctionObject,
    &raw mut ffi::PyFunction_Type,
    ffi::PyFunction_Check
);

impl_pyobject_wrapper!(
    GenObject,
    Object,
    ffi::PyGenObject,
    &raw mut ffi::PyGen_Type,
    ffi::PyGen_Check
);

impl_pyobject_wrapper!(ImportErrorObject, Object, ffi::PyImportErrorObject);

impl_pyobject_wrapper!(ListIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(ListRevIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    LongObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyLong_Type,
    ffi::PyLong_Check
);

impl_pyobject_wrapper!(LongRangeIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(MapObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    MemoryViewObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyMemoryView_Type,
    ffi::PyMemoryView_Check
);

impl_pyobject_wrapper!(ModuleDefObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(ModuleDef_Base, Object, ffi::PyModuleDef_Base);
impl_pyobject_wrapper!(ModuleDef, ModuleDef_Base, ffi::PyModuleDef);

impl_pyobject_wrapper!(
    ModuleObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyModule_Type,
    ffi::PyModule_Check
);

impl_pyobject_wrapper!(OSErrorObject, Object, ffi::PyOSErrorObject);

impl_pyobject_wrapper!(PropertyObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(RangeIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    RangeObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyRange_Type,
    ffi::PyRange_Check
);

impl_pyobject_wrapper!(ReversedObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    SeqIterObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PySeqIter_Type,
    ffi::PySeqIter_Check
);

impl_pyobject_wrapper!(SetIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(SetObject, Object, ffi::PySetObject);

impl_pyobject_wrapper!(
    SliceObject,
    Object,
    ffi::PySliceObject,
    &raw mut ffi::PySlice_Type,
    ffi::PySlice_Check
);

impl_pyobject_wrapper!(StopIterationObject, Object, ffi::PyStopIterationObject);

impl_pyobject_wrapper!(SuperObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(SyntaxErrorObject, Object, ffi::PySyntaxErrorObject);

impl_pyobject_wrapper!(SystemExitObject, Object, ffi::PySystemExitObject);

impl_pyobject_wrapper!(
    TraceBackObject,
    Object,
    ffi::PyObject,
    &raw mut ffi::PyTraceBack_Type,
    ffi::PyTraceBack_Check
);

impl_pyobject_wrapper!(TupleIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(UnicodeErrorObject, Object, ffi::PyUnicodeErrorObject);

impl_pyobject_wrapper!(UnicodeIterObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(VarObject, Object, ffi::PyVarObject);
impl_pyobject_wrapper!(
    ByteArrayObject,
    VarObject,
    ffi::PyByteArrayObject,
    &raw mut ffi::PyByteArray_Type,
    ffi::PyByteArray_Check
);
impl_pyobject_wrapper!(
    BytesObject,
    VarObject,
    ffi::PyBytesObject,
    &raw mut ffi::PyBytes_Type,
    ffi::PyBytes_Check
);
impl_pyobject_wrapper!(
    CodeObject,
    VarObject,
    ffi::PyCodeObject,
    &raw mut ffi::PyCode_Type,
    ffi::PyCode_Check
);
impl_pyobject_wrapper!(
    ListObject,
    VarObject,
    ffi::PyListObject,
    &raw mut ffi::PyList_Type,
    ffi::PyList_Check
);
impl_pyobject_wrapper!(
    TupleObject,
    VarObject,
    ffi::PyTupleObject,
    &raw mut ffi::PyTuple_Type,
    ffi::PyTuple_Check
);
impl_pyobject_wrapper!(
    TypeObject,
    VarObject,
    ffi::PyTypeObject,
    &raw mut ffi::PyType_Type,
    ffi::PyType_Check
);
impl_pyobject_wrapper!(HeapTypeObject, TypeObject, ffi::PyHeapTypeObject);

impl_pyobject_wrapper!(ZipObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(_CoroWrapperObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(
    _DateTime_BaseDateTime,
    Object,
    ffi::_PyDateTime_BaseDateTime
);

impl_pyobject_wrapper!(_DateTime_BaseTime, Object, ffi::_PyDateTime_BaseTime);

impl_pyobject_wrapper!(_ManagedBufferObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(_MethodWrapperObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(_NoneObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(_NotImplementedObject, Object, ffi::PyObject);

impl_pyobject_wrapper!(_WeakReference, Object, ffi::_PyWeakReference);
