use std::{collections::{HashMap, HashSet}, fs};


#[allow(dead_code)]
fn day9_part1() -> usize {
    let filename = "inputs/day8.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}

fn main() {
    let result = day9_part1();
    println!("result={result}");
}
