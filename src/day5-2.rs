use std::collections::HashMap;

mod common;

/// Returns the index of the first error if one was found, None otherwise
fn find_line_error(line: &Vec<u32>, position_map: &HashMap<u32, Vec<u32>>) -> Option<usize> {
    for (i, e) in line.iter().enumerate() {
        let e_is_before = position_map.get(&e).expect("value is not in the map");
        for j in 0..i {
            if e_is_before.contains(&line[j]) {
                return Some(i);
            }
        }
    }
    return None;
}

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
    let mut position_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for order in order_lines {
        let l = position_map.entry(order.0).or_insert(Vec::new());
        l.push(order.1);
    }

    println!("total :{}", pages_lines.len());

    let mut fixed_lines = Vec::new();

    for mut line in pages_lines.into_iter() {
        if find_line_error(&line, &position_map).is_some() {
            // if line is not valid, we find the first element that causes an error, and move it before the first element that sould be after it
            println!("line is not valid : {:?}", line);
            while let Some(error) = find_line_error(&line, &position_map) {
                let elem_before = position_map.get(&line[error]).expect("element not in map");
                let first = line
                    .iter()
                    .position(|e| elem_before.contains(e))
                    .expect("could not find error");
                line.swap(error, first);
            }
            println!("line fixed : {:?}", line);
            fixed_lines.push(line.clone());
        }
    }

    let sum: u32 = fixed_lines
        .into_iter()
        .map(|line| line[line.len() / 2])
        .sum();

    println!("{}", sum);
}
