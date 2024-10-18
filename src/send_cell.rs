use core::{
    mem::{self, ManuallyDrop},
    sync::atomic::{AtomicUsize, Ordering},
};
use std::num::NonZero;

static MAX_THREAD_ID: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static THREAD_ID: NonZero<usize> = {
        let thread_id = MAX_THREAD_ID.fetch_add(1, Ordering::Relaxed);
        NonZero::new(thread_id).expect("thread ID overflow")
    }
}

fn current_thread() -> NonZero<usize> {
    THREAD_ID.with(|id| *id)
}

pub struct SendCell<T> {
    inner: ManuallyDrop<T>,
    thread: NonZero<usize>,
}

unsafe impl<T> Send for SendCell<T> {}

impl<T> SendCell<T> {
    pub fn new(value: T) -> Self {
        SendCell {
            inner: ManuallyDrop::new(value),
            thread: current_thread(),
        }
    }

    pub fn try_into_inner(self) -> Result<T, ()> {
        if current_thread() == self.thread {
            let mut send_cell = ManuallyDrop::new(self);
            Ok(unsafe { ManuallyDrop::take(&mut send_cell.inner) })
        } else {
            Err(())
        }
    }

    pub fn into_inner(self) -> T {
        self.try_into_inner()
            .expect("into_inner called from non-owner thread")
    }

    pub fn try_get(&self) -> Result<&T, ()> {
        if current_thread() == self.thread {
            Ok(&*self.inner)
        } else {
            Err(())
        }
    }

    pub fn get(&self) -> &T {
        self.try_get().expect("get called from non-owner thread")
    }
}

impl<T> Drop for SendCell<T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            if current_thread() == self.thread {
                unsafe { ManuallyDrop::drop(&mut self.inner) };
            } else {
                panic!("SendCell dropped from non-owner thread");
            }
        }
    }
}
