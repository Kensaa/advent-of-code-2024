mod common;

type Grid = Vec<Vec<u32>>;
fn main() {
    let grid: Grid = common::load_lines("inputs/day10.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<u32>()
                        .expect("failed to parse number")
                })
                .collect::<Vec<u32>>()
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                // this is a starting point
                sum += find_paths(&grid, x, y);
            }
        }
    }
    println!("sum : {}", sum);
}

fn get_neighbors(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut neighbors = Vec::with_capacity(4);
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 {
        neighbors.push((x + 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 {
        neighbors.push((x, y + 1));
    }
    return neighbors;
}

fn find_paths(grid: &Grid, x: usize, y: usize) -> u32 {
    let val = grid[y][x];
    if val == 9 {
        return 1;
    }

    let neighbors = get_neighbors(grid, x, y);
    let mut sum = 0;
    for n in neighbors {
        if grid[n.1][n.0] == val + 1 {
            sum += find_paths(grid, n.0, n.1);
        }
    }
    return sum;
}
