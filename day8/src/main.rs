use std::{error::Error, fs};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn ray_trace(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    let h = matrix.len();
    let w = matrix[0].len();

    let mut score = 1u32;

    for dir in Direction::iter() {
        let mut next_i = i;
        let mut next_j = j;

        let mut seen_trees = 0;

        while next_i > 0 && next_j > 0 && next_i < w - 1 && next_j < h - 1 {
            match dir {
                Direction::Top => {
                    next_j = next_j - 1;
                }
                Direction::Bottom => {
                    next_j = next_j + 1;
                }
                Direction::Left => {
                    next_i = next_i - 1;
                }
                Direction::Right => {
                    next_i = next_i + 1;
                }
            }

            let next_height = matrix[next_j][next_i];

            seen_trees += 1;

            if next_height >= matrix[j][i] {
                break;
            }
        }

        if seen_trees > 0 {
            score = score * seen_trees;
        }
    }

    score
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let matrix = input
        .lines()
        .map(|s| {
            let a = s.chars().filter_map(|c| c.to_string().parse::<u8>().ok());
            a.collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let h = matrix.len();
    let w = matrix[0].len();

    dbg!(w, h);

    let zeroes = vec![0u32; w];
    let mut vmatrix = vec![zeroes; h];

    let mut max_score = 0u32;
    for j in 0..h {
        for i in 0..w {
            let score = ray_trace(&matrix, i, j);

            if score > 0 {
                if score > max_score {
                    max_score = score
                }
                vmatrix[j][i] = score;
            }
        }
    }

    let s = vmatrix
        .into_iter()
        .map(|r| r.into_iter().map(|v| v.to_string()).collect::<String>())
        .collect::<Vec<_>>();

    dbg!(s);

    println!("Max score: {}", max_score);

    Ok(())
}
