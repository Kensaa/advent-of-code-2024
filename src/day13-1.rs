mod common;

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
        lines.next();
    }
}

fn parse_line(s: String) -> (u64, u64) {
    // s = "Button A: X+15, Y+61"
    let s = s.split(": ").nth(1).expect("failed to parse button line");
    // s ="X+15, Y+61"
    let s = s.split(", ");
    // s ="X+15","Y+61"
    let mut s = s.map(|e| e.split('+').nth(1).expect("failed to parse button line"));
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
fn solve(machine: Machine) -> (u64, u64) {
    let (a_x,a_y,b_x,b_y,r_x,r_y) = machine;

    let y = () / ()
}
