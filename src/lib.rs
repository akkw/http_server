use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}


impl Drop for ThreadPool {


    fn drop(&mut self) {
        for _  in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        for worker in &mut self.workers {
            println!("Shuttind down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(4);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn executor<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let f = Box::new(f);
        self.sender.send(Message::NewJob(f)).unwrap()
    }
}


type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, job: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = job.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("worker!.....");
                        job()
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate
}
