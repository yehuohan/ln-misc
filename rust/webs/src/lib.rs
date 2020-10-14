//! Web请求线程池库

use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>, // Worker用于传递闭包给线程
    sender: mpsc::Sender<Message>, // 向Worker发送闭包
}

impl ThreadPool {
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
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
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
            // join需要thread的所有权，
            // 但是Worker拥用对thread的所有权，且这里只有Worker的可变借用，
            // 所以通过Option.take，将thread的所有权移出Worker实例（留下None在Worker中）
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            // while表达式，使得Mutex在整个while块都持有锁，
            // 这会导致其它线程获取不到锁
            //while let Ok(job) = receiver.lock().unwrap().recv() { job(); }
            loop {
                // 这里的Worker，只有一个处理recv阻塞调用，其余的在等待Mutex锁，
                // 在recv调用过程中，Mutex持有锁，
                // 在将recv的闭包绑定到job变量后，Mutex的锁释放了
                //
                // 代码：
                // let job = receiver.lock().unwrap().recv().unwrap();
                // println!("Worker {} got a job...", id);
                // job();
                //
                // 等同于以下代码：
                // let job;
                // {
                //      let r = receiver.lock().unwrap();
                //      job = r.recv().unwrap();
                // }

                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job...", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} terminated", id);
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
