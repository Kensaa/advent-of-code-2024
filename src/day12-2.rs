use std::collections::HashSet;

mod common;

type Grid = Vec<Vec<char>>;

type Region = HashSet<(usize, usize)>;
fn main() {
    let grid: Grid = common::load_lines("inputs/day12.txt")
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    print_grid(&grid);
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
    // for region in regions {
    let region = &regions[0];
    let info = get_region_info(&region);
    println!("region : {:?}", region);
    println!("infos : {:?}", info);
    println!();
    let price = info.0 * info.1;
    total_price += price;
    // }

    println!("total : {}", total_price);
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
    if x < 0 || y == 0 {
        return false;
    }
    return region.contains(&(x as usize, y as usize));
}

/// Returns a tuple containing (area,corner)
fn get_region_info(region: &Region) -> (usize, usize) {
    let mut corner = 0;
    for cell in region.into_iter() {
        let x = cell.0 as i32;
        let y = cell.1 as i32;

        // there is 8 ways that a cell could be a corner :
        // the 4 types of corners (┐,┌,┘,└)
        // but each type of corner can be either convex (the angle points outwards, meaning that the rest of the region is "inside" the angle) or concave (the angle points inward)

        let top = region_cell_exists(region, x, y - 1);
        let right = region_cell_exists(region, x + 1, y);
        let bottom = region_cell_exists(region, x, y - 1);
        let left = region_cell_exists(region, x - 1, y);
        let bottom_right = region_cell_exists(region, x + 1, y - 1);
        let bottom_left = region_cell_exists(region, x - 1, y - 1);
        let top_right = region_cell_exists(region, x + 1, y + 1);
        let top_left = region_cell_exists(region, x - 1, y + 1);

        let total_count = top as usize + right as usize + bottom as usize + left as usize;
        if total_count == 2 {
            // bottom left (┐)
            if bottom && left {
                println!("{},{} => ┐", x, y);
                corner += 1;
            }
            // bottom right (┌)
            if bottom && right {
                println!("{},{} => ┌", x, y);
                corner += 1;
            }
            // top left (┘)
            if top && left {
                println!("{},{} => ┘", x, y);
                corner += 1;
            }
            // top right (└)
            if top && right {
                println!("{},{} => └", x, y);
                corner += 1;
            }
        }

        // check if current cell is a corner
        // there is 4 type of corner
        // bottom left (┐)
    }

    return (region.len(), corner);
}
