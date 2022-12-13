use anyhow::anyhow;
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let stream = input.lines().next().ok_or(anyhow!("No input!"))?[..].as_bytes();

    let window_size = 14;

    let mut last = None;
    for (i, window) in stream.windows(window_size).enumerate() {
        let set = window.iter().collect::<HashSet<_>>();
        if set.len() == window_size {
            last = Some(i + window_size);
            break;
        }
    }

    println!("Index: {:?}", last);

    Ok(())
}
