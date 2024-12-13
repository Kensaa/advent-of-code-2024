use std::collections::HashSet;

mod common;

type Grid = Vec<Vec<char>>;

type Region = HashSet<(usize, usize)>;
fn main() {
    let grid: Grid = common::load_lines("inputs/day12.txt")
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut seen_cells: HashSet<(usize, usize)> = HashSet::with_capacity(width * height);
    let mut regions: Vec<Region> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if seen_cells.contains(&(x, y)) {
                continue;
            }
            let region = explore(&grid, &mut seen_cells, x as i32, y as i32);
            regions.push(region);
        }
    }

    let mut total_price = 0;
    for region in regions {
        let sides = get_region_sides(&region);
        let price = region.len() * sides;
        total_price += price;
    }

    println!("total : {}", total_price);
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

/// we do that because there is the same amount of corners as sides
fn explore(grid: &Grid, seen_cells: &mut HashSet<(usize, usize)>, x: i32, y: i32) -> Region {
    let val = grid[y as usize][x as usize];
    let cell = (x as usize, y as usize);
    seen_cells.insert(cell);

    let mut region: Region = Region::new();
    region.insert(cell);

    let neighbors = get_neighbors(x, y);
    for neighbor in neighbors {
        let nx = neighbor.0;
        let ny = neighbor.1;
        if !is_cell_outside(grid, nx, ny) {
            if !seen_cells.contains(&(nx as usize, ny as usize)) {
                if grid[ny as usize][nx as usize] == val {
                    let mut sub_region = explore(grid, seen_cells, nx, ny);
                    region.extend(sub_region.drain());
                }
            }
        }
    }

    return region;
}

fn region_cell_exists(region: &Region, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    return region.contains(&(x as usize, y as usize));
}

fn get_region_sides(region: &Region) -> usize {
    let mut corner = 0;
    for cell in region {
        let x = cell.0 as i32;
        let y = cell.1 as i32;

        let top = region_cell_exists(region, x, y - 1);
        let right = region_cell_exists(region, x + 1, y);
        let bottom = region_cell_exists(region, x, y + 1);
        let left = region_cell_exists(region, x - 1, y);
        let bottom_right = region_cell_exists(region, x + 1, y + 1);
        let bottom_left = region_cell_exists(region, x - 1, y + 1);
        let top_right = region_cell_exists(region, x + 1, y - 1);
        let top_left = region_cell_exists(region, x - 1, y - 1);

        // see https://www.reddit.com/r/adventofcode/comments/1hcpyic/comment/m1q437r
        if !top_left && top == left {
            corner += 1;
        }
        if top_left && !top && !left {
            corner += 1;
        }

        if !top_right && top == right {
            corner += 1;
        }
        if top_right && !top && !right {
            corner += 1;
        }

        if !bottom_left && bottom == left {
            corner += 1;
        }
        if bottom_left && !bottom && !left {
            corner += 1;
        }

        if !bottom_right && bottom == right {
            corner += 1;
        }
        if bottom_right && !bottom && !right {
            corner += 1;
        }
    }

    return corner;
}
