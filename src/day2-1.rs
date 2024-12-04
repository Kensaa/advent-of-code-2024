mod common;

fn is_safe(report: &Vec<u8>) -> bool {
    let mut report_iter = report.iter();
    let mut prev = report_iter.next().unwrap();

    let mut cmp: fn(&u8, &u8) -> bool = u8::lt;
    for (i, e) in report_iter.enumerate() {
        if i == 0 {
            cmp = if prev < e { u8::lt } else { u8::gt }
        }

        if !cmp(prev, e) {
            return false;
        }

        if prev.abs_diff(*e) > 3 {
            return false;
        }

        prev = e;
    }
    return true;
}

fn main() {
    let lines = common::load_lines("inputs/day2.txt");

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
