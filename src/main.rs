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

#[allow(dead_code)]
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

fn create_report(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|x| x.trim().parse::<i32>().expect("Parse error")).collect()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return report.len() == 1;
    }
    let increasing = report[0] < report[1];
    for i in 0..(report.len()-1) {
        if increasing && report[i] >= report[i+1] {
            return false;
        }
        if !increasing && report[i] <= report[i+1] {
            return false;
        }
        if report[i].abs_diff(report[i+1]) > 3 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn day2_part1() -> usize {
    let filename = "inputs/day2.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    contents.split("\n")
        .map(create_report)
        .filter(|x| is_safe(x))
        .count()
}

fn main() {
    let result = day2_part1();
    println!("result={result}");
}
