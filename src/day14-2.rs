use std::cmp::Ordering;

mod common;

const GRID_WIDTH: i32 = 101;
const GRID_HEIGHT: i32 = 103;

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

    let mut i = 0;
    loop {
        let mut max_amount = 0;
        // count the max number of robot in a verical line
        for x in 0..GRID_HEIGHT as u32 {
            let mut robot_column: Vec<&Robot> = robots.iter().filter(|r| r.x == x).collect();

            robot_column.sort_by(|a, b| {
                if a.y == b.y {
                    return Ordering::Equal;
                }
                if a.y < b.y {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            });

            let mut j = 0;

            while j < robot_column.len() {
                let mut curr_amount = 1;
                let mut r1 = robot_column[j];
                for r2 in robot_column.iter() {
                    if r2.y == r1.y + 1 {
                        curr_amount += 1;
                        r1 = r2;
                    }
                }
                if curr_amount > max_amount {
                    max_amount = curr_amount;
                }

                j += 1;
            }
        }

        if max_amount >= 10 {
            break;
        }

        for i in 0..robots.len() {
            robots.get_mut(i).unwrap().update();
        }
        i += 1;
    }

    print_robots(&robots);
    println!("{}", i);
}
