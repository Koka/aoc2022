use regex::Regex;
use std::{cmp, error::Error, fs, ops::RangeInclusive};
use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance_to(&self, tgt: &Point) -> usize {
        self.x.abs_diff(tgt.x) + self.y.abs_diff(tgt.y)
    }
}

#[derive(Debug)]
struct Sensor(Point);

#[derive(Debug)]
struct Beacon(Point);

impl Beacon {
    fn frequency(&self) -> isize {
        self.0.x * 4000000 + self.0.y
    }
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: usize,
}

fn scan_y(inspected_y: isize, circles: &Vec<Circle>) -> Vec<RangeInclusive<isize>> {
    let mut intersections = circles
        .iter()
        .filter(|c| c.center.y.abs_diff(inspected_y) <= c.radius)
        .map(|c| {
            let height = c.center.y.abs_diff(inspected_y);
            let width_at_height = c.radius - height;
            RangeInclusive::new(
                c.center.x - width_at_height as isize,
                c.center.x + width_at_height as isize,
            )
        })
        .collect::<Vec<_>>();

    intersections.sort_by(|a, b| a.start().cmp(b.start()));

    let mut prev_len = intersections.len();
    loop {
        let mut reduced: Vec<RangeInclusive<isize>> = vec![];
        for wnd in intersections.windows(2) {
            let a = wnd[0].clone();
            let b = wnd[1].clone();
            if a.contains(b.start()) && a.contains(b.end()) {
                reduced.push(a);
            } else if b.contains(a.start()) && b.contains(a.end()) {
                reduced.push(b);
            } else if a.contains(b.start()) || b.contains(a.start()) {
                reduced.push(RangeInclusive::new(
                    cmp::min(*a.start(), *b.start()),
                    cmp::max(*a.end(), *b.end()),
                ))
            }
        }

        if reduced.len() == 0 {
            break;
        }

        intersections = reduced;
        if prev_len == intersections.len() {
            break;
        }
        prev_len = intersections.len();
    }

    intersections
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let regexp =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let circles = input
        .lines()
        .filter_map(|s| {
            regexp.captures(s).map(|cap| -> Option<(Sensor, Beacon)> {
                Some((
                    Sensor(Point {
                        x: cap.get(1)?.as_str().parse().ok()?,
                        y: cap.get(2)?.as_str().parse().ok()?,
                    }),
                    Beacon(Point {
                        x: cap.get(3)?.as_str().parse().ok()?,
                        y: cap.get(4)?.as_str().parse().ok()?,
                    }),
                ))
            })
        })
        .filter(|opt| opt.is_some())
        .filter_map(|step| step)
        .map(|(s, b)| Circle {
            center: s.0,
            radius: s.0.distance_to(&b.0),
        })
        .collect::<Vec<_>>();

    let inspected_y: isize = 2_000_000;
    let intersections = scan_y(inspected_y, &circles);
    dbg!(inspected_y, &intersections);

    println!(
        "Impossible position count at {}: {}",
        inspected_y,
        intersections
            .into_iter()
            .map(|r| r.end() - r.start())
            .sum::<isize>(),
    );

    let min_x: isize = 0;
    let min_y: isize = 0;
    let max_x: isize = 4000000;
    let max_y: isize = 4000000;

    let mut beacon: Option<Beacon> = None;
    for inspected_y in min_y..max_y {
        let intersections = scan_y(inspected_y, &circles)
            .into_iter()
            .map(|r| RangeInclusive::new(cmp::max(*r.start(), min_x), cmp::min(*r.end(), max_x)))
            .filter(|r| r.end() - r.start() > 0 && !(*r.start() == min_x && *r.end() == max_y))
            .collect::<Vec<_>>();

        if intersections.len() > 0 {
            dbg!(inspected_y, &intersections);
        }

        if intersections.len() == 2 {
            beacon = Some(Beacon(Point {
                x: intersections[0].end() + 1,
                y: inspected_y,
            }));
            break;
        }
    }

    let beacon = beacon.ok_or(anyhow!("No beacon!"))?;

    dbg!(&beacon, &beacon.frequency());

    Ok(())
}
