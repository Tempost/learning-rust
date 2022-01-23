use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// creation of enums so that we can process new jobs and shutdown cleanly
enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            // Time to handle our messages from the receiver
            match message {
                Message::NewJob(job)  => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }

        });

        // wrapping our thread in Some() to conform to our struct definition
        Worker { id, thread: Some(thread)}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,   
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // checking that the user has supplied a valid number of threads to the function
        // if that is not the case we will panic!
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // worker(threads) creation to store in our ThreadPool
            workers.push(Worker::new(id, Arc::clone(&receiver))); 
        }
        ThreadPool { workers, sender } 
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // create a new job with our closure information inside
        let job = Box::new(f);

        // wrap our job in the NewJob enum and send it to the thread
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// "graceful" clean up of all threads, might sit and wait until working threads have finished their
// job
impl Drop for ThreadPool {
     fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // Here we are destructure the some and get access to the thread
            // in the case that a workers thread is None, then nothing will need to be done
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        // NOTE, old notes on code
        // unless our thread is of the Option type we will see a compiler error here
        // this is becuase join takes ownership of the argument, we want join to consume the
        // thread 
    }
}
