use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    sender: Sender<Job>,
}
struct Worker;
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        thread::spawn(move || loop {
            // todo 一旦lock 和recv拆开 就会经常是一个核心运作
            // let message = receiver.lock().unwrap().recv();

            // match message {
            //     Ok(job) => {
            //         println!("Worker {id} got a job; executing.");

            //         job();
            //     }
            //     Err(_) => {
            //         println!("Worker {id} disconnected; shutting down.");
            //         break;
            //     }
            // }


            let mutex_guard_result = receiver.lock();
            if let Ok(mutex_guard) = mutex_guard_result{
                let message = mutex_guard.recv();
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
            }
        });

        Worker
    }
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        println!("{}", size);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.send(Box::new(f));
    }
}
