mod common;

#[derive(Clone, Copy, Debug)]
enum Block {
    Empty,
    Filled(u32),
}

fn main() {
    let data: Vec<u32> = common::load_file("inputs/day9.txt")
        .chars()
        .into_iter()
        .map(|char| {
            char.to_string()
                .parse::<u32>()
                .expect("failed to parse char")
        })
        .collect();

    let mut memory: Vec<Block> = Vec::new();
    let mut index = 0;
    for (i, e) in data.into_iter().enumerate() {
        let block = if i % 2 == 1 {
            Block::Empty
        } else {
            let b = Block::Filled(index);
            index += 1;
            b
        };

        for _ in 0..e {
            memory.push(block);
        }
    }

    let mut i = memory.len() - 1;
    while i > 0 {
        match memory[i] {
            Block::Filled(_) => {
                let empty_pos = memory
                    .iter()
                    .position(|block| {
                        if let Block::Empty = block {
                            return true;
                        } else {
                            return false;
                        }
                    })
                    .expect("failed to find empty block");

                if empty_pos >= i {
                    break;
                }
                memory.swap(empty_pos, i);
            }
            _ => {}
        };
        i -= 1;
    }

    let checksum: u64 = memory
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Empty => 0,
            Block::Filled(n) => (i as u64) * (*n as u64),
        })
        .sum();

    println!("{}", checksum);
}
