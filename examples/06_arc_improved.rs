use std::{
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering, fence},
    thread,
};

struct SharedState<T: Send> {
    value: T,
    ref_count: AtomicUsize,
}

struct Arc<T: Send> {
    ptr: NonNull<SharedState<T>>,
}

impl<T: Send> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T: Send> Arc<T> {
    fn new(value: T) -> Self {
        let state = Box::new(SharedState {
            value,
            ref_count: AtomicUsize::new(1),
        });

        Arc {
            ptr: NonNull::new(Box::into_raw(state)).unwrap(),
        }
    }
}

impl<T: Send> Drop for Arc<T> {
    fn drop(&mut self) {
        let state = unsafe { self.ptr.as_ref() };

        if state.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            // The previous value was 1 so now we are at 0 references remaining.

            // Ensure we see the final writes to `value` before dropping.
            fence(Ordering::Acquire);

            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T: Send> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let state = unsafe { self.ptr.as_ref() };

        state.ref_count.fetch_add(1, Ordering::Relaxed);

        Self {
            ptr: self.ptr.clone(),
        }
    }
}

unsafe impl<T: Send> Send for Arc<T> {}

fn main() {
    let x = Arc::new(42);

    println!("Value: {:?}", *x);

    let thread_two = thread::spawn({
        let x = x.clone();

        move || {
            println!("Thread Two Value: {}", *x);
        }
    });

    let thread_three = thread::spawn({
        let x = x.clone();

        move || {
            println!("Thread Three Value: {}", *x);
        }
    });

    thread_two.join().unwrap();
    thread_three.join().unwrap();
}
