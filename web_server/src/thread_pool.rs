use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Each Worker will hold on to the receiving side of the channel.
// We’ll create a new Job struct that will hold the closures we want to send down the channel.
// The execute method will send the job it wants to execute down the sending side of the channel.
// In its thread, the Worker will loop over its receiving side of the channel and execute the closures of any jobs it receives.

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

        let (sender, receiver): (mpsc::Sender<Job>, mpsc::Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&receiver)))
        }
        Self { threads, sender }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            // Concurrency note: This next line uses an interesting trick of Rust.
            // Since the receiver.lock() is not stored to LHS of the expression, 
            // it gets dropped immediately after the line executes.
            // This means that no matter how long the rest of the loop (executing job())
            // takes, the lock will be released already to allow other workers to get
            // new jobs.
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Self { id, handle }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
