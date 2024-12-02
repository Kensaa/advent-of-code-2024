use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = File::open("inputs/day1/input1.txt").expect("failed to open file");
    let input_file = BufReader::new(input_file);
    let lines: Vec<String> = input_file
        .lines()
        .map(|l| l.expect("failed to read line"))
        .collect();

    let mut list1: Vec<u32> = Vec::with_capacity(lines.len());
    let mut list2: Vec<u32> = Vec::with_capacity(lines.len());
    for line in lines.into_iter() {
        let (n1, n2) = line.split_at(line.find("   ").unwrap());
        list1.push(n1.trim().parse().unwrap());
        list2.push(n2.trim().parse().unwrap());
    }

    assert!(list1.len() == list2.len(), "not same size");

    let mut map: HashMap<u32, u32> = HashMap::new();
    for num in list2 {
        map.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut score = 0;
    for num in list1 {
        let count = map.get(&num);
        let count = match count {
            None => 0,
            Some(n) => *n,
        };
        score += num * count;
    }

    println!("score: {}", score);
}
