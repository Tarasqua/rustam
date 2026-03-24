use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
use tracing::debug;

type Job = Box<dyn FnOnce() + Send + 'static>; // INFO: a trait object that can be called once and is sendable across threads

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        debug!("Worker {id} got a job; executing.");
                        job();
                    }
                    // INFO: on sender drop, recv() will return an error
                    Err(_) => {
                        debug!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Self { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); // INFO: Arc + Mutex to share ownership across multiple threads and allow the threads to mutate the value (one access at a time)

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap(); // INFO: sending the job to the channel, using unwrap is safe here since the channel is never closed
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // INFO: dropping sender closes the channel, which indicates no more messages will be sent

        // INFO: use &mut 'cause self is a mutable reference, and we also need to be able to mutate worker
        // INFO: drain(..) creates an iterator that takes ownership of each element from the vector, but leaves the vector itself empty (.. range syntax will remove every value from the vector)
        for worker in &mut self.workers.drain(..) {
            debug!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
