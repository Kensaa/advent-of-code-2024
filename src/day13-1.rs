mod common;

#[derive(Debug)]
struct Machine {
    /// Result X
    r_x: u64,
    /// Result Y
    r_y: u64,
    /// Button A x increment
    a_x: u64,
    /// Button A y increment
    a_y: u64,
    /// Button B x increment
    b_x: u64,
    /// Button B y increment
    b_y: u64,
}

fn main() {
    let mut lines = common::load_lines("inputs/day13.txt").into_iter();
    let mut machines: Vec<Machine> = Vec::new();

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let (a_x, a_y) = parse_line(a);
        let (b_x, b_y) = parse_line(b);
        let (r_x, r_y) = parse_line(c);
        machines.push(Machine {
            r_x,
            r_y,
            a_x,
            a_y,
            b_x,
            b_y,
        });
    }

    let mut token_count = 0;
    for (i, machine) in machines.into_iter().enumerate() {
        println!("machine n°{}:", i);
        let res = solve(&machine);
        match res {
            None => {
                println!("Not solvable");
            }
            Some((x, y)) => {
                println!("x = {}, y = {}", x, y);
                token_count += x * 3 + y;
            }
        }
    }

    println!("token count : {}", token_count);
}

fn parse_line(s: String) -> (u64, u64) {
    // s = "Button A: X+15, Y+61"
    let s = s.split(": ").nth(1).expect("failed to parse button line");
    // s ="X+15, Y+61"
    let s = s.split(", ");
    // s ="X+15","Y+61"
    let mut s = s.map(|e| {
        e.split('+')
            .map(|s| s.split('='))
            .flatten()
            .nth(1)
            .expect("failed to parse button line")
    });
    // s = [15,61]

    let x = s
        .next()
        .expect("failed to parse button line")
        .parse::<u64>()
        .expect("failed to parse number");
    let y = s
        .next()
        .expect("failed to parse button line")
        .parse::<u64>()
        .expect("failed to parse number");

    return (x, y);
}

/// X = \frac{\lambda_x-B_yY}{A_x}
/// Y = \frac{A_x\lambda_y - A_y\lambda_x}{A_xB_y - A_yB_x}
///
/// X number of Button A press
///
/// Y number of Button B press
///
/// Where A_x = Button A x increment
///
/// Where A_y = Button A y increment
///
/// Where B_x = Button B x increment
///
/// Where B_y = Button B y increment
///
/// Where \lambda_x = X Target
///
/// Where \lambda_y = Y Target
fn solve(machine: &Machine) -> Option<(u64, u64)> {
    let a_x = machine.a_x as i64;
    let a_y = machine.a_y as i64;
    let b_x = machine.b_x as i64;
    let b_y = machine.b_y as i64;
    let r_x = machine.r_x as i64;
    let r_y = machine.r_y as i64;

    let y_top = a_x * r_y - a_y * r_x;
    let y_bot = a_x * b_y - a_y * b_x;
    let y = y_top / y_bot;
    // println!("y = {}/{} = {}", y_top, y_bot, y);

    let x_top = r_x - b_x * y;
    let x_bot = a_x;
    let x = x_top / x_bot;

    // println!("x = {}/{} = {}", x_top, x_bot, x);
    if x < 0 || y < 0 {
        return None;
    }
    let x = x as u64;
    let y = y as u64;
    if !check_result(machine, x, y) {
        return None;
    }
    return Some((x, y));
}

fn check_result(machine: &Machine, x: u64, y: u64) -> bool {
    let tx = x * machine.a_x + y * machine.b_x;
    let ty = x * machine.a_y + y * machine.b_y;

    return tx == machine.r_x && ty == machine.r_y;
}
