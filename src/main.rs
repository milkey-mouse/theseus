#![deny(unsafe_op_in_unsafe_fn)]

use theseus::python::{ffi, Config, Interpreter, Module, PreConfig, Status, Unicode};

fn main() {
    Status::catch_unwind(_main)
}

fn _main() {
    let mut pre_config = PreConfig::isolated();
    pre_config.utf8_mode = 1;
    Interpreter::pre_initialize(&pre_config);

    let interpreter = Interpreter::initialize(&Config::isolated());

    Module::import(&Unicode::new(b"PIL"));

    unsafe { ffi::PyRun_SimpleString(c"print('bruh')".as_ptr()) };
}
