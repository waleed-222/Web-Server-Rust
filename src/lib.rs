use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

/// A thread pool that manages a fixed number of worker threads to execute tasks.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// Type alias for a boxed job that can be executed by a thread.
/// Jobs must be `FnOnce`, `Send`, and `'static`.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new `ThreadPool`.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of worker threads in the pool.
    ///
    /// # Panics
    ///
    /// Panics if `size` is zero.
    ///
    /// # Example
    ///
    /// ```
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
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

    /// Execute a task in the thread pool.
    ///
    /// The function `f` will be sent to an available worker thread for execution.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that implements `FnOnce`, `Send`, and `'static`.
    ///
    /// # Example
    ///
    /// ```
    /// pool.execute(|| {
    ///     println!("Task running in the thread pool");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

/// Internal structure representing a single worker thread in the pool.
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    /// Create a new worker.
    ///
    /// The worker listens for incoming jobs on the shared receiver and executes them.
    ///
    /// # Arguments
    ///
    /// * `id` - An identifier for the worker.
    /// * `receiver` - A shared receiver wrapped in `Arc<Mutex<_>>`.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread }
    }
}

impl Drop for ThreadPool {
    /// Gracefully shut down all workers in the pool.
    ///
    /// This is triggered automatically when the `ThreadPool` goes out of scope.
    fn drop(&mut self) {
        // Drop the sender so that all workers break out of recv()
        drop(self.sender.take());

        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_pool_executes_task_pass() {
        let pool = ThreadPool::new(2);
        let result = Arc::new(Mutex::new(0));

        let result_clone = Arc::clone(&result);
        pool.execute(move || {
            let mut num = result_clone.lock().unwrap();
            *num += 1;
        });

        thread::sleep(Duration::from_millis(100));

        assert_eq!(*result.lock().unwrap(), 1); 
    }

    #[test]
    fn test_thread_pool_executes_task_fail() {
        let pool = ThreadPool::new(2);
        let result = Arc::new(Mutex::new(0));

        let result_clone = Arc::clone(&result);
        pool.execute(move || {
            let mut num = result_clone.lock().unwrap();
            *num += 1;
        });

        thread::sleep(Duration::from_millis(100));

        assert_eq!(*result.lock().unwrap(), 5); 
    }
}
