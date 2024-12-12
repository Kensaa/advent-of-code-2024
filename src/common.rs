use std::{
    env::args,
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

#[allow(dead_code)]
pub fn load_lines(default_file: &str) -> Vec<String> {
    let args: Vec<String> = args().collect();
    let mut file = default_file;
    if args.len() >= 2 {
        if let Some(f) = args.last() {
            if fs::exists(f).unwrap() {
                file = f;
            }
        }
    }

    let input_file = File::open(file).expect("failed to open file");
    let input_file = BufReader::new(input_file);
    let lines: Vec<String> = input_file
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(|l| l.trim().to_string())
        .filter(|l| *l != "")
        .collect();
    return lines;
}

#[allow(dead_code)]
pub fn load_file(default_file: &str) -> String {
    let args: Vec<String> = args().collect();
    let mut file = default_file;
    if args.len() >= 2 {
        if let Some(f) = args.last() {
            if fs::exists(f).unwrap() {
                file = f;
            }
        }
    }

    let input_file = File::open(file).expect("failed to open file");
    let mut input_file = BufReader::new(input_file);
    let mut data = Vec::new();
    let _ = input_file
        // .read_to_end(&mut data)
        .read_to_end(&mut data)
        .expect("failed to read file");
    let mut data = String::from_utf8(data).expect("failed to convert to string");
    data = data.replace("\n", "").trim().to_string();
    return data;
}
