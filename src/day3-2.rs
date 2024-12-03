use regex::{self, Regex};
use std::{
    fs::File,
    io::{BufReader, Read},
};
fn main() {
    let input_file = File::open("inputs/day3.txt").expect("failed to open file");
    let mut input_file = BufReader::new(input_file);
    let mut data = Vec::new();
    let _ = input_file
        .read_to_end(&mut data)
        .expect("failed to read file");

    let mut data = String::from_utf8(data).expect("failed to convert to string");
    data = data.replace("\n", "");

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

    let mut enabled = true;

    let mut results = Vec::new();

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
                    // println!("{}*{}*{}", n, m, n * m);
                    n * m
                })
                .collect::<Vec<_>>();

            // println!("{:?}", mults);
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

    // let mut results = Vec::new();

    // for caps in regex.captures_iter(&data) {
    //     let n = caps
    //         .get(1)
    //         .map_or(0, |m| m.as_str().parse().expect("failed to parse number"));
    //     let m = caps
    //         .get(2)
    //         .map_or(0, |m| m.as_str().parse().expect("failed to parse number"));
    //     results.push(n * m);
    // }

    // let sum: u64 = results.iter().sum();

    // println!("{}", sum);
}
