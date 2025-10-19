#![cfg_attr(not(target_arch = "aarch64"), allow(unreachable_code))]

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

const INCREMENT_COUNT: u64 = 1_000_000;

fn main() {
    #[cfg(not(target_arch = "aarch64"))]
    {
        println!(
            "This example is only supported on ARM processors, as Intel/AMD processors have a stronger memory model that cannot show these effects."
        );
        return;
    }

    static mut COUNTER: u64 = 0;

    static THREAD_A_COMPLETED: AtomicBool = AtomicBool::new(false);

    let thread_a = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            unsafe {
                COUNTER += 1;
            }
        }

        THREAD_A_COMPLETED.store(true, Ordering::Relaxed);
    });

    let thread_b = thread::spawn(|| {
        while !THREAD_A_COMPLETED.load(Ordering::Relaxed) {}

        for _ in 0..INCREMENT_COUNT {
            unsafe {
                COUNTER += 1;
            }
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    println!("Final value: {}", unsafe { COUNTER });
}
