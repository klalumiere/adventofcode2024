use std::fs;

pub fn run() -> usize {
    let filename = "inputs/day10.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}
