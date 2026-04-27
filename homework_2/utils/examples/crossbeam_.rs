use std::{sync::atomic::AtomicBool, thread, time::Duration};

use crossbeam::{atomic::AtomicCell, sync::Parker, utils::Backoff};

fn main() {
    let a = AtomicCell::new(42usize);
    assert!(AtomicCell::<usize>::is_lock_free());
    let s = AtomicCell::new("hello".to_string());
    assert!(!AtomicCell::<String>::is_lock_free());
    a.fetch_add(s.take().len() as _); // as _ -> usize
    println!("{}", a.take());

    // INFO: parker is used to park the main thread until the other thread finishes
    let parker = Parker::new();
    let unparker = parker.unparker().clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("other thread finished");
        unparker.unpark();
    });

    parker.park();
    println!("main thread finished")
}

/// Spins until the given atomic boolean is set to true.
#[allow(dead_code)]
fn spin_wait(ready: &AtomicBool) {
    let backoff = Backoff::new();
    while !ready.load(std::sync::atomic::Ordering::SeqCst) {
        backoff.snooze();
    }
}
