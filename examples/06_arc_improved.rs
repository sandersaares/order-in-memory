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
        // SAFETY: The state was created in Arc::new and is valid as long as
        // there is at least one Arc instance pointing to it. We never create
        // &mut exclusive references to it so it is always valid to create a
        // & shared reference to it.
        let state = unsafe { self.ptr.as_ref() };

        // When a clone of the Arc is dropped, the reference count decrement
        // is performed with Release ordering to signal that any writes into
        // T on this thread must be visible before the decrement becomes
        // visible on other threads.
        if state.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            // The previous value was 1 so now we are at 0 references remaining.

            // When a clone of the Arc is dropped and we have verified that the
            // reference count became zero, we apply a fence with Acquire
            // ordering to ensure that we see all changes that happened on
            // other threads before they decremented their own reference count
            // (i.e. we are dropping the final version of the value of type T).
            fence(Ordering::Acquire);

            // SAFETY: This pointer was allocated by Box::into_raw in Arc::new
            // and we have verified that the reference count is now zero, so
            // it is safe to deallocate it here - nobody else can use it again.
            // We no longer reference the "state" variable after this point,
            // ensuring that this function does not attempt to reference this data.
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T: Send> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // SAFETY: The state was created in Arc::new and is valid as long as
        // there is at least one Arc instance pointing to it. We never create
        // &mut exclusive references to it so it is always valid to create a
        // & shared reference to it.
        let state = unsafe { self.ptr.as_ref() };

        // The data dependency between the reference count and the inner value of
        // type T only exists when dropping the object of type T, so we do not
        // need to define any ordering constraints when incrementing the reference
        // count because an increment cannot result in a drop of the inner value.
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
