use std::fs;

// fn parse_equations(content: &str) -> Vec<SystemOfEquation> {
//     let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
//     let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
//     let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

//     let mut equations: Vec<SystemOfEquation> = Vec::new();
//     let mut lines = content.lines();
//     while let Some(line0) = lines.next() {
//         let caps_a = re_button_a.captures(line0).unwrap();
//         let line1 = lines.next().expect("to be a line 1");
//         let caps_b = re_button_b.captures(line1).unwrap();
//         let line2 = lines.next().expect("to be a line 2");
//         let caps_prize = re_prize.captures(line2).unwrap();
//         let _ = lines.next(); // optional empty line
    
//         let a_x = caps_a[1].parse::<isize>().expect("a number");
//         let a_y = caps_a[2].parse::<isize>().expect("a number");
//         let b_x = caps_b[1].parse::<isize>().expect("a number");
//         let b_y = caps_b[2].parse::<isize>().expect("a number");
//         let d_x = caps_prize[1].parse::<isize>().expect("a number");
//         let d_y = caps_prize[2].parse::<isize>().expect("a number");
//         let equation_x = Equation { a: a_x, b: b_x, d: d_x };
//         let equation_y = Equation { a: a_y, b: b_y, d: d_y };

//         equations.push(SystemOfEquation { equation_x, equation_y });
//     }
//     equations
// }

pub fn run() -> isize {
    let filename = "inputs/day13.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_count_digits() {
//         assert_eq!(1, 1);
//     }
// }
