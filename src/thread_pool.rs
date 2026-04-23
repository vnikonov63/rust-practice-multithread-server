use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + 'static + Send>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        // TODO: Understand difference between sender and reciever
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let job = Box::new(f);
        let _ = self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {};", worker.id);

            // join takes ownership of its argument.
            // So when thread is owned by the worker, we cannot call the join method.
            // Takeaway: Ask a question who is the owner, constantly.

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: this may panic in production, thread::Builder should be used
        let thread = thread::spawn(move || {
            loop {
                let message = reciever.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing...");
                        job();
                    }
                    Err(err) => {
                        println!("worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}