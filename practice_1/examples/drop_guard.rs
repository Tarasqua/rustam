use std::{
    sync::{Arc, atomic::AtomicI32},
    time::Duration,
};

const TASKS_COUNT: i32 = 5;

fn main() {
    let task_counter = Arc::new(AtomicI32::default());

    let mut handles = Vec::with_capacity(TASKS_COUNT as usize);

    for i in 0..TASKS_COUNT {
        let task_counter = task_counter.clone();
        task_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let handle = std::thread::spawn(move || {
            let _dec = DecrementOnDrop(task_counter.clone());
            std::thread::sleep(Duration::from_millis(100 * i as u64));

            if i == 2 {
                panic!("Task {i} paniced");
            }

            i
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(val) => println!("Task completed: {:?}", val),
            Err(e) => println!("Task error: {:?}", e),
        }
    }

    println!(
        "Task count: {}",
        task_counter.load(std::sync::atomic::Ordering::Relaxed)
    );
}

struct DecrementOnDrop(Arc<AtomicI32>);

impl Drop for DecrementOnDrop {
    fn drop(&mut self) {
        self.0.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}
