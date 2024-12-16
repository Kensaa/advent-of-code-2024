use std::collections::VecDeque;

mod common;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellType {
    WALL,
    CRATE,
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
type Grid = Vec<Vec<CellType>>;

fn print_grid(grid: &Grid) {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    for y in 0..grid_height {
        for x in 0..grid_width {
            match grid[y][x] {
                CellType::CRATE => print!("O"),
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

    let mut grid: Grid = robots
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|cell| match cell {
                    '#' => CellType::WALL,
                    'O' => CellType::CRATE,
                    '.' => CellType::EMPTY,
                    '@' => CellType::ROBOT,
                    _ => {
                        panic!("invalid char");
                    }
                })
                .collect()
        })
        .collect();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut robot_x = -1;
    let mut robot_y = -1;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if let CellType::ROBOT = grid[y][x] {
                robot_x = x as isize;
                robot_y = y as isize;
            }
        }
    }

    if robot_x == -1 || robot_y == -1 {
        panic!("invalid input data");
    }

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

    // print_grid(&grid);
    for m in moves {
        // println!("MOVE : {:?}", m);
        let mut next = next_cell((robot_x as isize, robot_y as isize), m);
        let mut next_c = grid[next.1 as usize][next.0 as usize];
        let mut move_stack: VecDeque<Cell> = VecDeque::new();
        move_stack.push_front((robot_x as isize, robot_y as isize));

        while cell_in_grid(&grid, next) && next_c != CellType::EMPTY && next_c != CellType::WALL {
            move_stack.push_front(next);

            next = next_cell(next, m);
            next_c = grid[next.1 as usize][next.0 as usize];
        }

        while let Some(cell) = move_stack.pop_front() {
            let n = next_cell(cell, m);
            if grid[n.1 as usize][n.0 as usize] == CellType::EMPTY {
                grid[n.1 as usize][n.0 as usize] = grid[cell.1 as usize][cell.0 as usize];
                grid[cell.1 as usize][cell.0 as usize] = CellType::EMPTY;
                if grid[n.1 as usize][n.0 as usize] == CellType::ROBOT {
                    robot_x = n.0;
                    robot_y = n.1;
                }
            }
        }
        // print_grid(&grid);
    }

    let mut sum = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if grid[y][x] == CellType::CRATE {
                sum += x as u32 + y as u32 * 100;
            }
        }
    }

    println!("{}", sum);
}

fn cell_in_grid(grid: &Grid, cell: Cell) -> bool {
    let grid_height = grid.len();
    let grid_width = grid[0].len();

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
