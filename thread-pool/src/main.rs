use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("worker {} got a job; executing.", id);
                        job();
                    }
                    Err(_) => {
                        println!("worker {} shutting down.", id);
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::SyncSender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool must have at least 1 worker");

        let (sender, receiver) = mpsc::sync_channel(size * 2);

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

fn main() {
    println!("Creating thread pool with 4 workers");

    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("job {}: starting", i);

            thread::sleep(Duration::from_millis(200));

            println!("job {}: finished", i);
        });
    }

    println!("All jobs submitted. Pool will shut down when scope ends.");
}