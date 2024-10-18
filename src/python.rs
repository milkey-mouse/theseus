#![deny(unused_results)]
use std::{
    cell::Cell,
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    panic,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
};

mod exception;
mod object;
mod object_impl;
mod reference;

pub use exception::*; // TODO
pub use object::*; // TODO
pub use reference::*; // TODO

pub use pyo3_ffi as ffi;

use crate::send_cell::SendCell;

pub enum Status {
    Exception(Cell<Option<SendCell<ffi::PyStatus>>>),
}

impl Status {
    pub fn catch_unwind<R, F: FnOnce() -> R + panic::UnwindSafe>(f: F) -> R {
        match panic::catch_unwind(f) {
            Ok(value) => value,
            Err(err) => {
                if let Some(Status::Exception(exception)) = err.downcast_ref::<Status>() {
                    unsafe { ffi::Py_ExitStatusException(exception.take().unwrap().into_inner()) }
                } else {
                    panic::resume_unwind(err)
                }
            }
        }
    }

    pub fn handle(status: ffi::PyStatus) {
        if unsafe { ffi::PyStatus_Exception(status) } != 0 {
            panic::panic_any(Status::Exception(Cell::new(Some(SendCell::new(status)))))
        }
    }
}

pub struct PreConfig(ffi::PyPreConfig);

impl PreConfig {
    pub fn python() -> Self {
        unsafe {
            let mut pre_config = MaybeUninit::uninit();
            ffi::PyPreConfig_InitPythonConfig(pre_config.as_mut_ptr());
            Self(pre_config.assume_init())
        }
    }

    pub fn isolated() -> Self {
        unsafe {
            let mut pre_config = MaybeUninit::uninit();
            ffi::PyPreConfig_InitIsolatedConfig(pre_config.as_mut_ptr());
            Self(pre_config.assume_init())
        }
    }
}

impl Deref for PreConfig {
    type Target = ffi::PyPreConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PreConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Config(ffi::PyConfig);

impl Config {
    pub fn python() -> Self {
        unsafe {
            let mut config = MaybeUninit::uninit();
            ffi::PyConfig_InitPythonConfig(config.as_mut_ptr());
            Self(config.assume_init())
        }
    }

    pub fn isolated() -> Self {
        unsafe {
            let mut config = MaybeUninit::uninit();
            ffi::PyConfig_InitIsolatedConfig(config.as_mut_ptr());
            Self(config.assume_init())
        }
    }
}

impl Deref for Config {
    type Target = ffi::PyConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Config {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { ffi::PyConfig_Clear(&raw mut **self) };
    }
}

static INTERPRETER_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Interpreter(PhantomData<*mut ()>); // make !Send + !Sync

impl Interpreter {
    pub fn pre_initialize(pre_config: &PreConfig) {
        if INTERPRETER_INITIALIZED.load(Ordering::Acquire) {
            panic!("called Interpreter::pre_initialize after initialization");
        }

        Status::handle(unsafe { ffi::Py_PreInitialize(&**pre_config) })
    }

    pub fn initialize(config: &Config) -> Self {
        if INTERPRETER_INITIALIZED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            panic!("called Interpreter::initialize more than once");
        }

        Status::handle(unsafe { ffi::Py_InitializeFromConfig(&**config) });

        Self(PhantomData)
    }

    pub fn get() -> Self {
        if !INTERPRETER_INITIALIZED.load(Ordering::Acquire) {
            panic!("called Interpreter::get before initialization");
        }

        Self(PhantomData)
    }
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        if INTERPRETER_INITIALIZED
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            panic!("called Interpreter::drop before initialization");
        }

        if unsafe { ffi::Py_FinalizeEx() } != 0 {
            panic!("Py_FinalizeEx failed");
        }
    }
}
