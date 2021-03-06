use std::sync::mpsc;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job;

impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push( Worker::new(id, receiver));

        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

impl Worker {
     fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(||{
            receiver;
        });
        Worker { id, thread}
    }
}
