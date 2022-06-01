use std::fs;
use std::path::PathBuf;
use anyhow;

pub fn tree(entry: PathBuf, current_rank: u32) -> anyhow::Result<()> {
    let mut v = Vec::with_capacity(current_rank as usize);
    for _ in 0..current_rank {
        v.push("    ");
    }
    println!("{}|- {}", v.join(""), entry.clone().into_os_string().into_string().unwrap());

    if entry.is_dir() {
        for node in entry.read_dir()?.into_iter() {
            tree(node?.path(), current_rank + 1)?;
        }
    }

    Ok(())
}