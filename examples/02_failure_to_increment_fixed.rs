use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

const INCREMENT_COUNT: u64 = 1_000_000;

fn main() {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let thread_a = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    });

    let thread_b = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    println!("Final value: {}", COUNTER.load(Ordering::Relaxed));
}
