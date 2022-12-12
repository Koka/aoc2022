use itertools::Itertools;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let sums: Vec<_> = input
        .lines()
        .group_by(|s| s.len() > 0)
        .into_iter()
        .map(|(_k, g)| g.filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<_>>())
        .filter(|v| !v.is_empty())
        .map(|v| v.into_iter().sum::<u32>())
        .collect();

    println!(
        "{:?}",
        sums.into_iter()
            .sorted_by(|a, b| Ord::cmp(b, a))
            .take(3)
            .sum::<u32>()
    );

    Ok(())
}
