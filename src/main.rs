use std::{collections::HashMap, fs};
use regex::Regex;

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

fn exclude_index(report: &[i32], i: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(report.len() - 1);
    result.extend_from_slice(&report[..i]);
    result.extend_from_slice(&report[i + 1..]);
    result
}

fn is_safe_while_removing_problem(report: &[i32], i: usize) -> bool {
    let mut around = false;
    if i > 0 {
        around |= is_safe(&exclude_index(report, i-1));
    }
    if i+1 < report.len() {
        around |= is_safe(&exclude_index(report, i+1));
    }
    around || is_safe(&exclude_index(report, i))
}

fn is_safe_with_problem_dampener(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return report.len() == 1;
    }
    let increasing = report[0] < report[1];
    for i in 0..(report.len()-1) {
        if increasing && report[i] >= report[i+1] {
            return is_safe_while_removing_problem(report, i);
        }
        if !increasing && report[i] <= report[i+1] {
            return is_safe_while_removing_problem(report, i);
        }
        if report[i].abs_diff(report[i+1]) > 3 {
            return is_safe_while_removing_problem(report, i);
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

#[allow(dead_code)]
fn day2_part2() -> usize {
    let filename = "inputs/day2.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    contents.split("\n")
        .map(create_report)
        .filter(|x| is_safe_with_problem_dampener(x))
        .count()
}

#[allow(dead_code)]
fn day3_part1() -> i32 {
    let filename = "inputs/day3.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    re.captures_iter(&contents)
        .map(|x| x[1].parse::<i32>().expect("Parse error") * x[2].parse::<i32>().expect("Parse error"))
        .sum()
}

fn main() {
    let result = day3_part1();
    println!("result={result}");
}
