use std::fs;

pub fn run() -> usize {
    let filename = "inputs/day12.txt";
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
