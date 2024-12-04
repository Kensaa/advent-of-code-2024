mod common;

fn main() {
    let grid: Vec<Vec<char>> = common::load_lines("inputs/day4.txt")
        .into_iter()
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

    let width = grid.get(0).unwrap().len() as i32;
    let height = grid.len() as i32;
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
                    && grid[ty as usize][tx as usize] == target[target_i]
                {
                    target_i += 1;
                    tx = tx + dx;
                    ty = ty + dy;
                }
                if target_i == target.len() {
                    count += 1;
                }
            }
        }
    }

    println!("count: {}", count);
}
