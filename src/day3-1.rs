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
