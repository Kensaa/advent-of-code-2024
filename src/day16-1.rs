use std::collections::HashMap;

mod common;

const MOVE_COST: u32 = 1;
const TURN_COST: u32 = 1000;

struct Grid {
    grid: Vec<Vec<CellType>>,
    start: Cell,
    end: Cell,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellType {
    WALL,
    EMPTY,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
    fn rotate_anticlockwise(&self) -> Self {
        match self {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        }
    }
}

type Cell = (usize, usize);

#[derive(Debug)]
struct Node {
    pos: Cell,
    dir: Direction,
    cost: u32,
}

fn main() {
    let lines: Vec<String> = common::load_lines("inputs/day16.txt");

    let mut grid: Vec<Vec<CellType>> = Vec::with_capacity(lines.len());
    let mut start_x = usize::MAX;
    let mut start_y = usize::MAX;
    let mut end_x = usize::MAX;
    let mut end_y = usize::MAX;
    for (y, line) in lines.into_iter().enumerate() {
        let mut line_vec: Vec<CellType> = Vec::with_capacity(line.len());
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start_x = x;
                start_y = y;
            }
            if char == 'E' {
                end_x = x;
                end_y = y;
            }
            if char == '#' {
                line_vec.push(CellType::WALL);
            } else {
                line_vec.push(CellType::EMPTY);
            }
        }
        grid.push(line_vec);
    }

    if start_x == usize::MAX || start_y == usize::MAX || end_x == usize::MAX || end_y == usize::MAX
    {
        panic!("invalid input data");
    }

    let grid = Grid {
        grid,
        start: (start_x, start_y),
        end: (end_x, end_y),
        direction: Direction::EAST,
    };

    // A* algorithm : heuristic function (dist) is the Manhattan distance between two cells
    let mut open_set: Vec<(Cell, Direction)> = Vec::new();
    let mut came_from: HashMap<(Cell, Direction), (Cell, Direction)> = HashMap::new();
    let mut g_scores: HashMap<Cell, u32> = HashMap::new();
    let mut f_scores: HashMap<Cell, u32> = HashMap::new();

    open_set.push((grid.start, grid.direction));
    g_scores.insert(grid.start, 0);
    f_scores.insert(grid.start, dist(grid.start, grid.end));

    while !open_set.is_empty() {
        let mut lowest_index = 0;
        let mut lowest_score = f_scores.get(&open_set[lowest_index].0).unwrap_or(&u32::MAX);
        for (i, e) in open_set.iter().enumerate() {
            let score = f_scores.get(&e.0).unwrap_or(&u32::MAX);
            if score < lowest_score {
                lowest_score = score;
                lowest_index = i;
            }
        }

        let current = open_set[lowest_index];
        open_set.swap_remove(lowest_index);
        if current.0 == grid.end {
            // lets gooooo
            // println!("path found :{:?}", came_from);
            let path = build_path(came_from, current);
            // println!("path found : {:?}", path);
            println!("cost : {}", g_scores.get(&(end_x, end_y)).unwrap());
            // print_grid_with_path(&grid, &path);
            break;
        }

        let neighbors = get_cell_neighbors(&grid, current.0, current.1);
        // println!("current cell : {:?}", current);
        // println!("neighbors : {:?}", neighbors);
        for neighbor in neighbors {
            // skip if it's a wall
            if grid.grid[neighbor.cell.1][neighbor.cell.0] == CellType::WALL {
                continue;
            }
            // tentative_gScore := gScore[current] + d(current, neighbor)
            let tentative_g_score = g_scores.get(&current.0).unwrap_or(&u32::MAX) + neighbor.cost;
            if tentative_g_score < *g_scores.get(&neighbor.cell).unwrap_or(&u32::MAX) {
                // this path is better
                came_from.insert((neighbor.cell, neighbor.direction), current);
                g_scores.insert(neighbor.cell, tentative_g_score);
                f_scores.insert(
                    neighbor.cell,
                    tentative_g_score + dist(neighbor.cell, grid.end),
                );
                if !open_set.contains(&(neighbor.cell, neighbor.direction)) {
                    open_set.push((neighbor.cell, neighbor.direction));
                }
            }
        }
    }
}

