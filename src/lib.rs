use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; // type alias for a Job, which is a boxed closure that can be sent across threads

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); // create a new channel for sending jobs
        let receiver = Arc::new(Mutex::new(receiver)); // wrap the receiver in Arc and Mutex locks for shared access
        let mut workers = Vec::with_capacity(size); // pre-allocate space for the workers

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // execute a job by sending it to worker threads
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); // box the job to be sent
        self.sender.as_ref().unwrap().send(job).unwrap(); // send the job to the workers
    }
}

impl Drop for ThreadPool {
    // custom drop implementation to gracefully shut down the threads
    fn drop(&mut self) {
        drop(self.sender.take()); // drop the sender to close the channel

        // join each worker thread to ensure they finish before the thread pool is dropped
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // create a new worker that listens for jobs
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv(); // receive a message (job)

            match message {
                Ok(job) => {
                    print!("Worker {id} got a job, executing: ");
                    job(); // execute the job
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down...");
                    break; // break the loop if the channel is disconnected
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}