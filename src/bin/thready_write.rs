use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::sync::{Arc, mpsc, Mutex};
use std::{mem, thread};
use std::os::unix::fs::FileExt;

fn main() {
    let labels = ["A", "B", "C", "D"];
    let file = Arc::new(Mutex::new(LineWriter::new(File::create("buffer.txt").unwrap())));
    let counter = Arc::new(Mutex::new(0));

    let mut threads = Vec::with_capacity(labels.len());

    for label in labels {
        let counter = Arc::clone(&counter);
        let file = Arc::clone(&file);

        let thread = thread::spawn(move || {
            let mut i = counter.lock().unwrap();
            let mut file = file.lock().unwrap();

            let txt = format!("{label}: {i}\n");
            file.write_all(txt.as_bytes()).unwrap();

            *i += 1;
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
