use regex::{self, Regex};
mod common;

fn main() {
    let data = common::load_file("inputs/day3.txt");

    let mul_regex = Regex::new(r"mul\((0|\d{0,3}),(0|\d{0,3})\)").unwrap();
    let cond_regex = Regex::new(r"don't\(\)|do\(\)").unwrap();

    let do_pos = cond_regex
        .captures_iter(&data)
        .map(|capture| capture.get(0).unwrap());

    let mut slices = Vec::new();
    let mut start = 0;
    for m in do_pos {
        slices.push(data[start..m.end()].to_string());
        start = m.end();
    }
    slices.push(data[start..].to_string());

    let mut results = Vec::new();
    let mut enabled = true;
    for slice in slices {
        if enabled {
            let mults = mul_regex
                .captures_iter(&slice)
                .map(|capture| {
                    let n = capture
                        .get(1)
                        .map_or(0, |m| m.as_str().parse().expect("failed to parse number"));
                    let m = capture
                        .get(2)
                        .map_or(0, |m| m.as_str().parse().expect("failed to parse number"));
                    n * m
                })
                .collect::<Vec<_>>();

            results.extend(mults);
        }
        if slice.ends_with("do()") {
            enabled = true;
        } else {
            enabled = false;
        }
    }

    let sum: u64 = results.iter().sum();
    println!("{}", sum);
}
