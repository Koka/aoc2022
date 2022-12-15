use anyhow::{anyhow, bail, Error as AnyhowError};
use std::{error::Error, fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Cmd {
    Noop,
    Addx(isize),
}

fn cycles(cmd: &Cmd) -> u8 {
    match cmd {
        Cmd::Noop => 1,
        Cmd::Addx(_) => 2,
    }
}

impl FromStr for Cmd {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Cmd::Noop),
            s => {
                let (cmd, arg) = s.split_once(' ').ok_or(anyhow!("Bad command"))?;
                match cmd {
                    "addx" => Ok(Cmd::Addx(arg.parse::<isize>()?)),
                    _ => bail!("Bad syntax"),
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let program = input.lines().filter_map(|l| l.parse::<Cmd>().ok());

    let mut cycle = 1;
    let mut reg_x: isize = 1;

    let mut screen = vec![vec!['.'; 40]; 6];

    let mut total_strength = 0;

    let mut spy = |cycle_num, curr_x| {
        let crt_pos = (cycle_num - 1) % 240;
        let crt_x = crt_pos % 40;
        let crt_y = crt_pos / 40;

        if crt_x >= curr_x - 1 && crt_x <= curr_x + 1 {
            screen[crt_y as usize][crt_x as usize] = '#';
        }

        if (cycle_num == 20 || (cycle_num - 20) % 40 == 0) && (cycle_num <= 220) {
            let signal_strength = cycle_num * curr_x;

            dbg!(cycle_num, signal_strength);

            total_strength += signal_strength;
        }
    };

    for cmd in program.into_iter() {
        for _ in 0..cycles(&cmd) {
            spy(cycle, reg_x);
            cycle += 1;
        }

        match cmd {
            Cmd::Addx(v) => {
                reg_x += v;
            }
            _ => {}
        };
    }
    spy(cycle, reg_x);

    println!("Total strength: {}", total_strength);

    let screen = screen
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    dbg!(screen);

    Ok(())
}
