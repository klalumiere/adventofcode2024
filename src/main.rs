use std::fs;

#[allow(dead_code)]
fn day10_part1() -> isize {
    let filename = "inputs/day9.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}

fn main() {
    let result = day10_part1();
    println!("result={result}");
}
