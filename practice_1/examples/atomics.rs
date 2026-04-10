use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = counter.clone(); // the same as Arc::new(&counter)

    let join_handle = std::thread::spawn(move || {
        for _ in 0..1_000_000 {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        }
    });

    for _ in 0..1_000_000 {
        counter.fetch_sub(1, Ordering::Relaxed);
    }

    join_handle.join().unwrap();
    println!("Counter: {}", counter.load(Ordering::Relaxed));
}
