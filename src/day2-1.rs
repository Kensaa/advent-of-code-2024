use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_safe(report: &Vec<u8>) -> bool {
    let mut report = report.into_iter();
    let e = report.next().unwrap();
    let mut prev = report.next().unwrap();

    let op = if e < prev {
        u8::le
    } else if e > prev {
        u8::ge
    } else {
        return false;
    };

    if e.abs_diff(*prev) == 0 || e.abs_diff(*prev) > 3 {
        return false;
    }

    for e in report {
        if !op(prev, e) {
            return false;
        }
        if e.abs_diff(*prev) == 0 || e.abs_diff(*prev) > 3 {
            return false;
        }
        prev = e;
    }

    return true;
}

fn main() {
    let input_file = File::open("inputs/day2.txt").expect("failed to open file");
    let input_file = BufReader::new(input_file);
    let lines: Vec<String> = input_file
        .lines()
        .map(|l| l.expect("failed to read line"))
        .collect();

    let mut reports: Vec<Vec<u8>> = Vec::with_capacity(lines.len());
    for line in lines {
        let report: Vec<u8> = line
            .split(" ")
            .map(|e| e.parse::<u8>().expect("failed to parse number"))
            .collect();

        reports.push(report);
    }

    let count: u32 = reports.iter().map(is_safe).map(|e| e as u32).sum();
    println!("{}", count);
}
