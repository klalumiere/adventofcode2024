use std::{collections::HashMap, fs};

use pathfinding::prelude::dijkstra;
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Corruptions {
    position_to_time: HashMap<(isize, isize), isize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, std::hash::Hash)]
struct Position {
    x: isize,
    y: isize,
    t: isize,
}

impl Corruptions {
    fn from(content: &str) -> Corruptions {
        let mut position_to_time = HashMap::new();
        for (i, line) in content.lines().enumerate() {
            let coordinates = Regex::new(r"(\d+),(\d+)").unwrap();
            let x = coordinates.captures(line).unwrap()[1].parse::<isize>().expect("a number");
            let y = coordinates.captures(line).unwrap()[1].parse::<isize>().expect("a number");
            position_to_time.insert((x, y), i as isize);
        }

        Corruptions {  position_to_time }
    }
}

impl Position {
    const fn new(x: isize, y: isize, t: isize) -> Position {
        Position { x, y, t }
    }

    fn successors(&self, corruptions: &Corruptions, x_size: isize, y_size: isize) -> Vec<(Position, usize)> {
        let &Position { x, y, t } = self;
        let possibilities = vec![
            Position::new(x, y - 1, t + 1),
            Position::new(x, y + 1, t + 1),
            Position::new(x - 1, y, t + 1),
            Position::new(x + 1, y, t + 1)];
        possibilities.into_iter()
            .filter(|p| p.x >= 0 && p.y >= 0)
            .filter(|p| p.x < x_size && p.y < y_size)
            .filter(|p| {
                if let Some(t) = corruptions.position_to_time.get(&(p.x, p.y)) {
                    p.t < *t
                } else {
                    true
                }
            })
            .map(|p| (p, 1))
            .collect()
      }
}

pub fn run() -> usize {
    const MAP_SIZE: isize = 6;
    const GOAL: Position = Position::new(5, 5, 0);
    let filename = "inputs/day18.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let corruptions = Corruptions::from(&content);
    let result = dijkstra(&Position::new(0, 0, 0),
        |p| p.successors(&corruptions, MAP_SIZE, MAP_SIZE),
        |p| p.x == GOAL.x && p.y == GOAL.y);
    println!("result={:?}", result.expect("no path found"));
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
