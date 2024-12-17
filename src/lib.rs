use std::fs;

pub fn run() -> usize {
    let filename = "inputs/day16.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_step_forward() {
//         assert_eq!(1, 1);
//     }
// }
