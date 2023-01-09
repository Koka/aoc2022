use std::{error::Error, fs};

fn get_neighbor_water(it: &Vec<Vec<Vec<u8>>>, x: isize, y: isize, z: isize) -> Option<()> {
    if x < 0 || y < 0 || z < 0 {
        return Some(());
    }
    let x = x as usize;
    let y = y as usize;
    let z = z as usize;

    it.get(x)
        .map(|arr| {
            arr.get(y)
                .map(|arr| arr.get(z).map(|v| if *v == 2 { Some(()) } else { None }))
        })
        .flatten()
        .flatten()
        .flatten()
}

fn flood_fill(it: &mut Vec<Vec<Vec<u8>>>, x: usize, y: usize, z: usize, kind: u8) {
    let mut stack = vec![];

    stack.push((x, y, z));

    while let Some((x, y, z)) = stack.pop() {
        if it[x][y][z] != 0 {
            continue;
        }
        it[x][y][z] = kind;

        if x > 0 {
            stack.push((x - 1, y, z));
        }
        if x < it.len() - 1 {
            stack.push((x + 1, y, z));
        }
        if y > 0 {
            stack.push((x, y - 1, z));
        }
        if y < it.len() - 1 {
            stack.push((x, y + 1, z));
        }
        if z > 0 {
            stack.push((x, y, z - 1));
        }
        if z < it.len() - 1 {
            stack.push((x, y, z + 1));
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;
    let data = input
        .lines()
        .filter_map(|s| {
            let mut arr = s.split(",");
            let x: usize = arr.next()?.parse().ok()?;
            let y: usize = arr.next()?.parse().ok()?;
            let z: usize = arr.next()?.parse().ok()?;
            Some((x, y, z))
        })
        .collect::<Vec<_>>();

    let size = 32;
    let mut droplet = vec![vec![vec![0u8; size]; size]; size];
    for (x, y, z) in data {
        droplet[x][y][z] = 1u8;
    }

    flood_fill(&mut droplet, size - 1, size - 1, size - 1, 2);

    let mut surface: usize = 0;
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                let me = droplet[x][y][z];
                if me != 1 {
                    continue;
                }

                let x = x as isize;
                let y = y as isize;
                let z = z as isize;

                let neighbors = [
                    get_neighbor_water(&droplet, x - 1, y, z),
                    get_neighbor_water(&droplet, x + 1, y, z),
                    get_neighbor_water(&droplet, x, y - 1, z),
                    get_neighbor_water(&droplet, x, y + 1, z),
                    get_neighbor_water(&droplet, x, y, z - 1),
                    get_neighbor_water(&droplet, x, y, z + 1),
                ];

                let neighbor_count = neighbors
                    .into_iter()
                    .filter_map(|v| v.map(|_| 1u8))
                    .sum::<u8>();

                surface += neighbor_count as usize;
            }
        }
    }

    println!("Surface: {}", surface);

    Ok(())
}
