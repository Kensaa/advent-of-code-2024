use std::collections::HashSet;

mod common;

type Grid = Vec<Vec<char>>;

fn main() {
    let grid: Grid = common::load_lines("inputs/day12.txt")
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    print_grid(&grid);

    let height = grid.len();
    let width = grid[0].len();

    let mut seen_cells: HashSet<(usize, usize)> = HashSet::with_capacity(width * height);

    let mut total_price = 0;

    for y in 0..height {
        for x in 0..width {
            if seen_cells.contains(&(x, y)) {
                continue;
            }
            let region = explore(&grid, &mut seen_cells, x as i32, y as i32);
            // println!("{},{} => {:?}", x, y, region);
            let fence_price = region.0 * region.1;
            total_price += fence_price;
        }
    }
    // let r = explore(&grid, &mut seen_cells, 0, 0);

    // println!("{:?}", r);
    // println!("{:?}", seen_cells);

    println!("price : {}", total_price);
}

fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::with_capacity(4);
    neighbors.push((x - 1, y));
    neighbors.push((x + 1, y));

    neighbors.push((x, y - 1));
    neighbors.push((x, y + 1));
    return neighbors;
}

fn is_cell_outside(grid: &Grid, x: i32, y: i32) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    return x < 0 || x >= width || y < 0 || y >= height;
}

fn print_grid(grid: &Grid) {
    let height = grid.len();
    let width = grid[0].len();

    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y][x]);
        }
        print!("\n");
    }
}

/// return a tuple (area, perimeter)
fn explore(
    grid: &Grid,
    seen_cells: &mut HashSet<(usize, usize)>,
    x: i32,
    y: i32,
) -> (usize, usize) {
    let val = grid[y as usize][x as usize];
    seen_cells.insert((x as usize, y as usize));
    let mut area = 1;
    let mut perimeter = 0;

    let neighbors = get_neighbors(x, y);

    // println!("start of : {},{}", x, y);
    for neighbor in neighbors {
        let nx = neighbor.0;
        let ny = neighbor.1;
        if is_cell_outside(grid, nx, ny) {
            // if the neighbor cell is outside, it means that the current cell is on an edge, meaning that we can increase its perimeter
            // println!("\t{},{} => outside", nx, ny);
            perimeter += 1;
        } else if grid[ny as usize][nx as usize] != val {
            // the neighbor cell is not the same value as the one we are on, so we increase the perimeter of the current cell.
            perimeter += 1;
            // println!("\t{},{} => not the same", nx, ny);
        } else if !seen_cells.contains(&(nx as usize, ny as usize)) {
            // println!("\t{},{} => never seen", nx, ny);
            // if we have already seen the cell, we are not going to count it an other time
            let neighbor_info = explore(grid, seen_cells, nx, ny);
            area += neighbor_info.0;
            perimeter += neighbor_info.1;
        }
    }
    // println!("end of : {},{}", x, y);
    return (area, perimeter);
}
