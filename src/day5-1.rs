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

    // the key is before every elements in the vec
    let mut is_before: HashMap<u32, Vec<u32>> = HashMap::new();
    for order in order_lines {
        let l = is_before.entry(order.0).or_insert(Vec::new());
        l.push(order.1);
    }

    let mut valid_lines: Vec<Vec<u32>> = Vec::new();

    for line in pages_lines.iter() {
        let mut valid = true;
        for (i, e) in line.iter().enumerate() {
            let e_is_before = is_before.get(&e).expect("value is not in the map");
            for j in 0..i {
                if e_is_before.contains(&line[j]) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            valid_lines.push(line.clone());
        }
    }

    println!("{}", pages_lines.len());
    println!("{}", valid_lines.len());

    let sum: u32 = valid_lines
        .into_iter()
        .map(|line| line[line.len() / 2])
        .sum();

    println!("{}", sum);
}
