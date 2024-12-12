use std::collections::HashMap;

mod common;

const REPETITION: u64 = 75;

/// (value,blink_self)
type Stone = (u64, u64);

fn main() {
    let data = common::load_file("inputs/day11.txt");
    let stones: Vec<u64> = data
        .split(" ")
        .map(|n| n.parse::<u64>().expect("failed to parse number"))
        .collect();

    let mut blink_map: HashMap<Stone, u64> = HashMap::new();
    let total: u64 = stones
        .into_iter()
        .map(|stone| blink(&mut blink_map, (stone, REPETITION)))
        .sum();

    println!("{}", total);
}

// Map<(stone_value, blink_left), num_of_stones>
fn blink(blink_map: &mut HashMap<Stone, u64>, stone: Stone) -> u64 {
    if let Some(count) = blink_map.get(&stone) {
        return *count;
    }
    let val = stone.0;
    let blink_left = stone.1;

    if blink_left == 0 {
        return 1;
    }

    let val_string = val.to_string();

    let mut count = 0;

    if val == 0 {
        count += blink(blink_map, (1, blink_left - 1));
    } else if val_string.len() % 2 == 0 {
        let mid = val_string.len() / 2;
        let new_val1 = val_string[0..mid].parse::<u64>().unwrap();
        let new_val2 = val_string[mid..].parse::<u64>().unwrap();
        count += blink(blink_map, (new_val1, blink_left - 1));
        count += blink(blink_map, (new_val2, blink_left - 1));
    } else {
        count += blink(blink_map, (val * 2024, blink_left - 1));
    }

    blink_map.insert(stone, count);

    return count;
}
