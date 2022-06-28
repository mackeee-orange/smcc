use std::fs::File;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;

fn main() -> anyhow::Result<()> {
    let mut f = File::create("buffer.txt")?;
    let thread_labels = ["A", "B", "C", "D"];

    let pool = ThreadPool::new(&thread_labels);

    pool.execute(|| {
        insert_counter_result(f);
    });

    Ok(())
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: String,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: &str, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().expect("Jobの受け取りに失敗");

                println!("{id} is available now");
                job.call_box();
            }
        });

        Worker {
            id,
            thread
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    fn new(thread_labels: &[&str]) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(thread_labels.len());

        for id in thread_labels {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

async fn insert_counter_result(file: File) {

}