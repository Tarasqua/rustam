use std::{
    sync::{
        Arc, Barrier, Mutex, RwLock,
        atomic::{self, AtomicI32},
        mpsc::{self, Receiver, Sender, SyncSender},
    },
    thread,
};

fn main() {
    barrier_example();
}

fn primitive_arc() {
    let init_cmd = String::from("Show database");
    let cmd_arc = Arc::new(init_cmd);
    let cmd_ref = cmd_arc.clone();
    let thread_ = thread::spawn(move || {
        perform(cmd_ref.as_str());
    });
    thread_.join().unwrap();
}

fn perform(cmd: &str) {
    println!("Performing command: {}", cmd);
}

fn atomics() {
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter.fetch_add(1, atomic::Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.load(atomic::Ordering::SeqCst));
}

fn mutexes() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // std::mem::drop(num); // INFO: unlocks the mutex
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}

fn rwlocks() {
    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let num = counter.read().unwrap(); // INFO: RwLock guarantees multiple simultaneous save read
            println!("{}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn barrier_example() {
    // INFO: Barrier forces to wait for all threads to reach the barrier before continuing execution
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];

    for i in 0..10 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} started A phace", i);

            b.wait(); // INFO: the point of synchronization: the thread will stop here until the other 9 arrive

            println!("Thread {} passed the barrier", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn chans() {
    // mpsc::sync_channel() // INFO: sync with fixed buffer (sender can be blocked in case of overflow)
    // mpsc::channel() // INFO: async (fire-and-forget)

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let (tx, rx): (SyncSender<i32>, Receiver<i32>) = mpsc::sync_channel(1024); // INFO: bound - queue maxlen

    // tx.send(...).unwrap();
    // rx.recv().unwrap();
}
