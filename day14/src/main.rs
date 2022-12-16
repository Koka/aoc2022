use anyhow::{anyhow, Error as AnyhowError};
use std::{cmp, collections::HashSet, error::Error, fs};

fn parse_points(input: String) -> Vec<(usize, usize)> {
    let rock_traces = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .filter_map(|p| {
                    p.split_once(",")
                        .map(|(x, y)| -> Result<(usize, usize), AnyhowError> {
                            let x: usize = x.parse()?;
                            let y: usize = y.parse()?;

                            Ok((x, y))
                        })
                        .transpose()
                        .ok()
                })
                .filter_map(|o| o)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let points: Vec<(usize, usize)> = rock_traces
        .into_iter()
        .flat_map(|path| {
            path.windows(2)
                .flat_map(|w| {
                    let start = w[0];
                    let finish = w[1];
                    let horizontal = start.1 == finish.1;

                    if horizontal {
                        let min_x = cmp::min(start.0, finish.0);
                        let max_x = cmp::max(start.0, finish.0);

                        (min_x..=max_x).map(|x| (x, start.1)).collect::<Vec<_>>()
                    } else {
                        let min_y = cmp::min(start.1, finish.1);
                        let max_y = cmp::max(start.1, finish.1);

                        (min_y..=max_y).map(|y| (start.0, y)).collect::<Vec<_>>()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    points
}

fn print_points(points: &Vec<(usize, usize)>, sands: &Vec<(usize, usize)>) {
    let mut min_x = 500;
    let mut min_y = 0;
    let mut max_x = 500;
    let mut max_y = 0;
    for p in points.iter().chain(sands.iter()) {
        if p.0 < min_x {
            min_x = p.0;
        }
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 < min_y {
            min_y = p.1;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    }

    let w = max_x - min_x + 1;
    let h = max_y - min_y + 1;
    let mut field = vec![vec!['.'; w]; h];

    for p in points {
        field[p.1 - min_y][p.0 - min_x] = '#';
    }

    for p in sands {
        field[p.1 - min_y][p.0 - min_x] = '~';
    }

    field[0 - min_y][500 - min_x] = '+';

    let field = field
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    dbg!(min_x, min_y, max_x, max_y, field);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut rock_points = parse_points(input);
    print_points(&rock_points, &vec![]);

    let mut rock_set: HashSet<(usize, usize)> = HashSet::from_iter(rock_points.iter().cloned());

    let mut sand_rested: usize = 0;
    let mut i = 0;

    let max_rock_y = rock_points
        .iter()
        .max_by(|p1, p2| p1.1.cmp(&p2.1))
        .map(|p| p.1)
        .ok_or(anyhow!("No floor!"))?;

    let mut sands: Vec<(usize, usize)> = vec![(500, 0)];

    'outer: loop {
        let mut survived: Vec<(usize, usize)> = vec![];

        for sand in &mut sands {
            let whats_next = [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ]
            .into_iter()
            .filter(|p| !rock_set.contains(p) && p.1 < (max_rock_y + 2))
            .take(1)
            .collect::<Vec<_>>();

            let candidate = whats_next.first();

            if let Some(next_sand) = &candidate {
                survived.push(**next_sand);
            } else {
                rock_points.push(sand.clone());
                rock_set.insert(sand.clone());
                sand_rested += 1;
            }

            if rock_points.contains(&(500, 0)) {
                break 'outer;
            }
        }

        sands = survived;
        sands.push((500, 0));

        if i % 1000 == 0 {
            println!("Iteration {}, sand rested {}", i, sand_rested);
        }

        if i % 10_000 == 0 {
            print_points(&rock_points, &sands);
        }

        i += 1;
    }

    println!("Total iterations: {}", i);
    println!("Sand units rested: {}", sand_rested);

    print_points(&rock_points, &sands);

    Ok(())
}
