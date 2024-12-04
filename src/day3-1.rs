use regex::{self, Regex};
mod common;

fn main() {
    let data = common::load_file("inputs/day3.txt");

    let regex = Regex::new(r"mul\((0|\d{0,3}),(0|\d{0,3})\)").expect("failed to create regex");
    let mut results = Vec::new();

    for caps in regex.captures_iter(&data) {
        let n = caps
            .get(1)
            .map_or(0, |m| m.as_str().parse().expect("failed to parse number"));
        let m = caps
            .get(2)
            .map_or(0, |m| m.as_str().parse().expect("failed to parse number"));
        results.push(n * m);
    }

    let sum: u64 = results.iter().sum();

    println!("{}", sum);
}
