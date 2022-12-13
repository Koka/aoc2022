use itertools::Itertools;
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let chunks = input
        .lines()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .chunks(3);

    let result: u16 = chunks
        .into_iter()
        .filter_map(|chunk| chunk.reduce(|acc, it| acc.intersection(&it).map(|c| *c).collect()))
        .filter_map(|isec| isec.iter().next().and_then(|c| Some(*c)))
        .map(|c| {
            let ascii_code = c as u8;
            let the_code = if ascii_code > 96 {
                ascii_code - 96
            } else {
                ascii_code - 64 + 26
            };
            the_code as u16
        }).sum();

    println!("Sum: {}", result);

    Ok(())
}
