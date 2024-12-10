mod common;

#[derive(Clone, Copy, Debug)]
struct File {
    id: usize,
    size: usize,
}

const EMPTY: usize = usize::MAX;

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

    let mut memory: Vec<File> = Vec::new();
    for (i, e) in data.into_iter().enumerate() {
        if i % 2 == 0 {
            memory.push(File {
                id: i / 2,
                size: e as usize,
            })
        } else {
            memory.push(File {
                id: EMPTY,
                size: e as usize,
            })
        }
    }

    // for block in memory.iter() {
    //     println!("{:?}", block);
    // }

    let mut target_file_index = memory.len() - 1;
    while target_file_index > 0 {
        // print_memory(&memory);

        let target_file = memory[target_file_index];

        for empty_pos in 0..target_file_index {
            let empty_block = memory[empty_pos];
            if is_empty(&empty_block) {
                if empty_block.size == target_file.size {
                    // if the free space is the same size as the file, just swap them
                    memory.swap(target_file_index, empty_pos);
                    break;
                }
                if empty_block.size > target_file.size {
                    // if the free space is bigger than the file, insert the file at the start of the free space and reduce it
                    memory[empty_pos].size -= target_file.size;
                    memory[target_file_index].id = EMPTY;
                    memory.insert(empty_pos, target_file);
                    break;
                }
            }
        }

        // find next file (next file is the file with the id target_file.id - 1)
        target_file_index = memory
            .iter()
            .position(|file| file.id == target_file.id - 1)
            .expect("failed to next find file");
    }

    println!("{}", gen_checksum(&memory));
}

fn is_empty(file: &File) -> bool {
    return file.id == EMPTY;
}

fn gen_checksum(memory: &Vec<File>) -> u64 {
    let mut checksum = 0;
    let mut index = 0;
    for file in memory {
        if is_empty(file) {
            index += file.size;
        } else {
            for _ in 0..file.size {
                checksum += index as u64 * file.id as u64;
                index += 1;
            }
        }
    }
    return checksum;
}
