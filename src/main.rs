use std::{collections::HashMap, fs};

#[allow(dead_code)]
fn day1_part1() -> u32 {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: Vec<u32> = Vec::new();
    let filename = "inputs/day1.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    for part in contents.split("\n") {
        let data: Vec<&str> = part.split_whitespace().collect();
        if data.len() != 2 {
            continue;
        }
        lhs.push(data[0].trim().parse().expect("Parse error"));
        rhs.push(data[1].trim().parse().expect("Parse error"));
    }
    lhs.sort();
    rhs.sort();
    lhs.iter()
        .zip(rhs.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .reduce(|acc, x| acc+x)
        .unwrap_or(0)
}

fn day1_part2() -> u32 {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: HashMap<u32, u32> = HashMap::new();
    let filename = "inputs/day1.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    for part in contents.split("\n") {
        let data: Vec<&str> = part.split_whitespace().collect();
        if data.len() != 2 {
            continue;
        }
        lhs.push(data[0].trim().parse().expect("Parse error"));
        let number = data[1].trim().parse().expect("Parse error");
        *rhs.entry(number).or_insert(0) += 1;
    }
    let mut similarity = 0;
    for integer in lhs.iter() {
        match rhs.get(integer) {
            Some(count) => {
                similarity += integer*count;
            }
            None => continue,
        }
    }
    similarity
}

fn main() {
    let result = day1_part2();
    println!("result={result}");
}
