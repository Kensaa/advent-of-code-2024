mod common;

struct Robot {
    x: u32,
    y: u32,
    vx: i32,
    vy: i32,
}

fn main() {
    let robots: Vec<Robot> = common::load_lines("inputs/day14.txt")
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
}
