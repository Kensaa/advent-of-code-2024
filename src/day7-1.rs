use std::collections::VecDeque;

mod common;

fn main() {
    let lines = common::load_lines("inputs/day7.txt");
    let lines: Vec<(VecDeque<u64>, u64)> = lines
        .into_iter()
        .map(|mut line| {
            let divider = line.chars().position(|c| c == ':').expect(": not found");
            let inputs = line.split_off(divider);
            let result = line.trim().parse::<u64>().expect("failed to parse result");

            let inputs: VecDeque<u64> = inputs
                .replace(":", "")
                .trim()
                .split(' ')
                .map(|input| input.trim().parse::<u64>().expect("failed to parse input"))
                .collect();

            (inputs, result)
        })
        .collect();

    let mut sum = 0;
    for line in lines {
        let inputs = line.0;
        let result = line.1;

        if feur(inputs, result, 0) {
            sum += result;
        }
    }
    println!("result : {}", sum);
}

enum Operation {
    ADD,
    MULT,
}

fn compute(a: u64, b: u64, operation: &Operation) -> u64 {
    match operation {
        Operation::ADD => a + b,
        Operation::MULT => a * b,
    }
}

fn feur(mut inputs: VecDeque<u64>, result: u64, curr: u64) -> bool {
    if inputs.len() == 0 {
        return curr == result;
    }

    let next = inputs.pop_front().unwrap();
    let operations = vec![Operation::ADD, Operation::MULT];
    return operations
        .iter()
        .map(|op| compute(next, curr, op))
        .map(|i| feur(inputs.clone(), result, i))
        .any(|r| r);
}
