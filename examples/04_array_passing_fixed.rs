use std::{
    ops::ControlFlow,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
    thread,
};

const ARRAY_SIZE: usize = 1_000;
const ITERATIONS: usize = 250_000;

fn main() {
    // Thread A creates the array, fills it with data, and stores it here.
    // Thread B loops until this is non-null, then processes contents of array.
    static ARRAY_PTR: AtomicPtr<[u8; ARRAY_SIZE]> = AtomicPtr::new(ptr::null_mut());

    for i in 0..ITERATIONS {
        let thread_b = thread::spawn(|| {
            // We start thread A here, to ensure that thread B starts first.
            // This increases the probability of seeing the desired effects.
            let _thread_a = thread::spawn(|| {
                // The array starts with 0x00 values in every slot.
                let mut data = Box::new([0u8; ARRAY_SIZE]);

                // Then we fill it with 0x01 in every slot.
                for i in 0..ARRAY_SIZE {
                    data[i] = 1;
                }

                // Then we publish this array for the thread B to verify its contents.
                ARRAY_PTR.store(Box::into_raw(data), Ordering::Release);
            });

            let mut ptr: *mut [u8; ARRAY_SIZE];

            // Wait for array pointer to be set by thread A.
            loop {
                ptr = ARRAY_PTR.load(Ordering::Acquire);

                if !ptr.is_null() {
                    break;
                }
            }

            let array: &[u8; ARRAY_SIZE] = unsafe { &*ptr };

            // If every field is 0x01, the sum will be ARRAY_SIZE.
            let sum: usize = array.iter().map(|b| *b as usize).sum();

            if sum != ARRAY_SIZE {
                println!("Observed anomalous sum: {} x 1 == {}", ARRAY_SIZE, sum);
                return ControlFlow::Break(());
            } else {
                // Sum is correct. Try again.
                // We leak the memory to ensure each iteration uses a different memory address.
                ARRAY_PTR.store(ptr::null_mut(), Ordering::Release);
                return ControlFlow::Continue(());
            }
        });

        match thread_b.join().unwrap() {
            ControlFlow::Break(()) => {
                println!("Anomaly was detected on iteration {}/{ITERATIONS}.", i + 1);
                return;
            }
            ControlFlow::Continue(()) => {}
        }
    }

    println!("No anomalies observed after {} iterations.", ITERATIONS);
}
