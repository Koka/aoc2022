use std::{error::Error, fs, ops::RangeInclusive};

fn parse_range(s: &str) -> Option<RangeInclusive<u32>> {
    let (s1, s2) = s.split_once("-")?;

    Some(RangeInclusive::new(
        s1.parse::<u32>().ok()?,
        s2.parse::<u32>().ok()?,
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let ranges = input.lines().filter_map(|s| s.split_once(",")).filter_map(
        |(left, right)| -> Option<(RangeInclusive<u32>, RangeInclusive<u32>)> {
            Some((parse_range(left)?, parse_range(right)?))
        },
    );

    let result = ranges
        .map(|(r1, r2)| {
            let fwd = r1.contains(r2.start()) || r1.contains(r2.end());
            let rev = r2.contains(r1.start()) || r2.contains(r1.end());
            if fwd || rev {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("Count: {}", result);

    Ok(())
}
