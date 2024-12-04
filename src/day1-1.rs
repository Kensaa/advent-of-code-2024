mod common;

fn main() {
    let lines = common::load_lines("inputs/day1.txt");

    let mut list1: Vec<u32> = Vec::with_capacity(lines.len());
    let mut list2: Vec<u32> = Vec::with_capacity(lines.len());

    for line in lines {
        let (n1, n2) = line.split_at(line.find("   ").unwrap());
        list1.push(n1.trim().parse().unwrap());
        list2.push(n2.trim().parse().unwrap());
    }

    assert!(list1.len() == list2.len(), "not same size");
    list1.sort();
    list2.sort();

    let mut list3: Vec<u32> = Vec::with_capacity(list1.len());

    for (n1, n2) in list1.into_iter().zip(list2) {
        list3.push(n1.abs_diff(n2));
    }
    let sum: u32 = list3.iter().sum();
    println!("sum: {}", sum);
}
