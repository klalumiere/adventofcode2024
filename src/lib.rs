use std::{fs};

const BOX: char = 'O';
const NOTHING: char = '.';
const ROBOT: char = '@';
const WALL: char = '#';


#[derive(Debug)]
struct StaticMap {
    terrain: Vec<Vec<char>>,
}

fn create_maps(content: &str) -> () {
    // const INITIAL_ROBOT_POSITION: (isize, isize) = (-1, -1);
    // let mut terrain: Vec<Vec<char>> = Vec::new();
    // let mut boxes: HashSet<Box> = HashSet::new();
    // let mut robot: (isize, isize) = INITIAL_ROBOT_POSITION;

    // let mut lines = content.lines();
    // for (y, line) in lines.by_ref().enumerate() {
    //     if line.is_empty() {
    //         break;
    //     }
    //     let mut row = Vec::new();
    //     for (x, c) in line.chars().enumerate() {
    //         match c {
    //             BOX => {
    //                 boxes.insert(Box::from_lhs((2*x as isize, y as isize)));
    //                 row.push(NOTHING);
    //                 row.push(NOTHING);
    //             } 
    //             ROBOT => {
    //                 robot = (2*x as isize, y as isize);
    //                 row.push(NOTHING);
    //                 row.push(NOTHING);
    //             }
    //             _ => {
    //                 row.push(c);
    //                 row.push(c);
    //             }
    //         }
    //     }
    //     terrain.push(row);
    // }
    // let moves: String = lines.flat_map(|line| line.chars()).filter(|&c| c != '\n').collect();
    // assert_ne!(robot, INITIAL_ROBOT_POSITION);

    // (StaticMap{terrain}, DynamicMap{robot, boxes }, moves)
}


pub fn run() -> isize {
    let filename = "inputs/day15.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    0
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_move_point() {
//         assert_eq!(1, 1);
//     }
// }
