use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    mem,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, RecvError, Sender},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

// A task is boxed because different closures have different concrete types.
// `Send + 'static` is required because tasks move into worker threads.
type Task = Box<dyn FnOnce() + Send + 'static>;

const CHUNK_SIZE: usize = 100_000;

fn main() {
    let data: Arc<[_]> = (0..100_000_000).rev().collect();
    let to_find = 1_000;

    println!("Testing st_find with {}", to_find);
    let found = measure(|| st_find(&data, to_find));
    println!("Found: {:?}", found);

    thread::sleep(Duration::from_secs(2));

    println!("----------------");
    println!("Testing mt_find with {}", to_find);
    let data_clone = Arc::clone(&data);
    let found = measure(|| mt_find(data_clone, to_find));
    println!("Found: {:?}", found);

    thread::sleep(Duration::from_secs(2));

    println!("----------------");
    println!("Testing mt_pool_find with {}", to_find);
    let pool = ThreadPool::new(40).unwrap();
    let data_clone = Arc::clone(&data);
    let found = measure(|| mt_pool_find(data_clone, to_find, pool));
    println!("Found: {:?}", found);

    println!("----------------");
    println!("Testing mt_rayon_pool_find with {}", to_find);
    let data_clone = Arc::clone(&data);
    let found = measure(|| mt_rayon_pool_find(data_clone, to_find));
    println!("Found: {:?}", found);
}

fn measure<T>(f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now() - start;
    println!("Complete in: {:?}", duration);
    result
}

fn st_find(data: &[i32], val: i32) -> Option<usize> {
    // Straight single-threaded baseline.
    data.iter()
        .enumerate()
        .find(|(_, v)| **v == val)
        .map(|(i, _)| i)
    // Equivalent short form:
    // data.iter().position(|&v| v == val)
}

/// Search by spawning one OS thread per chunk.
///
/// This is educational, but usually too expensive for real workloads because
/// thread creation has a noticeable cost.
fn mt_find(data: Arc<[i32]>, val: i32) -> Option<usize> {
    let chunk_count = data.len().div_ceil(CHUNK_SIZE);
    let (tx, rx) = mpsc::channel();

    for chunk in 0..chunk_count {
        let tx = tx.clone();
        let data = Arc::clone(&data);

        thread::spawn(move || {
            let (chunk_start, chunk_end) = chunk_bounds(data.len(), chunk);
            let data = &data[chunk_start..chunk_end];

            let found = data
                .iter()
                .enumerate()
                .find(|(_, v)| **v == val)
                .map(|(i, _)| chunk_start + i);

            // Always send one reply per worker.
            // If we sent only `Some`, the receiver could block forever when the
            // value does not exist in some or all chunks.
            tx.send(found).unwrap();
        });
    }

    collect_first_match(rx, chunk_count)
}

/// Search using a custom fixed-size thread pool.
///
/// Compared with `mt_find`, this reuses worker threads instead of creating a
/// brand-new OS thread for every chunk.
fn mt_pool_find(data: Arc<[i32]>, val: i32, pool: ThreadPool) -> Option<usize> {
    let chunk_count = data.len().div_ceil(CHUNK_SIZE);
    let (tx, rx) = mpsc::channel();

    for chunk in 0..chunk_count {
        let tx = tx.clone();
        let data = Arc::clone(&data);

        pool.spawn(move || {
            let (chunk_start, chunk_end) = chunk_bounds(data.len(), chunk);
            let data = &data[chunk_start..chunk_end];

            let found = data
                .iter()
                .enumerate()
                .find(|(_, v)| **v == val)
                .map(|(i, _)| chunk_start + i);

            tx.send(found).unwrap();
        });
    }

    collect_first_match(rx, chunk_count)
}

/// Search using Rayon.
///
/// Rayon already owns a work-stealing pool, so this is the most idiomatic
/// parallel version in this file.
fn mt_rayon_pool_find(data: Arc<[i32]>, val: i32) -> Option<usize> {
    let chunk_count = data.len().div_ceil(CHUNK_SIZE);

    (0..chunk_count).into_par_iter().find_map_any(|chunk| {
        let (chunk_start, chunk_end) = chunk_bounds(data.len(), chunk);
        let data = &data[chunk_start..chunk_end];

        data.iter()
            .enumerate()
            .find(|(_, v)| **v == val)
            .map(|(i, _)| chunk_start + i)
    })
}

fn chunk_bounds(len: usize, chunk: usize) -> (usize, usize) {
    let start = chunk * CHUNK_SIZE;
    let end = ((chunk + 1) * CHUNK_SIZE).min(len);
    (start, end)
}

fn collect_first_match(rx: Receiver<Option<usize>>, expected_messages: usize) -> Option<usize> {
    for _ in 0..expected_messages {
        if let Some(found) = rx.recv().unwrap() {
            return Some(found);
        }
    }

    None
}

/// Basic thread pool with a fixed number of worker threads.
#[derive(Debug, Clone)]
struct ThreadPool {
    // `Arc` makes the pool clonable while still sharing the same workers.
    handles: Arc<Vec<JoinHandle<()>>>,
    // `None` during drop means "the pool is shutting down".
    sender: Option<Sender<Task>>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Option<Self> {
        if thread_count == 0 {
            return None;
        }

        let (sender, rx) = mpsc::channel::<Task>();

        // `Receiver<T>` is not `Sync`, so several threads cannot hold `&Receiver<T>`
        // directly. Wrapping it in `Mutex` gives us synchronized shared access.
        let rx = Arc::new(Mutex::new(rx));

        let handles = (0..thread_count)
            .map(|_| {
                let rx = Arc::clone(&rx);

                thread::spawn(move || {
                    // Each worker keeps taking tasks until all senders are dropped.
                    while let Ok(task) = get_task(&rx) {
                        task();
                    }
                })
            })
            .collect();

        Some(Self {
            handles: Arc::new(handles),
            sender: Some(sender),
        })
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

fn get_task(recv: &Mutex<Receiver<Task>>) -> Result<Task, RecvError> {
    // Only one worker may call `recv()` at a time because all workers share the
    // same underlying channel receiver.
    recv.lock().unwrap().recv()
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let handles = mem::take(&mut self.handles);

        // Only the final clone should shut workers down and join them.
        if let Ok(handles) = Arc::try_unwrap(handles) {
            // Dropping the last sender makes every blocked `recv()` return Err,
            // which lets worker threads exit their loop naturally.
            mem::drop(self.sender.take());

            for handle in handles {
                handle.join().unwrap();
            }
        }
    }
}
