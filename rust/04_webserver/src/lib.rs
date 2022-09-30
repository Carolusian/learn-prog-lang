use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // init all workers of the thread pool
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(_f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Once a worker is initiated, loop and wait for jobs to be passed as messages
        let thread = thread::spawn(move || loop {
            // How Mutual Exclusion works:
            // job function is passed as a message
            // one message is only acquired and locked by only one thread at any given time
            let message = receiver.lock().unwrap().recv();

            // process the job
            match message {
                Ok(job) => {
                    println!("Worker {0} is busy..", id);
                    job();
                    println!("Worker {0} is idle..", id);
                }
                Err(_) => {
                    break;
                }
            }
        });
        println!("Worker {0} is running..", id);
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
