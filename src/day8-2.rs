use std::collections::{HashMap, HashSet};

mod common;

type Cell = (i32, i32);

fn is_cell_outside(cell: Cell, width: i32, height: i32) -> bool {
    return cell.0 < 0 || cell.0 >= width || cell.1 < 0 || cell.1 >= height;
}
fn main() {
    let grid = common::load_lines("inputs/day8.txt");
    let grid_height = grid.len() as i32;
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let grid_width = grid[0].len() as i32;

    let mut antenna_map: HashMap<char, Vec<Cell>> = HashMap::new();

    for y in 0..grid_height {
        for x in 0..grid_width {
            let c = grid[y as usize][x as usize];
            if c != '.' {
                let entry = antenna_map.entry(c).or_insert(Vec::new());
                entry.push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<Cell> = HashSet::new();

    for (_, antennas) in antenna_map.iter() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                let antenna_1 = antennas[i];
                let antenna_2 = antennas[j];
                antinodes.insert(antenna_1);
                antinodes.insert(antenna_2);
                let dx = antenna_1.0 - antenna_2.0;
                let dy = antenna_1.1 - antenna_2.1;

                let mut antinode = antenna_1;
                loop {
                    antinode = (antinode.0 + dx, antinode.1 + dy);
                    if is_cell_outside(antinode, grid_width, grid_height) {
                        break;
                    }
                    antinodes.insert(antinode);
                }
                let mut antinode = antenna_2;
                loop {
                    antinode = (antinode.0 - dx, antinode.1 - dy);
                    if is_cell_outside(antinode, grid_width, grid_height) {
                        break;
                    }
                    antinodes.insert(antinode);
                }
            }
        }
    }

    for y in 0..grid_height {
        for x in 0..grid_width {
            let c = grid[y as usize][x as usize];
            let is_antinode = antinodes.contains(&(x, y));
            if is_antinode {
                if c != '.' {
                    print!("{}{}", c, '^');
                } else {
                    print!("#");
                }
            } else {
                print!("{}", c);
            }
        }
        print!("\n");
    }

    println!("count: {}", antinodes.len())
}
