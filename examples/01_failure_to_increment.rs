use std::thread;

const INCREMENT_COUNT: u64 = 1_000_000;

fn main() {
    static mut COUNTER: u64 = 0;

    let thread_a = thread::spawn(|| {
        for _ in 0..INCREMENT_COUNT {
            unsafe {
                COUNTER += 1;
            }
        }
    });

    let thread_b = thread::spawn(|| {
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
