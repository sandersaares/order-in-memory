use std::{hint::black_box, thread};

const INCREMENT_COUNT: u64 = 1_000_000;

fn main() {
    static mut COUNTER: u64 = 0;

    let thread_a = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            _ = black_box(unsafe {
                COUNTER += 1;
            });
        }
    });

    let thread_b = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            _ = black_box(unsafe {
                COUNTER += 1;
            });
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    assert_eq!(unsafe { COUNTER }, INCREMENT_COUNT * 2);
    println!("All increments were correctly applied.");
}
