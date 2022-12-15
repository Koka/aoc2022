use anyhow::{anyhow, bail, Error as AnyhowError};
use std::{error::Error, fs, str::Lines};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    factor: Option<u8>,
    test_factor: u8,
    throw_true: usize,
    throw_false: usize,
    times_inspected: usize,
}

impl Monkey {
    fn build<'a>(lines: &mut Lines<'a>) -> Result<Option<Self>, AnyhowError> {
        let next_line = lines.next();
        if next_line.is_none() {
            return Ok(None);
        }

        next_line.ok_or(anyhow!("No header"))?;

        let (_, items_str) = lines
            .next()
            .ok_or(anyhow!("Bad structure"))?
            .split_once(':')
            .ok_or(anyhow!("Bad item list"))?;

        let start_items = items_str
            .trim()
            .split(", ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let (_, op_str) = lines
            .next()
            .ok_or(anyhow!("Bad op"))?
            .split_once("old ")
            .ok_or(anyhow!("Bad op structure"))?;

        let (op, factor_str) = op_str.split_once(' ').ok_or(anyhow!("Bad op structure"))?;

        let (_, test_factor_str) = lines
            .next()
            .ok_or(anyhow!("Bad test"))?
            .split_once("by ")
            .ok_or(anyhow!("Bad test structure"))?;

        let (_, true_monkey) = lines
            .next()
            .ok_or(anyhow!("Bad true monkey"))?
            .split_once("monkey ")
            .ok_or(anyhow!("Bad true monkey structure"))?;

        let (_, false_monkey) = lines
            .next()
            .ok_or(anyhow!("Bad false monkey"))?
            .split_once("monkey ")
            .ok_or(anyhow!("Bad false monkey structure"))?;

        lines.next();

        let op = match op {
            "*" => {
                if factor_str == "old" {
                    Op::Square
                } else {
                    Op::Multiply
                }
            }
            "+" => Op::Add,
            s => bail!("Bad op {}", s),
        };

        Ok(Some(Monkey {
            op,
            items: start_items,
            factor: if op != Op::Square {
                Some(factor_str.parse()?)
            } else {
                None
            },
            test_factor: test_factor_str.parse()?,
            throw_false: false_monkey.parse()?,
            throw_true: true_monkey.parse()?,
            times_inspected: 0,
        }))
    }

    fn inspect_items(&mut self, modulo: usize) -> Result<(), AnyhowError> {
        for i in 0..self.items.len() {
            self.times_inspected += 1;

            let mut worry = self.items[i];
            match self.op {
                Op::Add => {
                    let f_num = self.factor.ok_or(anyhow!("Bad factor"))? as usize;
                    worry += f_num;
                }
                Op::Multiply => {
                    let f_num = self.factor.ok_or(anyhow!("Bad factor"))? as usize;
                    worry *= f_num;
                }
                Op::Square => {
                    worry *= worry;
                }
            }
            // No relief in part 2!
            // worry = (worry as f64 / 3.0).floor() as usize;

            worry = worry % modulo;

            self.items[i] = worry;
        }

        Ok(())
    }

    fn who_next(&self) -> Vec<usize> {
        self.items
            .iter()
            .map(|worry| {
                if worry % self.test_factor as usize == 0 {
                    self.throw_true
                } else {
                    self.throw_false
                }
            })
            .collect::<Vec<_>>()
    }
}

fn do_round(mut monkeys: Vec<Monkey>, modulo: usize) -> Result<Vec<Monkey>, AnyhowError> {
    let mut mut_monkeys = borrow_mut_elementwise(&mut monkeys);
    for i in 0..mut_monkeys.len() {
        mut_monkeys[i].inspect_items(modulo)?;

        let targets = mut_monkeys[i].who_next();

        for j in 0..targets.len() {
            let what = mut_monkeys[i].items[j];
            let who = targets[j];

            mut_monkeys[who].items.push(what);
        }
        mut_monkeys[i].items.clear();
    }

    Ok(monkeys)
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    for (i, m) in monkeys.into_iter().enumerate() {
        println!(
            "Monkey {}: {:?} (inspected {} times)",
            i, m.items, m.times_inspected
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut monkeys: Vec<Monkey> = vec![];
    let mut lines = input.lines();

    while let Some(m) = Monkey::build(&mut lines)? {
        monkeys.push(m);
    }

    let modulo = &monkeys
        .iter()
        .map(|m| m.test_factor as usize)
        .reduce(|acc, it| acc * it)
        .ok_or(anyhow!("No modulo!"))?;

    dbg!(modulo);

    let mut current_monkeys = monkeys;

    for round in 1..=10_000 {
        current_monkeys = do_round(current_monkeys.clone(), *modulo)?;

        if round % 1000 == 0 {
            println!("===== Round {} =====", { round });
            print_monkeys(&current_monkeys);
            println!("");
        }
    }

    let mut counts = current_monkeys
        .into_iter()
        .map(|m| m.times_inspected)
        .collect::<Vec<_>>();

    counts.sort_by(|a, b| b.cmp(a));

    let monkey_business = counts
        .into_iter()
        .take(2)
        .reduce(|acc, it| acc * it)
        .ok_or(anyhow!("No monkey business!"))?;

    println!("Monkey business: {}", monkey_business);

    Ok(())
}
