use std::{collections::HashMap, fs};

use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BoardSize {
    size_x: isize,
    size_y: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Speed {
    vx: isize,
    vy: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Robot {
    position: Position,
    speed: Speed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, std::hash::Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl BoardSize {
    const fn new(size_x: isize, size_y: isize) -> BoardSize {
        BoardSize { size_x, size_y }
    }

    fn get_quadrant_of_robot(&self, robot: Robot) -> Quadrant {
        self.get_quadrant(robot.position)
    }
    
    fn get_quadrant(&self, position: Position) -> Quadrant {
        let distance_end_x = position.x.abs_diff(self.size_x) as isize;
        let distance_end_y = position.y.abs_diff(self.size_y) as isize;
        if position.x + 1 == distance_end_x || position.y + 1 == distance_end_y {
            Quadrant::Center
        } else if position.x < distance_end_x && position.y < distance_end_y {
            Quadrant::TopLeft
        } else if position.x < distance_end_x && position.y > distance_end_y {
            Quadrant::BottomLeft
        } else if position.x > distance_end_x && position.y < distance_end_y {
            Quadrant::TopRight
        } else {
            Quadrant::BottomRight
        }
    }

    fn get_robot_count_by_quadrant(&self, robots: &[Robot]) -> HashMap<Quadrant, usize> {
        let mut quadrants_to_count: HashMap<Quadrant, usize> = HashMap::new();
        for robot in robots {
            let quadrant = self.get_quadrant_of_robot(*robot);
            *quadrants_to_count.entry(quadrant).or_insert(0) += 1;
        }
        quadrants_to_count
    }
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    fn add(&self, other: &Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }

    fn add_on_finite_board(&self, other: &Position, board_size: BoardSize) -> Position {
        let new_position = self.add(other);
        let bounded_x = new_position.x % board_size.size_x;
        let bounded_y = new_position.y % board_size.size_y;
        Position {
            x: if bounded_x >= 0 { bounded_x } else { board_size.size_x + bounded_x },
            y: if bounded_y >= 0 { bounded_y } else { board_size.size_y + bounded_y },
        }
    }
}

impl Speed {
    fn new(vx: isize, vy: isize) -> Speed {
        Speed { vx, vy }
    }

    fn get_delta_position(&self, t: isize) -> Position {
        Position::new(self.vx*t, self.vy*t)
    }
}

fn evolve(position: Position, speed: Speed, t: isize, board_size: BoardSize) -> Position {
    position.add_on_finite_board(&speed.get_delta_position(t), board_size)
}

fn evolve_robot(robot: Robot, t: isize, board_size: BoardSize) -> Robot {
    Robot {
        position: evolve(robot.position, robot.speed, t, board_size),
        ..robot
    }
}

fn parse_robots(content: &str) -> Vec<Robot> {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<Robot> = Vec::new();
    for line in content.lines() {
        let robot_info = robot_regex.captures(line).unwrap();
    
        let x = robot_info[1].parse::<isize>().expect("a number");
        let y = robot_info[2].parse::<isize>().expect("a number");
        let vx = robot_info[3].parse::<isize>().expect("a number");
        let vy = robot_info[4].parse::<isize>().expect("a number");

        robots.push(Robot { position: Position::new(x, y), speed: Speed::new(vx, vy) });
    }
    robots
}

pub fn run() -> usize {
    const T: isize = 100;
    const BOARD_SIZE: BoardSize = BoardSize::new(11, 7);
    let filename = "inputs/day14.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let robots = parse_robots(&content);
    let evolved_robots: Vec<Robot> = robots.iter().map(|robot| evolve_robot(*robot, T, BOARD_SIZE)).collect();
    BOARD_SIZE.get_robot_count_by_quadrant(&evolved_robots)
        .iter()
        .filter(|(quadrant, _)| **quadrant != Quadrant::Center)
        .map(|(_, value)| value)
        .product()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolve_single_step() {
        let position = Position::new(0, 0);
        let speed = Speed::new(1, 2);
        let board_size = BoardSize::new(11, 7);
        assert_eq!(Position::new(1,2), evolve(position, speed, 1, board_size));
    }

    #[test]
    fn test_evolve_example() {
        let position = Position::new(2, 4);
        let speed = Speed::new(2, -3);
        let board_size = BoardSize::new(11, 7 );
        assert_eq!(Position::new(4,1), evolve(position, speed, 1, board_size));
        assert_eq!(Position::new(6,5), evolve(position, speed, 2, board_size));
        assert_eq!(Position::new(8,2), evolve(position, speed, 3, board_size));
        assert_eq!(Position::new(10,6), evolve(position, speed, 4, board_size));
        assert_eq!(Position::new(1,3), evolve(position, speed, 5, board_size));
    }

    #[test]
    fn test_get_quadrant_limit() {
        let board_size = BoardSize::new(11, 7);
        assert_eq!(Quadrant::TopLeft, board_size.get_quadrant(Position::new(4, 2)));
        assert_eq!(Quadrant::TopRight, board_size.get_quadrant(Position::new(6, 2)));
        assert_eq!(Quadrant::BottomLeft, board_size.get_quadrant(Position::new(4, 4)));
        assert_eq!(Quadrant::BottomRight, board_size.get_quadrant(Position::new(6, 4)));
    }

    #[test]
    fn test_center() {
        let board_size = BoardSize::new(11, 7);
        assert_eq!(Quadrant::Center, board_size.get_quadrant(Position::new(5, 2)));
        assert_eq!(Quadrant::Center, board_size.get_quadrant(Position::new(4, 3)));
    }
}
