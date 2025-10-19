use std::{
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

const INCREMENT_COUNT: u64 = 1_000_000;

fn main() {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let thread_a = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            _ = black_box(COUNTER.fetch_add(1, Ordering::Relaxed));
        }
    });

    let thread_b = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            _ = black_box(COUNTER.fetch_add(1, Ordering::Relaxed));
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    assert_eq!(COUNTER.load(Ordering::Relaxed), INCREMENT_COUNT * 2);
    println!("All increments were correctly applied.");
}
