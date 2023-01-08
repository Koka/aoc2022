use anyhow::{anyhow, bail, Error as AnyhowError};
use itertools::Itertools;
use std::{collections::HashSet, fs, io};

static ROCKS: &[&[&[u8]]] = &[
    &[&[1, 1, 1, 1]],
    &[&[0, 1, 0], &[1, 1, 1], &[0, 1, 0]],
    &[&[0, 0, 1], &[0, 0, 1], &[1, 1, 1]],
    &[&[1], &[1], &[1], &[1]],
    &[&[1, 1], &[1, 1]],
];

fn print_field(
    field: &Vec<Vec<u8>>,
    rock_idx: usize,
    height: usize,
    left: usize,
    stop_after: bool,
) -> Result<(), AnyhowError> {
    let mut field = field.clone();

    let rock = ROCKS[rock_idx];

    for j in 0..rock.len() {
        for i in 0..rock[j].len() {
            if field[height + j][left + i] == 0 {
                field[height + j][left + i] = 2 * rock[rock.len() - j - 1][i];
            }
        }
    }

    let real_len = field.len();

    field.reverse();
    field.truncate(30);
    field.reverse();

    println!();
    for i in 0..field.len() {
        println!(
            "|{}|",
            field[field.len() - i - 1]
                .iter()
                .map(|&v| match v {
                    0 => '.',
                    1 => '#',
                    2 => '@',
                    _ => ' ',
                })
                .collect::<String>()
        );
    }
    if real_len > 20 {
        println!("~~~~~~~~~");
    } else {
        println!("+-------+");
    }
    println!();

    if stop_after {
        dbg!(rock_idx, height, real_len);
        io::stdin().read_line(&mut "".to_owned())?;
    }

    Ok(())
}

fn will_collide_at(rock_idx: usize, left: &usize, height: &usize, field: &Vec<Vec<u8>>) -> bool {
    let rock = ROCKS[rock_idx];

    let mut collided = false;

    'outer: for j in 0..rock.len() {
        for i in 0..rock[j].len() {
            if field[*height + j][*left + i] != 0 && rock[rock.len() - j - 1][i] != 0 {
                collided = true;
                break 'outer;
            }
        }
    }

    collided
}

fn move_rock(
    rock_idx: usize,
    jets: &mut dyn Iterator<Item = &char>,
    left: &mut usize,
    height: &mut usize,
    field: &Vec<Vec<u8>>,
    bottom: usize,
    debug: bool,
) -> Result<bool, AnyhowError> {
    let jet = jets.next().ok_or(anyhow!("No jet!"))?;
    let rock = ROCKS[rock_idx];

    match jet {
        '>' => {
            if (*left + rock[0].len() < 7)
                && !will_collide_at(rock_idx, &(*left + 1), height, field)
            {
                *left += 1;
            }
        }
        '<' => {
            if *left > 0 && !will_collide_at(rock_idx, &(*left - 1), height, field) {
                *left -= 1;
            }
        }
        v => bail!("Unknown jet direction {}", v),
    };

    let will_collide_down = *height == 0
        || (*height <= bottom && will_collide_at(rock_idx, left, &(*height - 1), field));

    if debug {
        println!("Move {}", jet);
        print_field(field, rock_idx, *height, *left, true)?;
    }

    if !will_collide_down {
        *height -= 1;

        if debug {
            println!("Move down");
            print_field(field, rock_idx, *height, *left, true)?;
        }
    }

    Ok(will_collide_down)
}

fn stop_rock(
    rock_idx: usize,
    left: &mut usize,
    height: &mut usize,
    field: &mut Vec<Vec<u8>>,
) -> usize {
    let rock = ROCKS[rock_idx];

    for j in 0..rock.len() {
        for i in 0..rock[j].len() {
            field[*height + j][*left + i] =
                field[*height + j][*left + i] | rock[rock.len() - j - 1][i];
        }
    }

    rock.len()
}

fn rock_simulator(stone_count: usize, debug: bool) -> Result<Vec<usize>, AnyhowError> {
    let gas_jets = fs::read_to_string("./input.txt")?
        .lines()
        .take(1)
        .map(|a| Some(a.to_owned()))
        .collect::<Option<String>>()
        .ok_or(anyhow!("No input!"))?
        .chars()
        .collect::<Vec<_>>();

    let mut additions = vec![];

    let mut field: Vec<Vec<u8>> = vec![vec![0; 7]; 4];

    let mut rock_idx: usize = 0;
    let mut bottom = 0;
    let mut height = 3;
    let mut left = 2;
    let mut jets = gas_jets.iter().cycle();

    let mut rocks_stopped = 0;

    if debug {
        print_field(&field, rock_idx, height, left, true)?;
    }

    while rocks_stopped < stone_count {
        loop {
            let will_collide = move_rock(
                rock_idx,
                &mut jets,
                &mut left,
                &mut height,
                &field,
                bottom,
                debug,
            )?;

            if will_collide {
                break;
            }
        }

        let height_added = stop_rock(rock_idx, &mut left, &mut height, &mut field);
        rocks_stopped += 1;

        rock_idx += 1;
        if rock_idx > ROCKS.len() - 1 {
            rock_idx = 0;
        }

        let new_bottom = height + height_added;

        if new_bottom > bottom {
            additions.push(new_bottom - bottom);
            bottom = new_bottom;
        } else {
            additions.push(0);
        }

        height = 3 + bottom;
        left = 2;

        let rock = ROCKS[rock_idx];
        if field.len() < height + rock.len() {
            for _ in 0..(height + rock.len() - field.len()) {
                field.push(vec![0; 7]);
            }
        }

        if debug {
            print_field(&field, rock_idx, height, left, true)?;
        }
    }

    println!("Rocks stopped: {}", rocks_stopped);
    println!("Tower height: {}", bottom);

    dbg!(additions.len(), additions.iter().sum::<usize>());

    Ok(additions)
}

fn main() -> Result<(), AnyhowError> {
    let stone_count = 10_000;

    let v = rock_simulator(stone_count, false)?;

    let mut period = None;
    'outer: for start in (v.len() / 2)..v.len() {
        let orig = v[start..].to_vec();

        for i in 1..(v.len() - start) {
            let mut rot = orig.clone();
            rot.rotate_left(i);

            if orig.eq(&rot) {
                period = Some((start, i));
                break 'outer;
            }
        }
    }

    if let Some((start, period)) = period {
        dbg!(start, period);

        let prefix_height: usize = v[0..start].iter().sum();

        let cycle_body = v[start..start + period].to_owned();
        let height_per_cycle = cycle_body.iter().sum::<usize>();

        let target_stones: usize = 1000000000000;
        let stones_to_compute = target_stones - start;

        let cycle_count: usize = stones_to_compute / period;
        let cycle_total_height = cycle_count * height_per_cycle;
        let target_height = prefix_height + cycle_total_height;

        let stones_remaining = stones_to_compute - cycle_count * period;
        let partial_cycle_height: usize = cycle_body[0..stones_remaining].into_iter().sum();

        dbg!(
            target_stones,
            cycle_count,
            height_per_cycle,
            target_height,
            stones_remaining,
            partial_cycle_height,
        );

        println!("Target height: {}", target_height + partial_cycle_height);
    } else {
        println!("No period!");
    }

    Ok(())
}
