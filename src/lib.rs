use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + 'static + Send>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a Thread new pool
    ///
    /// The size is the number of threads in the pool
    ///
    /// ## Panics
    ///
    /// The `new` function will panic if the size is not positive
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // TODO: UNderstand netween what and what is this channe; created?
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(job);
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: this will panic in production, thread::Builder should be used
        let thread = thread::spawn(move || {
            loop {
                let job = reciever.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing");

                job();
            }
        });
        return Worker { id, thread };
    }
}
