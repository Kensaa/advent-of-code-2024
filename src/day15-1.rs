mod common;

enum CellType {
    WALL,
    CRATE,
    ROBOT,
    EMPTY,
}

enum Move {
    UP,
    RIGHT,
    BOTTOM,
    LEFT,
}

fn main() {
    let mut robots: Vec<String> = common::load_lines("inputs/day15.txt");
    let empty_line_index = robots
        .iter()
        .position(|l| l == "")
        .expect("failed to parse input file");

    let moves = robots.split_off(empty_line_index + 1);

    let grid: Vec<Vec<CellType>> = robots
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
                .collect::<Vec<CellType>>()
        })
        .collect();

    let moves: Vec<Move> = moves
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|m| match m {
                    '^' => Move::UP,
                    '>' => Move::RIGHT,
                    'v' => Move::BOTTOM,
                    '<' => Move::LEFT,
                    _ => {
                        panic!("invalid char");
                    }
                })
                .collect::<Vec<Move>>()
        })
        .flatten()
        .collect::<Vec<Move>>();
}
