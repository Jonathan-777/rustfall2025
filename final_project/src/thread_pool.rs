/// Custom thread pool implementation using only std::thread and std::sync
/// 
/// - Uses Arc<Mutex<>> for shared state
/// - Uses Condvar for efficient worker wake-up
/// - Supports configurable number of workers
/// - Handles graceful shutdown
/// - Implements proper task distribution

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;

/// A task that can be executed by a worker thread
pub type Task = Box<dyn FnOnce() + Send + 'static>;

/// Message passed between main thread and workers
enum Message {
    NewTask(Task),
    Terminate,
}

/// A worker thread in the pool
struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Spawn a new worker thread
    fn new(receiver: Arc<Mutex<Vec<Message>>>, condvar: Arc<Condvar>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let mut queue = receiver.lock().unwrap();
                
                // Wait until there's a message available
                while queue.is_empty() {
                    queue = condvar.wait(queue).unwrap();
                }
                
                // Dequeue a message (from the front for FIFO behavior)
                let message = if !queue.is_empty() {
                    queue.remove(0) // Get from front instead of pop() from back
                } else {
                    drop(queue); // Release lock if queue is empty
                    continue; // Loop back to check again
                };
                
                drop(queue); // Release lock before executing task
                
                match message {
                    Message::NewTask(task) => {
                        task(); // Execute the task
                    }
                    Message::Terminate => {
                        break; // Exit the worker loop
                    }
                }
            }
        });
        
        Worker {
            thread: Some(thread),
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join(); // Wait for thread to finish
        }
    }
}

/// A thread pool that manages a pool of worker threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Arc<Mutex<Vec<Message>>>,
    condvar: Arc<Condvar>,
}

impl ThreadPool {
    /// Create a new thread pool with the specified number of workers
    /// 
    /// # Panics
    /// Panics if size is 0
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Thread pool size must be greater than 0");
        
        let sender = Arc::new(Mutex::new(Vec::new()));
        let condvar = Arc::new(Condvar::new());
        let mut workers = Vec::with_capacity(size);
        
        for _id in 0..size {
            workers.push(Worker::new(Arc::clone(&sender), Arc::clone(&condvar)));
        }
        
        ThreadPool {
            workers,
            sender,
            condvar,
        }
    }
    
    /// Submit a task to the thread pool for execution
    /// 
    /// The task will be executed by one of the available worker threads.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut queue = self.sender.lock().unwrap();
        queue.push(Message::NewTask(Box::new(f)));
        drop(queue);
        
        // Wake up all waiting worker threads so one can pick up the task
        self.condvar.notify_all();
    }
    
    /// Get the number of workers in this pool
    pub fn num_workers(&self) -> usize {
        self.workers.len()
    }
    
    /// Gracefully shutdown the thread pool
    /// 
    /// This will wait for all pending tasks to complete and then shut down all workers.
    pub fn shutdown(mut self) {
        let num_workers = self.workers.len();
        
        // Send terminate message for each worker
        {
            let mut queue = self.sender.lock().unwrap();
            for _ in 0..num_workers {
                queue.push(Message::Terminate);
            }
        }
        
        // Wake all workers so they can process the terminate messages
        self.condvar.notify_all();
        
        // Join all workers (this happens automatically in Drop, but we can be explicit)
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let num_workers = self.workers.len();
        
        // Send terminate messages
        {
            let mut queue = self.sender.lock().unwrap();
            for _ in 0..num_workers {
                queue.push(Message::Terminate);
            }
        }
        
        // Wake all workers to process terminate messages
        self.condvar.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.num_workers(), 4);
    }
    
    #[test]
    fn test_thread_pool_execution() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));
        
        for _ in 0..5 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
        
        // Give tasks time to complete
        thread::sleep(std::time::Duration::from_millis(100));
        
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
    
    #[test]
    #[should_panic]
    fn test_thread_pool_zero_size() {
        ThreadPool::new(0);
    }
}
