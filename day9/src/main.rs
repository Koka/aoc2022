use anyhow::{anyhow, bail, Error as AnyhowError};
use std::{cmp, collections::HashSet, error::Error, fs, str::FromStr};

#[derive(Debug)]
enum Move {
    Up(u8),
    Right(u8),
    Down(u8),
    Left(u8),
}

impl FromStr for Move {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_once(' ').ok_or(anyhow!("Bad format"))?;
        let num: u8 = steps.parse()?;

        match dir {
            "U" => Ok(Move::Up(num)),
            "R" => Ok(Move::Right(num)),
            "D" => Ok(Move::Down(num)),
            "L" => Ok(Move::Left(num)),
            s => bail!("Bad move {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn follow(&mut self, tgt: Point) {
        let dist_x = (tgt.x - self.x).abs();
        let dist_y = (tgt.y - self.y).abs();

        let dist = cmp::max(dist_x, dist_y);

        if dist < 2 {
            return;
        }

        if dist_x > 0 {
            if tgt.x > self.x {
                self.x += 1;
            } else if tgt.x < self.x {
                self.x -= 1;
            }
        }

        if dist_y > 0 {
            if tgt.y > self.y {
                self.y += 1;
            } else if tgt.y < self.y {
                self.y -= 1;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let moves = input.lines().filter_map(|s| s.parse::<Move>().ok());

    let mut rope = vec![Point { x: 0, y: 0 }; 10];

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point { x: 0, y: 0 });

    let mut max_x = 0isize;
    let mut max_y = 0isize;
    let mut min_x = 99999isize;
    let mut min_y = 99999isize;

    for mv in moves {
        let mut dx = 0isize;
        let mut dy = 0isize;

        match mv {
            Move::Up(n) => {
                dy = n as isize;
            }
            Move::Down(n) => {
                dy = -(n as isize);
            }
            Move::Left(n) => {
                dx = -(n as isize);
            }
            Move::Right(n) => {
                dx = n as isize;
            }
        }

        let n = cmp::max(dx.abs(), dy.abs());

        for _ in 0..n {
            {
                let head = &mut rope[0];

                if dx.abs() > 0 {
                    head.x = head.x + dx.signum();
                }
                if dy.abs() > 0 {
                    head.y = head.y + dy.signum();
                }
            }

            for i in 1..rope.len() {
                let prev = rope[i - 1];
                rope[i].follow(prev);
            }

            let head = &rope[0];
            let tail = &rope[rope.len() - 1];

            visited.insert(*tail);

            let my_max_x = cmp::max(head.x, tail.x);
            let my_max_y = cmp::max(head.y, tail.y);
            let my_min_x = cmp::min(head.x, tail.x);
            let my_min_y = cmp::min(head.y, tail.y);

            if my_max_x > max_x {
                max_x = my_max_x;
            }
            if my_max_y > max_y {
                max_y = my_max_y;
            }
            if my_min_x < min_x {
                min_x = my_min_x;
            }
            if my_min_y < min_y {
                min_y = my_min_y;
            }
        }
    }

    let w = (max_x - min_x) as usize;
    let h = (max_y - min_y) as usize;

    let mut field = vec![vec!['.'; w]; h];
    for p in &visited {
        field[(p.y - min_y) as usize][(p.x - min_x) as usize] = '#';
    }
    let mut field = field
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    field.reverse();

    dbg!(w, h, field);

    println!("Visited positions: {}", visited.len());

    Ok(())
}