fn print_grid_with_path(grid: &Grid, path: &HashMap<Cell, Direction>) {
    let grid_height = grid.grid.len();
    let grid_width = grid.grid[0].len();
    for y in 0..grid_height {
        for x in 0..grid_width {
            let cell = grid.grid[y][x];
            let is_in_path = path.contains_key(&(x, y));
            if is_in_path {
                let path_dir = path.get(&(x, y)).unwrap();
                match path_dir {
                    Direction::NORTH => print!("^"),
                    Direction::EAST => print!(">"),
                    Direction::SOUTH => print!("v"),
                    Direction::WEST => print!("<"),
                }
            } else {
                match cell {
                    CellType::EMPTY => print!("."),
                    CellType::WALL => print!("#"),
                }
            }
        }
        print!("\n");
    }
}

fn build_path(
    came_from: HashMap<(Cell, Direction), (Cell, Direction)>,
    current: (Cell, Direction),
) -> HashMap<Cell, Direction> {
    let mut curr = current;
    let mut path = HashMap::new();
    path.insert(curr.0, curr.1);
    while came_from.contains_key(&curr) {
        curr = *came_from.get(&curr).unwrap();
        path.insert(curr.0, curr.1);
    }
    return path;
}

fn dist(cell_1: Cell, cell_2: Cell) -> u32 {
    (cell_1.0.abs_diff(cell_2.0) + cell_1.1.abs_diff(cell_2.1))
        .try_into()
        .expect("distance is too big")
}

#[derive(Debug)]
struct NeighborCell {
    /// The cell
    cell: Cell,
    /// the cost to move to that cell
    cost: u32,
    /// the direction from the source cell in which this cell is
    direction: Direction,
}

/// Given the grid and a cell, returns a vector of NeighborCell
fn get_cell_neighbors(grid: &Grid, cell: Cell, direction: Direction) -> Vec<NeighborCell> {
    let mut neighbors = Vec::with_capacity(4);

    // cell in front
    let front_dir = direction;
    let front = get_cell_in_dir(grid, cell, front_dir);
    if let Some(front) = front {
        neighbors.push(NeighborCell {
            cell: front,
            cost: MOVE_COST,
            direction: front_dir,
        });
    }
    // cell to the right
    let right_dir = direction.rotate_clockwise();
    let right = get_cell_in_dir(grid, cell, right_dir);
    if let Some(right) = right {
        neighbors.push(NeighborCell {
            cell: right,
            cost: TURN_COST + MOVE_COST,
            direction: right_dir,
        });
    }
    // cell to the left
    let left_dir = direction.rotate_anticlockwise();
    let left = get_cell_in_dir(grid, cell, left_dir);
    if let Some(left) = left {
        neighbors.push(NeighborCell {
            cell: left,
            cost: TURN_COST + MOVE_COST,
            direction: left_dir,
        });
    }
    // cell behind
    let behind_dir = right_dir.rotate_clockwise();
    let behind = get_cell_in_dir(grid, cell, behind_dir);
    if let Some(behind) = behind {
        neighbors.push(NeighborCell {
            cell: behind,
            cost: TURN_COST * 2 + MOVE_COST,
            direction: behind_dir,
        });
    }
    return neighbors;
}

fn get_cell_in_dir(grid: &Grid, cell: Cell, direction: Direction) -> Option<Cell> {
    let grid_height = grid.grid.len();
    let grid_width = grid.grid[0].len();
    let x = cell.0;
    let y = cell.1;
    match direction {
        Direction::NORTH => {
            if y == 0 {
                return None;
            }
            return Some((x, y - 1));
        }
        Direction::EAST => {
            if x == grid_width - 1 {
                return None;
            }
            return Some((x + 1, y));
        }
        Direction::SOUTH => {
            if y == grid_height - 1 {
                return None;
            }
            return Some((x, y + 1));
        }
        Direction::WEST => {
            if x == 0 {
                return None;
            }
            return Some((x - 1, y));
        }
    }
}
