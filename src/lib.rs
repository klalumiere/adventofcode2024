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
    boxes: HashSet<(isize, isize)>,
    robot: (isize, isize),
}

fn create_maps(content: &str) -> (StaticMap, DynamicMap, String) {
    const INITIAL_ROBOT_POSITION: (isize, isize) = (-1, -1);
    let mut terrain: Vec<Vec<char>> = Vec::new();
    let mut boxes: HashSet<(isize, isize)> = HashSet::new();
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
                    boxes.insert((x as isize, y as isize));
                    row.push(NOTHING);
                } 
                ROBOT => {
                    robot = (x as isize, y as isize);
                    row.push(NOTHING);
                }
                _ => { row.push(c); }
            }
        }
        terrain.push(row);
    }
    let moves: String = lines.flat_map(|line| line.chars()).filter(|&c| c != '\n').collect();
    assert_ne!(robot, INITIAL_ROBOT_POSITION);

    (StaticMap{terrain}, DynamicMap{boxes, robot}, moves)
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
    if dynamic_map.boxes.contains(&new_position) {
        let box_moved = move_box(new_position, dynamic_map, static_map, direction);
        if !box_moved {
            return;
        }
    }
    dynamic_map.robot = new_position;
}

fn move_box(position: (isize, isize), dynamic_map: & mut DynamicMap, static_map: &StaticMap, direction: char) -> bool {
    let new_position = move_point(position, direction);
    if static_map.terrain[new_position.1 as usize][new_position.0 as usize] == WALL {
        return false;
    }
    if dynamic_map.boxes.contains(&new_position) {
        let box_moved = move_box(new_position, dynamic_map, static_map, direction);
        if !box_moved {
            return false;
        }
    }
    dynamic_map.boxes.remove(&position);
    dynamic_map.boxes.insert(new_position);
    true
}

fn evolve(dynamic_map: & mut DynamicMap, static_map: &StaticMap, moves: &str) {
    for direction in moves.chars() {
        move_robot(dynamic_map, static_map, direction);
    }
}

fn get_gps_coordinates((x, y): &(isize, isize)) -> isize {
    const Y_AXIS_MULTIPLE: isize = 100;
    Y_AXIS_MULTIPLE * y + x
}

pub fn run() -> isize {
    let filename = "inputs/day15.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let (static_map, dynamic_map, moves) = create_maps(&content);
    let mut dynamic_map = dynamic_map.clone();
    evolve(&mut dynamic_map, &static_map, &moves);
    dynamic_map.boxes.iter().map(get_gps_coordinates).sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_point() {
        let point = (1, 1);
        assert_eq!(move_point(point, LEFT), (0, 1));
        assert_eq!(move_point(point, RIGHT), (2, 1));
        assert_eq!(move_point(point, UP), (1, 0));
        assert_eq!(move_point(point, DOWN), (1, 2));
    }

    #[test]
    fn test_move_robot_dont_hit_wall() {
        let static_map = StaticMap{terrain: vec![vec![WALL, NOTHING, WALL]]};
        let mut dynamic_map = DynamicMap{boxes: HashSet::new(), robot: (1, 0)};
        move_robot(&mut dynamic_map, &static_map, LEFT);
        assert_eq!(dynamic_map.robot, (1, 0));
    }

    #[test]
    fn test_move_robot_moves() {
        let static_map = StaticMap{terrain: vec![vec![NOTHING, NOTHING, WALL]]};
        let mut dynamic_map = DynamicMap{boxes: HashSet::new(), robot: (1, 0)};
        move_robot(&mut dynamic_map, &static_map, LEFT);
        assert_eq!(dynamic_map.robot, (0, 0));
    }

    #[test]
    fn test_move_robot_moves_box() {
        let static_map = StaticMap{terrain: vec![vec![NOTHING, NOTHING, NOTHING]]};
        let mut dynamic_map = DynamicMap{boxes: HashSet::new(), robot: (2, 0)};
        dynamic_map.boxes.insert((1, 0));
        move_robot(&mut dynamic_map, &static_map, LEFT);
        assert_eq!(dynamic_map.robot, (1, 0));
        assert!(dynamic_map.boxes.contains(&(0, 0)));
        assert!(!dynamic_map.boxes.contains(&(1, 0)));
    }

    #[test]
    fn test_move_robot_moves_boxes() {
        let static_map = StaticMap{terrain: vec![vec![NOTHING, NOTHING, NOTHING, NOTHING]]};
        let mut dynamic_map = DynamicMap{boxes: HashSet::new(), robot: (3, 0)};
        dynamic_map.boxes.insert((2, 0));
        dynamic_map.boxes.insert((1, 0));
        move_robot(&mut dynamic_map, &static_map, LEFT);
        assert_eq!(dynamic_map.robot, (2, 0));
        assert_eq!(dynamic_map.boxes.len(), 2);
        assert!(dynamic_map.boxes.contains(&(1, 0)));
        assert!(dynamic_map.boxes.contains(&(0, 0)));
    }

    #[test]
    fn test_move_robot_moves_boxes_doesnt_replace_wall() {
        let static_map = StaticMap{terrain: vec![vec![WALL, NOTHING, NOTHING, NOTHING]]};
        let mut dynamic_map = DynamicMap{boxes: HashSet::new(), robot: (3, 0)};
        dynamic_map.boxes.insert((2, 0));
        dynamic_map.boxes.insert((1, 0));
        move_robot(&mut dynamic_map, &static_map, LEFT);
        assert_eq!(dynamic_map.robot, (3, 0));
        assert_eq!(dynamic_map.boxes.len(), 2);
        assert!(dynamic_map.boxes.contains(&(2, 0)));
        assert!(dynamic_map.boxes.contains(&(1, 0)));
    }
}
