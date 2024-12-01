use std::fs;

fn day1() -> u32 {
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

fn main() {
    let result = day1();
    println!("result={result}");
}
