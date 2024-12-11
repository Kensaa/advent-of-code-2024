mod common;

const REPETITION: usize = 25;

fn main() {
    let data = common::load_file("inputs/day11.txt");
    let mut stones: Vec<u64> = data
        .split(" ")
        .map(|n| n.parse::<u64>().expect("failed to parse number"))
        .collect();

    // println!("init: {:?}", stones);
    for _ in 0..REPETITION {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones.get_mut(i).unwrap();
            let stone_string = stone.to_string();
            if *stone == 0 {
                *stone = 1;
                i += 1;
            } else if stone_string.len() % 2 == 0 {
                let mid = stone_string.len() / 2;
                let n1 = stone_string[0..mid].parse::<u64>().expect("1");
                let n2 = stone_string[mid..].parse::<u64>().expect("2");
                *stone = n1;
                stones.insert(i + 1, n2);
                i += 2;
            } else {
                *stone *= 2024;
                i += 1;
            }
        }
        // println!("{}: {:?}", r, stones);
    }

    println!("{}", stones.len());
}
