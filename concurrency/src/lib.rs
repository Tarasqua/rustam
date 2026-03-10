use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

pub fn first_step() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawn thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // INFO: waits for the thread to finish before exiting the main thread
}

pub fn move_closure() {
    let v = vec![1, 2, 3];

    // INFO: the move keyword forces the closure to take ownership of the variables it uses
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

pub fn channels() {
    let (tx, rx) = mpsc::channel(); // INFO: multiple producer, single consumer channel

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // INFO: sends the value through the channel
    });

    let received = rx.recv().unwrap(); // INFO: receives the value from the channel. Blocks the thread until a value is sent through the channel
    // INFO: try_recv() can be used to attempt to receive a value without blocking the thread. It returns a Result that is Ok if a value was received and Err if no value was available.
    println!("Got the value: {received}");

    handle.join().unwrap();

    // ==================================================

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // ==================================================

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn mutexes() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // INFO: locks the mutex and returns a MutexGuard that allows access to the data inside the mutex. The lock is automatically released when the MutexGuard goes out of scope
        *num = 6;
    }

    println!("m = {m:?}");

    // ==================================================

    // INFO: Mutex provides interior mutability, which allows us to mutate data even when there are immutable references to it. This is useful in concurrent programming because it allows us to safely share data between threads without having to worry about data races. However, it also means that we need to be careful when using Mutexes to avoid deadlocks and other syncronization issues.
    let counter = Arc::new(Mutex::new(0)); // INFO: Arc (similar to Rc) is a thread-safe reference-counting pointer that allows multiple threads to access the same data. Mutex is used to ensure that only one thread can access the data at a time
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // INFO: creates a new reference to the same data
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Result: 10
}
