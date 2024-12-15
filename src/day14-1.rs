mod common;

const GRID_WIDTH: i32 = 101;
const GRID_HEIGHT: i32 = 103;
const ITERATION: usize = 100;

struct Robot {
    x: u32,
    y: u32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn update(&mut self) {
        let mut nx = self.x as i32 + self.vx;
        let mut ny = self.y as i32 + self.vy;

        if nx < 0 {
            nx = GRID_WIDTH + nx;
        }
        if ny < 0 {
            ny = GRID_HEIGHT + ny;
        }

        self.x = (nx % GRID_WIDTH) as u32;
        self.y = (ny % GRID_HEIGHT) as u32;
    }
}

fn print_robots(robot: &Vec<Robot>) {
    for y in 0..GRID_HEIGHT as u32 {
        for x in 0..GRID_WIDTH as u32 {
            let count = robot.iter().filter(|r| r.x == x && r.y == y).count();
            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut robots: Vec<Robot> = common::load_lines("inputs/day14.txt")
        .into_iter()
        .map(|line| {
            let mut line_split = line.split(" ");
            let mut position = line_split.next().expect("failed to parse line").split('=');
            let mut velocity = line_split.next().expect("failed to parse line").split('=');
            position.next();
            velocity.next();
            let position = position
                .next()
                .expect("failed to parse line")
                .split(',')
                .map(|n| n.parse::<u32>().expect("failed to parse position"))
                .collect::<Vec<u32>>();
            let velocity = velocity
                .next()
                .expect("failed to parse line")
                .split(',')
                .map(|n| n.parse::<i32>().expect("failed to parse position"))
                .collect::<Vec<i32>>();

            Robot {
                x: *position.get(0).expect("failed to parse line"),
                y: *position.get(1).expect("failed to parse line"),
                vx: *velocity.get(0).expect("failed to parse line"),
                vy: *velocity.get(1).expect("failed to parse line"),
            }
        })
        .collect();

    for _ in 0..ITERATION {
        print_robots(&robots);
        for i in 0..robots.len() {
            robots.get_mut(i).unwrap().update();
        }
    }

    let mut quadrant_grid: Vec<Vec<u32>> = Vec::with_capacity(2);
    quadrant_grid.push(vec![0, 0]);
    quadrant_grid.push(vec![0, 0]);

    let middle_x = GRID_WIDTH as u32 / 2;
    let middle_y = GRID_HEIGHT as u32 / 2;
    for robot in robots {
        if robot.x == middle_x || robot.y == middle_y {
            continue;
        }

        let qx = robot.x / (middle_x + 1);
        let qy = robot.y / (middle_y + 1);

        quadrant_grid[qy as usize][qx as usize] += 1;
    }

    let safety_factor = quadrant_grid
        .iter()
        .map(|e| e.into_iter().product::<u32>())
        .product::<u32>();

    println!("safety factor : {}", safety_factor);
}
