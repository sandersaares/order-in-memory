#![allow(dead_code, unused)]

use std::{ops::Deref, ptr::NonNull};

struct SharedState<T> {
    value: T,
    ref_count: usize,
}

struct Arc<T> {
    ptr: NonNull<SharedState<T>>,
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

fn main() {
    unimplemented!("This example is not meant to be executed - it is for editorial purposes only");
}
