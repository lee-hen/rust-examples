use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>); // take ownership of the closure and move the closure out of the Box<T>
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
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
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // A slow request will still cause other requests to wait to be processed
        // The reason is somewhat subtle: the Mutex struct has no public unlock method
        // because the ownership of the lock is based on the lifetime of the MutexGuard<T>
        // within the LockResult<MutexGuard<T>> that the lock method returns.
        // At compile time, the borrow checker can then enforce the rule that a resource guarded
        // by a Mutex cannot be accessed unless we hold the lock.
        // But this implementation can also result in the lock being held longer than intended if we don’t
        // think carefully about the lifetime of the MutexGuard<T>.
        // Because the values in the while expression remain in scope for the duration of the block, the lock
        // remains held for the duration of the call to job.call_box(), meaning other workers cannot receive jobs.

        // let thread = thread::spawn(move || {
        //     while let Ok(job) = receiver.lock().unwrap().recv() {
        //         println!("Worker {} got a job; executing.", id);
        //         job.call_box();
        //     }
        // });

        // By using loop instead and acquiring the lock and a job within the block
        // rather than outside it, the MutexGuard returned from the lock method is
        // dropped as soon as the let job statement ends.
        // This ensures that the lock is held during the call to recv,
        // but it is released before the call to job.call_box(),
        // allowing multiple requests to be serviced concurrently.

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
