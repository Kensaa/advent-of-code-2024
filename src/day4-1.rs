use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = File::open("inputs/day4.txt").expect("failed to open file");
    let input_file = BufReader::new(input_file);
    let lines: Vec<Vec<char>> = input_file
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(|l| l.chars().collect())
        .collect();

    let mut directions: Vec<(i32, i32)> = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            directions.push((x, y));
        }
    }

    let target: Vec<char> = "XMAS".chars().collect();

    let width = lines.get(0).unwrap().len() as i32;
    let height = lines.len() as i32;
    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            for dir in directions.iter() {
                let mut tx = x as i32;
                let mut ty = y as i32;
                let (dx, dy) = dir;
                let mut target_i = 0usize;
                while tx < width
                    && tx >= 0
                    && ty < height
                    && ty >= 0
                    && target_i < target.len()
                    && lines[tx as usize][ty as usize] == target[target_i]
                {
                    target_i += 1;
                    tx = tx + dx;
                    ty = ty + dy;
                }
                if target_i == target.len() {
                    // println!("found at {} {}, dir: {:?}", x, y, dir);
                    count += 1;
                }
            }
        }
    }

    println!("count: {}", count);
}
