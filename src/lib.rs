use std::{collections::HashMap, fs};

pub fn run() -> usize {
    let filename = "inputs/day15.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolve_single_step() {
        assert_eq!(1, 1);
    }
}
