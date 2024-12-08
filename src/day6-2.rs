use std::collections::HashSet;

mod common;

type Cell = (i32, i32);

struct Guard {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Guard {
    fn next_cell(&self) -> Cell {
        match self.dir {
            Direction::NORTH => {
                return (self.x, self.y - 1);
            }
            Direction::EAST => {
                return (self.x + 1, self.y);
            }
            Direction::SOUTH => {
                return (self.x, self.y + 1);
            }
            Direction::WEST => {
                return (self.x - 1, self.y);
            }
        }
    }

    fn forward(&mut self) {
        let next = self.next_cell();
        self.x = next.0;
        self.y = next.1;
    }

    fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn is_cell_outside(cell: Cell, width: i32, height: i32) -> bool {
    return cell.0 < 0 || cell.0 >= width || cell.1 < 0 || cell.1 >= height;
}

fn main() {
    let grid = common::load_lines("inputs/day6.txt");
    let grid_height = grid.len() as i32;
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let grid_width = grid[0].len() as i32;

    let guard_y = grid
        .iter()
        .position(|line| line.contains(&'^'))
        .expect("could not find guard") as i32;
    let guard_x = grid[guard_y as usize]
        .iter()
        .position(|c| *c == '^')
        .expect("could not find guard") as i32;

    let mut guard = Guard {
        x: guard_x,
        y: guard_y,
        dir: Direction::NORTH,
    };

    let mut seen_cells: HashSet<(usize, usize)> = HashSet::new();

    loop {
        seen_cells.insert((guard.x as usize, guard.y as usize));

        let next_cell = guard.next_cell();
        if is_cell_outside(next_cell, grid_width, grid_height) {
            break;
        }
        if grid[next_cell.1 as usize][next_cell.0 as usize] == '#' {
            guard.turn();
        } else {
            guard.forward();
        }
    }
    println!("seen cells count: {}", seen_cells.len());

    let mut loop_count = 0;

    for cell in seen_cells.iter() {
        let mut guard = Guard {
            x: guard_x,
            y: guard_y,
            dir: Direction::NORTH,
        };
        let mut local_seen_cells: HashSet<(usize, usize, Direction)> =
            HashSet::with_capacity(seen_cells.len());
        let mut tmp_grid = grid.clone();
        tmp_grid[cell.1][cell.0] = '#';

        loop {
            let to_add = (guard.x as usize, guard.y as usize, guard.dir);
            if local_seen_cells.contains(&to_add) {
                loop_count += 1;
                break;
            } else {
                local_seen_cells.insert(to_add);
            }

            let next_cell = guard.next_cell();
            if is_cell_outside(next_cell, grid_width, grid_height) {
                break;
            }
            if tmp_grid[next_cell.1 as usize][next_cell.0 as usize] == '#' {
                guard.turn();
            } else {
                guard.forward();
            }
        }
    }

    println!("loop count : {}", loop_count);
}
