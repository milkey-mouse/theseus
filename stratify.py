#!/usr/bin/env python3
import re
import sys
from collections import defaultdict

STRUCT = re.compile(
    r"pub\s+struct\s+(\w+)(?:\s*{\s*(?:[^{}]*?(?:pub\s+\w+\s*:\s*([^,;]+),)?)?[^}]*}|[^;]*;)",
    re.DOTALL,
)

TYPE_OBJECT = re.compile(
    r"pub\s+(?:unsafe\s+)?(?:static|const)\s+(?:mut\s+)?((\w+)_Type)\s*:\s*PyTypeObject;",
)

EXCEPTION = re.compile(
    r"pub\s+(?:unsafe\s+)?(?:static|const)\s+(?:mut\s+)?(PyExc_(\w+))\s*:\s*\*mut\s+PyObject;",
)

CHECK_FN = re.compile(
    r"pub\s+(?:unsafe\s+)?fn\s+((Py\w+)_Check)\(op:\s*\*mut\s*PyObject\)\s*->\s*c_int;"
)


def ffi_to_local_name(name):
    if name.startswith("_Py"):
        return f"_{name[3:]}"
    if name.startswith("Py"):
        return name[2:]
    return name


def main():
    src = sys.stdin.read()

    ffis = {}
    deps = defaultdict(list)
    for struct in STRUCT.finditer(src):
        name, inner = struct.groups()
        if inner:
            ffis[name] = f"ffi::{name}"
            deps[inner].append(name)
        elif name.endswith("Object"):
            ffis[name] = "ffi::PyObject"
            deps["PyObject"].append(name)

    types = {}
    checks = {}

    for type_obj in TYPE_OBJECT.finditer(src):
        name_type, name = type_obj.groups()
        name = f"{name}Object"
        if name not in ffis:
            ffis[name] = "ffi::PyObject"
            deps["PyObject"].append(name)

        types[name] = f"&raw mut ffi::{name_type}"

    for exception in EXCEPTION.finditer(src):
        pyexc_name, name = exception.groups()
        name = f"Py{name}Object"
        if name not in ffis:
            ffis[name] = "ffi::PyBaseExceptionObject"
            deps["PyBaseExceptionObject"].append(name)

        types[name] = f"unsafe {{ ffi::{pyexc_name} as *mut ffi::PyTypeObject }}"

    for check_fn in CHECK_FN.finditer(src):
        py_name_check, name = check_fn.groups()
        checks[f"{name}Object"] = f"ffi::{py_name_check}"

    for dep_list in deps.values():
        dep_list.sort()

    def print_all_deps(name, inner):
        args = [ffi_to_local_name(name), ffi_to_local_name(inner), ffis[name]]
        if (type := types.get(name)) and (check := checks.get(name)):
            args.extend((type, check))

        print(f"impl_pyobject_wrapper!({", ".join(args)});")

        for dep in deps[name]:
            print_all_deps(dep, name)

    roots = deps["PyObject"]

    if roots:
        print_all_deps(roots[0], "PyObject")
        for dep in roots[1:]:
            print()
            print_all_deps(dep, "PyObject")


if __name__ == "__main__":
    main()
