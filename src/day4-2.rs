use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_cross(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let c = grid[y][x];
    if c == 'A' {
        match grid[y + 1][x - 1] {
            'M' => {
                if grid[y - 1][x + 1] != 'S' {
                    return false;
                }
            }
            'S' => {
                if grid[y - 1][x + 1] != 'M' {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
        match grid[y + 1][x + 1] {
            'M' => {
                if grid[y - 1][x - 1] != 'S' {
                    return false;
                }
            }
            'S' => {
                if grid[y - 1][x - 1] != 'M' {
                    return false;
                }
            }
            _ => {
                return false;
            }
        };
        return true;
    }
    return false;
}

fn main() {
    let input_file = File::open("inputs/day4.txt").expect("failed to open file");
    let input_file = BufReader::new(input_file);
    let lines: Vec<Vec<char>> = input_file
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(|l| l.chars().collect())
        .collect();

    let width = lines.get(0).unwrap().len();
    let height = lines.len();

    let mut count = 0;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if is_cross(&lines, x, y) {
                count += 1;
            }
        }
    }
    println!("count: {}", count);
}
