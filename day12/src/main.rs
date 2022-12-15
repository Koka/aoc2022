use anyhow::anyhow;
use pathfinding::prelude::astar;
use std::{error::Error, fs};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(usize, usize);

impl Point {
    fn can_go_to(&self, tgt: &Point, map: &Vec<Vec<u8>>) -> bool {
        let src_h = map[self.1][self.0];
        let tgt_h = map[tgt.1][tgt.0];

        tgt_h <= src_h + 1
    }

    fn distance(&self, other: &Point) -> usize {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
}

fn print_map(map: &Vec<Vec<u8>>, start: &Point, finish: &Point, path: &Vec<Point>) {
    let mut display = map
        .into_iter()
        .map(|r| r.into_iter().map(|h| (h + 97) as char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for p in path {
        display[p.1][p.0] = '.';
    }

    display[start.1][start.0] = 'S';
    display[finish.1][finish.0] = 'E';

    let display = display
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    dbg!(display);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut start = Point(0, 0);
    let mut finish = Point(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(j, s)| {
            s.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    'S' => {
                        start = Point(i, j);
                        0
                    }
                    'E' => {
                        finish = Point(i, j);
                        26
                    }
                    c => (c as u8) - 97,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let h = map.len();
    let w = map[0].len();

    dbg!(w, h, start, finish);
    print_map(&map, &start, &finish, &vec![]);

    let mut possible_starts: Vec<Point> = vec![];
    for (i, r) in map.iter().enumerate() {
        for (j, p) in r.iter().enumerate() {
            if *p == 0 {
                possible_starts.push(Point(j, i));
            }
        }
    }

    dbg!(possible_starts.len());

    let mut min_len: usize = 9999999999999;
    let mut min_path: Option<Vec<Point>> = None;
    let mut min_start: Point = start;

    for candidate in possible_starts {
        let presult = astar(
            &candidate,
            |p| {
                vec![
                    if p.0 > 0 {
                        let n = Point(p.0 - 1, p.1);
                        if p.can_go_to(&n, &map) {
                            Some((n, 1))
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    if p.1 > 0 {
                        let n = Point(p.0, p.1 - 1);
                        if p.can_go_to(&n, &map) {
                            Some((n, 1))
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    if p.1 < h - 1 {
                        let n = Point(p.0, p.1 + 1);
                        if p.can_go_to(&n, &map) {
                            Some((n, 1))
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    if p.0 < w - 1 {
                        let n = Point(p.0 + 1, p.1);
                        if p.can_go_to(&n, &map) {
                            Some((n, 1))
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                ]
                .into_iter()
                .filter_map(|p| p)
                .collect::<Vec<_>>()
            },
            |p| p.distance(&finish),
            |p| *p == finish,
        );

        if let Some((path, plen)) = presult {
            if plen < min_len {
                min_path = Some(path);
                min_len = plen;
                min_start = candidate;
            }
        }
    }

    print_map(
        &map,
        &min_start,
        &finish,
        &min_path.ok_or(anyhow!("No path found!"))?,
    );

    dbg!(min_start, min_len);

    Ok(())
}
