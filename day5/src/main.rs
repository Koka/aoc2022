use itertools::Itertools;
use regex::Regex;
use std::{error::Error, fs};

#[derive(Debug)]
struct Step {
    count: usize,
    from: usize,
    to: usize,
}

fn borrow_mut_elementwise<'a, T>(v: &'a mut Vec<T>) -> Vec<&'a mut T> {
    let mut result: Vec<&mut T> = Vec::new();
    let mut current: &mut [T];
    let mut rest = &mut v[..];
    while rest.len() > 0 {
        (current, rest) = rest.split_at_mut(1);
        result.push(&mut current[0]);
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut schema = input
        .lines()
        .take_while(|s| s.len() > 0)
        .map(|s| {
            let chunks = s.chars().into_iter().chunks(4);

            chunks
                .into_iter()
                .filter_map(|chunk| chunk.skip(1).next())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    schema.reverse();

    let stack_count = schema[0].len();
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(schema.len() - 1); stack_count];
    for row in schema.into_iter().skip(1) {
        for (i, el) in row.iter().enumerate() {
            if *el != ' ' {
                stacks[i].push(*el);
            }
        }
    }

    let regexp = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

    let program = input
        .lines()
        .skip_while(|s| s.len() > 0)
        .skip(1)
        .filter_map(|s| {
            regexp.captures(s).map(|cap| -> Option<Step> {
                Some(Step {
                    count: cap.get(1)?.as_str().parse::<usize>().ok()?,
                    from: cap.get(2)?.as_str().parse::<usize>().ok()? - 1,
                    to: cap.get(3)?.as_str().parse::<usize>().ok()? - 1,
                })
            })
        })
        .filter(|opt| opt.is_some())
        .filter_map(|step| step);

    let mut mut_stacks = borrow_mut_elementwise(&mut stacks);
    for Step { count, from, to } in program {
        let mut buf = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(c) = mut_stacks[from].pop() {
                buf.push(c);
            }
        }
        buf.reverse();
        mut_stacks[to].append(&mut buf);
    }

    let result = stacks
        .into_iter()
        .filter_map(|v| v.last().map(|c| *c))
        .join("");

    println!("Result: {}", result);

    Ok(())
}
