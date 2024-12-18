mod common;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellType {
    WALL,
    CrateLeft,
    CrateRight,
    ROBOT,
    EMPTY,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

type Cell = (isize, isize);
struct Grid {
    grid: Vec<Vec<CellType>>,
    robot_x: usize,
    robot_y: usize,
}

fn print_grid(grid: &Grid) {
    let grid_height = grid.grid.len();
    let grid_width = grid.grid[0].len();
    for y in 0..grid_height {
        for x in 0..grid_width {
            match grid.grid[y][x] {
                CellType::CrateLeft => print!("["),
                CellType::CrateRight => print!("]"),
                CellType::EMPTY => print!("."),
                CellType::ROBOT => print!("@"),
                CellType::WALL => print!("#"),
            }
        }
        print!("\n");
    }
}
fn main() {
    let mut robots: Vec<String> = common::load_lines("inputs/day15.txt");
    let empty_line_index = robots
        .iter()
        .position(|l| l == "")
        .expect("failed to parse input file");

    let moves = robots.split_off(empty_line_index + 1);
    robots.pop();

    let grid: Vec<Vec<CellType>> = robots
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|cell| match cell {
                    '#' => vec![CellType::WALL, CellType::WALL],
                    'O' => vec![CellType::CrateLeft, CellType::CrateRight],
                    '.' => vec![CellType::EMPTY, CellType::EMPTY],
                    '@' => vec![CellType::ROBOT, CellType::EMPTY],
                    _ => {
                        panic!("invalid char");
                    }
                })
                .flatten()
                .collect()
        })
        .collect();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut robot_x = usize::MAX;
    let mut robot_y = usize::MAX;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if let CellType::ROBOT = grid[y][x] {
                robot_x = x;
                robot_y = y;
            }
        }
    }

    if robot_x == usize::MAX || robot_y == usize::MAX {
        panic!("invalid input data");
    }

    let mut grid = Grid {
        grid,
        robot_x,
        robot_y,
    };

    let moves: Vec<Move> = moves
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|m| match m {
                    '^' => Move::UP,
                    '>' => Move::RIGHT,
                    'v' => Move::DOWN,
                    '<' => Move::LEFT,
                    _ => {
                        panic!("invalid char");
                    }
                })
                .collect::<Vec<Move>>()
        })
        .flatten()
        .collect();

    println!("START:");
    print_grid(&grid);

    for m in moves {
        let robot = (grid.robot_x as isize, grid.robot_y as isize);
        if !is_move_possible(&grid, robot, m) {
            continue;
        }
        move_cell(&mut grid, robot, m);
    }

    let mut sum = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if grid.grid[y][x] == CellType::CrateLeft {
                sum += x as u32 + y as u32 * 100;
            }
        }
    }

    println!("{}", sum);
}

fn is_move_possible(grid: &Grid, cell: Cell, m: Move) -> bool {
    let curr_x = cell.0 as usize;
    let curr_y = cell.1 as usize;
    let curr_type = grid.grid[curr_y][curr_x];
    match m {
        Move::DOWN | Move::UP => {
            let next_cells = get_cell_pair(grid, next_cell(cell, m));
            let mut possible = true;
            for next in next_cells {
                let next_x = next.0 as usize;
                let next_y = next.1 as usize;
                let next_type = grid.grid[next_y][next_x];
                if next_type == CellType::CrateLeft || next_type == CellType::CrateRight {
                    if !get_cell_pair(grid, next)
                        .into_iter()
                        .all(|c| is_move_possible(grid, c, m))
                    {
                        possible = false;
                    }
                } else if next_type == CellType::WALL {
                    possible = false;
                }
            }
            return possible;
        }
        Move::LEFT if curr_type == CellType::CrateRight => {
            return is_move_possible(grid, (cell.0 - 1, cell.1), m);
        }
        Move::RIGHT if curr_type == CellType::CrateLeft => {
            return is_move_possible(grid, (cell.0 + 1, cell.1), m);
        }
        Move::RIGHT | Move::LEFT => {
            let next = next_cell(cell, m);
            let next_x = next.0 as usize;
            let next_y = next.1 as usize;
            let next_type = grid.grid[next_y][next_x];
            if next_type != CellType::EMPTY && next_type != CellType::WALL {
                return is_move_possible(grid, next, m);
            }
            return next_type == CellType::EMPTY;
        }
    }
}

fn move_cell(grid: &mut Grid, cell: Cell, m: Move) {
    let next = next_cell(cell, m);
    if !cell_in_grid(grid, next) {
        return;
    }
    let next_x = next.0 as usize;
    let next_y = next.1 as usize;
    let curr_x = cell.0 as usize;
    let curr_y = cell.1 as usize;

    let next_cells = match m {
        Move::LEFT | Move::RIGHT => {
            vec![next]
        }
        Move::DOWN | Move::UP => get_cell_pair(grid, next),
    };

    for next in next_cells {
        let next_type = grid.grid[next_y][next_x];
        if next_type != CellType::EMPTY && next_type != CellType::WALL {
            move_cell(grid, next, m);
        }
        let next_type = grid.grid[next_y][next_x];
        if next_type == CellType::EMPTY {
            grid.grid[next_y][next_x] = grid.grid[curr_y][curr_x];
            grid.grid[curr_y][curr_x] = CellType::EMPTY;
            if grid.grid[next_y][next_x] == CellType::ROBOT {
                grid.robot_x = next_x;
                grid.robot_y = next_y;
            }
        }
    }
}

fn cell_in_grid(grid: &Grid, cell: Cell) -> bool {
    let grid_height = grid.grid.len();
    let grid_width = grid.grid[0].len();

    return !(cell.0 < 0
        || cell.0 >= grid_width as isize
        || cell.1 < 0
        || cell.1 >= grid_height as isize);
}

fn next_cell(cell: Cell, m: Move) -> Cell {
    let x = cell.0;
    let y = cell.1;
    match m {
        Move::UP => (x, y - 1),
        Move::RIGHT => (x + 1, y),
        Move::DOWN => (x, y + 1),
        Move::LEFT => (x - 1, y),
    }
}

/// Given a cell that is a create (Left and Right), returns a 2 elements array containing the given part and the other part of the cell
fn get_cell_pair(grid: &Grid, cell: Cell) -> Vec<Cell> {
    match grid.grid[cell.1 as usize][cell.0 as usize] {
        CellType::CrateLeft => vec![(cell.0 + 1, cell.1), cell],
        CellType::CrateRight => vec![cell, (cell.0 - 1, cell.1)],
        _ => vec![cell],
    }
}
