use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let mut file_w = File::create("buffer.txt")?;
    // let file_w = OpenOptions::new().write(true).truncate(true).open("buffer.txt")?;

    let (sender, receiver) = mpsc::channel();

    let sender1 = mpsc::Sender::clone(&sender);
    // thread::spawn(move || {
    //     let id = "A";
    //     loop {
    //         let f = File::open("buffer.txt").expect("ファイル開けません");
    //         let file = BufReader::new(f);
    //         let mut lines: Vec<_> = file.lines().map(|line| { line.unwrap() }).collect();
    //         if let Some(line) = lines.last() {
    //             let num = line.split(": ").last().unwrap().parse::<i32>().unwrap();
    //             if num >= 10 {
    //                 return
    //             }
    //
    //             sender1.send((id.to_string(), num + 1));
    //         }
    //     }
    // });

    let id = "A";
    loop {
        let f = File::open("buffer.txt").expect("ファイル開けません");
        let file = BufReader::new(f);
        let mut lines: Vec<_> = file.lines().map(|line| { line.unwrap() }).collect();
        if let Some(line) = lines.last() {
            let num = line.split(": ").last().unwrap().parse::<i32>().unwrap();
            if num >= 10 {
                bail!("dame");
            }

            sender1.send((id.to_string(), num + 1));
        }
    }

    for (label, val) in receiver {
        println!("{label} {val}");
        writeln!(&mut file_w, "{}", format!("{label}: {val}"));
    }

    Ok(())
}
