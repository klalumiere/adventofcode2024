use std::collections::HashSet;
use std::fs;
use std::cmp::max;
use std::cmp::min;

use regex::Regex;

#[derive(Debug)]
struct Equation {
    a: isize,
    b: isize,
    d: isize,
}

#[derive(Debug)]
struct SystemOfEquation {
    equation_x: Equation,
    equation_y: Equation,
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn solve_diophantine(a: isize, b: isize, d: isize) -> Option<(isize, isize)> {
    let g = gcd(a, b);
    if d % g != 0 {
        return None; // No solution
    }
    let (g, x0, y0) = extended_gcd(a, b);
    let scale = d / g;
    Some((x0 * scale, y0 * scale))
}

fn generate_solution(a: isize, b: isize, x0: isize, y0: isize, k: isize) -> (isize, isize) {
    let g = gcd(a, b);
    let x = x0 + k * (b / g);
    let y = y0 - k * (a / g);
    (x, y)
}

fn get_scaling_range(a: isize, b: isize, x0: isize, y0: isize, max_solution: isize) -> (isize, isize) {
    let delta: isize = 10; // just to be sure not to make a off by one error
    let g = gcd(a, b);
    let k_min = max((-x0 * g) / b, (y0 * g - max_solution*g) / a);
    let k_max = min((y0 * g) / a, (max_solution*g -x0 * g) / b);
    (k_min - delta, k_max + delta)
}

fn generate_solutions(a: isize, b: isize, d: isize, max_solution: isize) -> HashSet<(isize, isize)> {
    let mut solutions: HashSet<(isize, isize)> = HashSet::new();
    if let Some((x0, y0)) = solve_diophantine(a, b, d) {
        let (k_min, k_max) = get_scaling_range(a, b, x0, y0, max_solution);
        for k in k_min..k_max {
            solutions.insert(generate_solution(a, b, x0, y0, k));
        }
    }
    solutions.into_iter().filter(|(i, j)| *i >= 0 && *j >= 0 && *i <= max_solution && *j <= max_solution ).collect()
}

fn parse_equations(content: &str) -> Vec<SystemOfEquation> {
    let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut equations: Vec<SystemOfEquation> = Vec::new();
    let mut lines = content.lines();
    while let Some(line0) = lines.next() {
        let caps_a = re_button_a.captures(line0).unwrap();
        let line1 = lines.next().expect("to be a line 1");
        let caps_b = re_button_b.captures(line1).unwrap();
        let line2 = lines.next().expect("to be a line 2");
        let caps_prize = re_prize.captures(line2).unwrap();
        let _ = lines.next(); // optional empty line
    
        let a_x = caps_a[1].parse::<isize>().expect("a number");
        let a_y = caps_a[2].parse::<isize>().expect("a number");
        let b_x = caps_b[1].parse::<isize>().expect("a number");
        let b_y = caps_b[2].parse::<isize>().expect("a number");
        let d_x = caps_prize[1].parse::<isize>().expect("a number");
        let d_y = caps_prize[2].parse::<isize>().expect("a number");
        let equation_x = Equation { a: a_x, b: b_x, d: d_x };
        let equation_y = Equation { a: a_y, b: b_y, d: d_y };

        equations.push(SystemOfEquation { equation_x, equation_y });
    }
    equations
}

fn generate_solutions_for_system(system: &SystemOfEquation, max_solution: isize) -> Vec<(isize, isize)> {
    let SystemOfEquation { equation_x, equation_y } = system;
    let mut solutions: Vec<(isize, isize)> = Vec::new();
    let solution_x = generate_solutions(equation_x.a, equation_x.b, equation_x.d, max_solution);
    let solution_y = generate_solutions(equation_y.a, equation_y.b, equation_y.d, max_solution);
    for solution in solution_x {
        if solution_y.contains(&solution) {
            solutions.push(solution);
        }
    }
    solutions
}

fn get_solution_price(solution: (isize, isize), price_a: isize, price_b: isize) -> isize {
    let (a, b) = solution;
    a * price_a + b * price_b
}

pub fn run() -> isize {
    const MAX_SOLUTION: isize = 100;
    const PRICE_A: isize = 3;
    const PRICE_B: isize = 1;
    let filename = "inputs/day13.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let systems_of_equations = parse_equations(&content);
    systems_of_equations.iter()
        .map(|system| generate_solutions_for_system(system, MAX_SOLUTION))
        .map(|solutions| solutions.iter().map(|solution| get_solution_price(*solution, PRICE_A, PRICE_B)).min().unwrap_or(0isize))
        .sum()
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_count_digits() {
//         assert_eq!(1, 1);
//     }
// }
