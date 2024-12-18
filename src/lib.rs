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
            let y = coordinates.captures(line).unwrap()[2].parse::<isize>().expect("a number");
            position_to_time.insert((x, y), i as isize);
        }

        Corruptions {  position_to_time }
    }
}

impl Position {
    const fn new(x: isize, y: isize, t: isize) -> Position {
        Position { x, y, t }
    }

    fn successors(&self, corruptions: &Corruptions, x_size: isize, y_size: isize, fixed_time: isize) -> Vec<(Position, usize)> {
        let &Position { x, y, t } = self;
        let possibilities = vec![
            Position::new(x, y - 1, fixed_time),
            Position::new(x, y + 1, fixed_time),
            Position::new(x - 1, y, fixed_time),
            Position::new(x + 1, y, fixed_time)];
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

pub fn run() -> (isize, isize) {
    let filename = "inputs/day18.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let corruptions = Corruptions::from(&content);

    const MAP_SIZE: isize = 71; // 7
    const FIRST_TIME: isize = 1024; // 12;
    const GOAL: Position = Position::new(70, 70, 0); // 6, 6
    let last_time = corruptions.position_to_time.len() as isize;

    let mut bottom = FIRST_TIME;
    let mut top = last_time;
    let mut mid_time = (top + bottom)/2;
    let mut result_time: isize = 0;
    loop {
        let result = dijkstra(&Position::new(0, 0, mid_time),
            |p| p.successors(&corruptions, MAP_SIZE, MAP_SIZE, mid_time),
            |p| p.x == GOAL.x && p.y == GOAL.y);
        let previous_mid_time = mid_time;
        if result.is_some() {
            println!("There's a result for mid_time={mid_time}");
            bottom = mid_time;
            mid_time = (top + mid_time) / 2;
        } else {
            println!("No result for mid_time={mid_time}");
            top = mid_time;
            result_time = mid_time;
            mid_time = (mid_time + bottom) / 2;
        }
        if mid_time == previous_mid_time {
            break;
        }
    }
    corruptions.position_to_time.iter()
        .filter(|((_, _), &t)| t == result_time)
        .map(|((x, y), _)| (*x, *y))
        .next().expect("a point")
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_step_forward() {
//         assert_eq!(1, 1);
//     }
// }



