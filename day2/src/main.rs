use anyhow::{anyhow, Error as AnyhowError};
use std::{error::Error, fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(Debug, Clone, Copy)]
enum Figure {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Figure {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Figure::Rock),
            "B" | "Y" => Ok(Figure::Paper),
            "C" | "Z" => Ok(Figure::Scissors),
            s => Err(anyhow!("Invalid figure code {}", s)),
        }
    }
}

impl FromStr for Outcome {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            s => Err(anyhow!("Invalid outcome code {}", s)),
        }
    }
}

fn outcome(my: &Figure, theirs: &Figure) -> Outcome {
    match (my, theirs) {
        (Figure::Rock, Figure::Rock) => Outcome::Draw,
        (Figure::Paper, Figure::Paper) => Outcome::Draw,
        (Figure::Scissors, Figure::Scissors) => Outcome::Draw,

        (Figure::Scissors, Figure::Paper) => Outcome::Win,
        (Figure::Rock, Figure::Scissors) => Outcome::Win,
        (Figure::Paper, Figure::Rock) => Outcome::Win,

        (Figure::Rock, Figure::Paper) => Outcome::Lose,
        (Figure::Paper, Figure::Scissors) => Outcome::Lose,
        (Figure::Scissors, Figure::Rock) => Outcome::Lose,
    }
}

fn inv_outcome(my: &Outcome, theirs: &Figure) -> Figure {
    match (my, theirs) {
        (Outcome::Win, Figure::Rock) => Figure::Paper,
        (Outcome::Win, Figure::Paper) => Figure::Scissors,
        (Outcome::Win, Figure::Scissors) => Figure::Rock,
        (Outcome::Lose, Figure::Rock) => Figure::Scissors,
        (Outcome::Lose, Figure::Paper) => Figure::Rock,
        (Outcome::Lose, Figure::Scissors) => Figure::Paper,
        (Outcome::Draw, fig) => *fig,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let total_score = input
        .lines()
        .filter_map(|s| {
            s.split_once(" ")
                .map(|(l, r)| (l.parse::<Figure>().ok(), r.parse::<Outcome>().ok()))
        })
        .filter_map(|p| match p {
            (None, _) | (_, None) => None,
            (Some(theirs), Some(outcome)) => {
                Some(outcome as u16 + inv_outcome(&outcome, &theirs) as u16)
            }
        })
        .sum::<u16>();

    println!("Score: {}", total_score);

    Ok(())
}
