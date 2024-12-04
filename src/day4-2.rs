mod common;

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
    let grid: Vec<Vec<char>> = common::load_lines("inputs/day4.txt")
        .into_iter()
        .map(|l| l.chars().collect())
        .collect();

    let width = grid.get(0).unwrap().len();
    let height = grid.len();

    let mut count = 0;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if is_cross(&grid, x, y) {
                count += 1;
            }
        }
    }
    println!("count: {}", count);
}
