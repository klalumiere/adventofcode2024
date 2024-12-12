use std::{collections::HashMap, fs};


pub fn run() -> usize {
    let filename = "inputs/day11.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    // let stones: Vec<Stone> = content.split(' ')
    //     .map(|x| x.parse::<usize>().expect("positive integer"))
    //     .map(Stone::new)
    //     .collect();
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(1, 1);
    }
}
