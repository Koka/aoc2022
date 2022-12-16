use anyhow::{anyhow, bail, Error as AnyhowError};
use itertools::Itertools;
use std::{cmp, error::Error, fmt::Display, fs, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    Just(u8),
    Nested(Vec<PacketData>),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Just(n) => {
                write!(f, "{}", n)
            }
            Self::Nested(p) => {
                write!(f, "[")?;
                if let Some((tail, head)) = p.split_last() {
                    for nested in head {
                        write!(f, "{}, ", nested)?;
                    }
                    write!(f, "{}", tail)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl FromStr for PacketData {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current: Vec<Vec<PacketData>> = vec![];
        let mut buf: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '[' => {
                    buf.clear();
                    current.push(vec![]);
                }
                '0'..='9' => {
                    buf.push(c);
                }
                ',' => {
                    if buf.len() > 0 {
                        let num: u8 = buf.iter().collect::<String>().parse()?;
                        buf.clear();

                        current
                            .last_mut()
                            .ok_or(anyhow!("No current"))?
                            .push(PacketData::Just(num));
                    }
                }
                ']' => {
                    if buf.len() > 0 {
                        let num: u8 = buf.iter().collect::<String>().parse()?;
                        buf.clear();

                        current
                            .last_mut()
                            .ok_or(anyhow!("No current"))?
                            .push(PacketData::Just(num));
                    }

                    if current.len() > 1 {
                        let nested = current.pop().ok_or(anyhow!("Can't pop this!"))?;
                        current
                            .last_mut()
                            .ok_or(anyhow!("No current"))?
                            .push(PacketData::Nested(nested));
                    }
                }
                err_c => {
                    bail!("Parse error: {}", err_c);
                }
            };
        }

        let packet = current.last().ok_or(anyhow!("No current"))?.clone();

        Ok(PacketData::Nested(packet))
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (PacketData::Just(a), PacketData::Just(b)) => a.cmp(b),
            (a @ PacketData::Nested(_), b @ PacketData::Just(_)) => {
                a.cmp(&PacketData::Nested(vec![b.clone()]))
            }
            (a @ PacketData::Just(_), b @ PacketData::Nested(_)) => {
                PacketData::Nested(vec![a.clone()]).cmp(b)
            }
            (PacketData::Nested(a), PacketData::Nested(b)) => {
                let mut result = cmp::Ordering::Equal;
                for i in 0..cmp::max(a.len(), b.len()) {
                    let ai = a.get(i);
                    let bi = b.get(i);

                    if ai.is_none() {
                        result = cmp::Ordering::Less;
                    } else if bi.is_none() {
                        result = cmp::Ordering::Greater;
                    } else {
                        let outcome = match ai.partial_cmp(&bi) {
                            ord @ Some(cmp::Ordering::Greater) => ord,
                            ord @ Some(cmp::Ordering::Less) => ord,
                            _ => None,
                        };

                        if let Some(res) = outcome {
                            result = res;
                            break;
                        }
                    }
                }
                result
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let pairs = input
        .lines()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.take(2)
                .filter_map(|s| {
                    s.parse::<PacketData>()
                        .map_err(|e| {
                            dbg!(&e);
                            e
                        })
                        .ok()
                })
                .collect::<Vec<_>>()
        })
        .map(|p| (p[0].clone(), p[1].clone()))
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, (lp, rp)) in pairs.iter().enumerate() {
        if lp.cmp(&rp) == cmp::Ordering::Less {
            sum += i + 1;
        }

        println!("{}", lp);
        println!("{}", rp);
        println!("lp {:?} rp", lp.cmp(&rp));
        println!();
    }

    println!("Sum is {}", sum);
    println!();

    let mut unpaired = pairs
        .into_iter()
        .flat_map(|p| vec![p.0, p.1])
        .collect::<Vec<_>>();

    let div_one = "[[2]]".parse::<PacketData>()?;
    let div_two = "[[6]]".parse::<PacketData>()?;

    unpaired.push(div_one.clone());
    unpaired.push(div_two.clone());

    unpaired.sort();

    for pkt in &unpaired {
        println!("{}", pkt);
    }

    let idx_one = unpaired
        .binary_search(&div_one)
        .map_err(|_| anyhow!("Divider not found"))?
        + 1;

    let idx_two = unpaired
        .binary_search(&div_two)
        .map_err(|_| anyhow!("Divider not found"))?
        + 1;

    println!("Decoder key: {}", idx_one * idx_two);

    Ok(())
}
