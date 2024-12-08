use std::{collections::{HashMap, HashSet}, fs};

#[allow(dead_code)]
fn day8_part1() -> isize {
    let filename = "inputs/day7.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let char_matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    0
}

fn main() {
    let result = day8_part1();
    println!("result={result}");
}
