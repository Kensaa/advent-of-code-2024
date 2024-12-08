use std::collections::HashMap;

mod common;

fn main() {
    let mut lines = common::load_lines("inputs/day5.txt");
    let sep = lines
        .iter()
        .position(|line| **line == "".to_string())
        .expect("invalid file");
    let mut pages_lines = lines.split_off(sep);
    pages_lines.remove(0);

    let order_lines: Vec<(u32, u32)> = lines
        .into_iter()
        .map(|l| l.split("|").map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|pair| {
            let e1 = pair
                .get(0)
                .expect("invalid file")
                .parse::<u32>()
                .expect("failed to parse number");
            let e2 = pair
                .get(1)
                .expect("invalid file")
                .parse::<u32>()
                .expect("failed to parse number");
            (e1, e2)
        })
        .collect();

    let pages_lines: Vec<Vec<u32>> = pages_lines
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().expect("failed to parse number"))
                .collect::<Vec<u32>>()
        })
        .collect();

    // println!("{:?}", lines);
    // println!("{:?}", order_lines);

    let mut map = HashMap::new();
    for order in order_lines {
        
    }
    map.entry(key)

    println!("{:?}", pages_lines);
}
