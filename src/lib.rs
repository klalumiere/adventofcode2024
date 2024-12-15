use std::{collections::HashSet, fmt, fs};

const BOX: char = 'O';
const NOTHING: char = '.';
const ROBOT: char = '@';
const WALL: char = '#';

const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const UP: char = '^';

#[derive(Debug)]
struct StaticMap {
    terrain: Vec<Vec<char>>,
}

impl fmt::Display for StaticMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.terrain {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct DynamicMap {
    boxes: HashSet<Box>,
    robot: (isize, isize),
}

#[derive(Clone, Copy, Debug, std::hash::Hash, std::cmp::PartialEq, std::cmp::Eq)]
struct Box {
    lhs: (isize, isize),
    rhs: (isize, isize),
}

impl DynamicMap {
    fn get_box(&self, position: (isize, isize)) -> Vec<Box> {
        let a_box = Box::from_lhs(position);
        if self.boxes.contains(&a_box) {
            return vec![a_box];
        }
        let another_box = Box::from_rhs(position);
        if self.boxes.contains(&another_box) {
            return vec![another_box];
        }
        vec![]
    }

    fn get_boxes(&self, positions: Vec<(isize, isize)>) -> HashSet<Box> {
        let mut result: HashSet<Box> = HashSet::new();
        for position in positions {
            result.extend(self.get_box(position));
        }
        result
    }

    fn move_box(& mut self, a_box: Box, direction: char) {
        let new_lhs = move_point(a_box.lhs, direction);
        self.boxes.remove(&a_box);
        self.boxes.insert(Box::from_lhs(new_lhs));
    }

    fn get_total_gps_coordinates(&self) -> isize {
        self.boxes.iter().map(Box::get_gps_coordinates).sum()
    }
}

impl Box {
    fn from_lhs((x, y): (isize, isize)) -> Self {
        Self { lhs: (x, y), rhs: (x + 1, y) }
    }

    fn from_rhs((x, y): (isize, isize)) -> Self {
        Self { lhs: (x - 1, y), rhs: (x, y) }
    }

    fn move_box(&self, direction: char) -> Self {
        let new_lhs = move_point(self.lhs, direction);
        let new_rhs = move_point(self.rhs, direction);
        Self { lhs: new_lhs, rhs: new_rhs }
    }

    fn get_gps_coordinates(&self) -> isize {
        let (x, y) = self.lhs;
        const Y_AXIS_MULTIPLE: isize = 100;
        Y_AXIS_MULTIPLE * y + x
    }

    fn get_ovelapping_coordinates(&self) -> Vec<(isize, isize)> {
        vec![self.lhs, self.rhs]
    }
}

fn create_maps(content: &str) -> (StaticMap, DynamicMap, String) {
    const INITIAL_ROBOT_POSITION: (isize, isize) = (-1, -1);
    let mut terrain: Vec<Vec<char>> = Vec::new();
    let mut boxes: HashSet<Box> = HashSet::new();
    let mut robot: (isize, isize) = INITIAL_ROBOT_POSITION;

    let mut lines = content.lines();
    for (y, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                BOX => {
                    boxes.insert(Box::from_lhs((2*x as isize, y as isize)));
                    row.push(NOTHING);
                    row.push(NOTHING);
                } 
                ROBOT => {
                    robot = (2*x as isize, y as isize);
                    row.push(NOTHING);
                    row.push(NOTHING);
                }
                _ => {
                    row.push(c);
                    row.push(c);
                }
            }
        }
        terrain.push(row);
    }
    let moves: String = lines.flat_map(|line| line.chars()).filter(|&c| c != '\n').collect();
    assert_ne!(robot, INITIAL_ROBOT_POSITION);

    (StaticMap{terrain}, DynamicMap{robot, boxes }, moves)
}

fn move_point(point: (isize, isize), direction: char) -> (isize, isize) {
    match direction {
        DOWN => (point.0, point.1 + 1),
        LEFT => (point.0 - 1, point.1),
        RIGHT => (point.0 + 1, point.1),
        UP => (point.0, point.1 - 1),
        _ => panic!("Invalid direction: {}", direction),
    }
}

fn move_robot(dynamic_map: & mut DynamicMap, static_map: &StaticMap, direction: char) {
    let new_position = move_point(dynamic_map.robot, direction);
    if static_map.terrain[new_position.1 as usize][new_position.0 as usize] == WALL {
        return;
    }
    if let Some(a_box) = dynamic_map.get_box(new_position).first() {
        let mut boxes: HashSet<Box> = HashSet::new();
        boxes.insert(*a_box);
        let box_moved = move_boxes(&boxes, dynamic_map, static_map, direction);
        if !box_moved {
            return;
        }
    }
    dynamic_map.robot = new_position;
}

fn move_boxes(boxes: &HashSet<Box>, dynamic_map: & mut DynamicMap, static_map: &StaticMap, direction: char) -> bool {
    let new_coordinates: Vec<(isize, isize)> = boxes.iter()
        .map(|a_box| a_box.move_box(direction))
        .flat_map(|a_box: Box| Box::get_ovelapping_coordinates(&a_box))
        .collect();
    if new_coordinates.iter().any(|&point| static_map.terrain[point.1 as usize][point.0 as usize] == WALL) {
        return false;
    }
    let new_boxes: HashSet<Box> = dynamic_map.get_boxes(new_coordinates).iter().copied()
        .filter(|a_box| !boxes.contains(a_box)).collect();
    if ! new_boxes.is_empty() {
        let boxes_moved = move_boxes(&new_boxes, dynamic_map, static_map, direction);
        if !boxes_moved {
            return false;
        }
    }
    for a_box in boxes {
        dynamic_map.move_box(*a_box, direction);
    }
    true
    // true
}

fn evolve(dynamic_map: & mut DynamicMap, static_map: &StaticMap, moves: &str) {
    for direction in moves.chars() {
        move_robot(dynamic_map, static_map, direction);
    }
}

pub fn run() -> isize {
    let filename = "inputs/day15.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let (static_map, dynamic_map, moves) = create_maps(&content);
    let mut dynamic_map = dynamic_map.clone();
    evolve(&mut dynamic_map, &static_map, &moves);
    dynamic_map.get_total_gps_coordinates()
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_move_point() {
//         assert_eq!(1, 1);
//     }
// }
