use std::{collections::{HashMap, HashSet, VecDeque}, fs};

const COST_TO_STEP: usize = 1;
const COST_TO_TURN: usize = 1000;

const END: char = 'E';
const NOTHING: char = '.';
const START: char = 'S';
const WALL: char = '#';


#[derive(Clone, Debug)]
struct Labyrinth {
    terrain: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
}

impl Labyrinth {
    fn from(content: &str) -> Labyrinth {
        const INITIAL_POSITION: (isize, isize) = (-1, -1);
        let mut terrain: Vec<Vec<char>> = Vec::new();
        let mut start = INITIAL_POSITION;
        let mut end = INITIAL_POSITION;
    
        let mut lines = content.lines();
        for (y, line) in lines.by_ref().enumerate() {
            if line.is_empty() {
                break;
            }
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    START => {
                        start = (x as isize, y as isize);
                        row.push(NOTHING);
                    }
                    END => {
                        end = (x as isize, y as isize);
                        row.push(NOTHING);
                    } 
                    _ => {
                        row.push(c);
                    }
                }
            }
            terrain.push(row);
        }
        assert_ne!(end, INITIAL_POSITION);
        assert_ne!(start, INITIAL_POSITION);
    
        Labyrinth { terrain, start, end }
    }

    fn get_positions_around(&self, reindeer: &Reindeer) -> Vec<(isize, isize)> {
        let (x, y) = reindeer.position;
        let mut around: Vec<(isize, isize)> = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].iter()
            .copied()
            .filter(|&(x,y)| x >= 0 && y >= 0)
            .filter(|&(x,y)| x < self.terrain[0].len() as isize && y < self.terrain.len() as isize)
            .filter(|&(x,y)| self.terrain[y as usize][x as usize] != WALL)
            .collect();
        around.sort_by_key(|a| reindeer.score_move(*a));
        around
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct VisitedHistory {
    position: (isize, isize),
    direction: Direction,
}

#[derive(Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Reindeer {
    position: (isize, isize),
    direction: Direction,
    score: usize,

    history: Vec<(isize, isize)>,
}

impl Reindeer {
    fn new(position: (isize, isize)) -> Self {
        let history: Vec<(isize, isize)> = vec![position];
        Reindeer { position, direction: Direction::Right, score: 0, history }
    }

    fn score_moves(&self) -> usize {
        self.score
    }

    fn step_forward(&mut self) -> &Self {
        let delta = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        self.position = (self.position.0 + delta.0, self.position.1 + delta.1);
        self.history.push(self.position);
        self.score += COST_TO_STEP;
        self
    }

    fn turn_to_face(&mut self, position_to_face: (isize, isize)) -> &Self {
        self.turn(self.get_new_direction(position_to_face))
    }

    fn turn(&mut self, new_direction: Direction) -> &Self {
        self.score += self.score_turn(new_direction);
        self.direction = new_direction;
        self
    }

    fn get_turn_len(&self, new_direction: Direction) -> usize {
        if new_direction == self.direction {
            return 0;
        }
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Right) => 1,
            (Direction::Up, Direction::Left) => 1,
            (Direction::Up, Direction::Down) => 2,
            (Direction::Down, Direction::Right) => 1,
            (Direction::Down, Direction::Left) => 1,
            (Direction::Down, Direction::Up) => 2,
            (Direction::Left, Direction::Up) => 1,
            (Direction::Left, Direction::Down) => 1,
            (Direction::Left, Direction::Right) => 2,
            (Direction::Right, Direction::Up) => 1,
            (Direction::Right, Direction::Down) => 1,
            (Direction::Right, Direction::Left) => 2,
            _ => panic!("Invalid turn"),
        }
    }

    fn get_new_direction(&self, position_to_face: (isize, isize)) -> Direction {
        assert!(self.position.0.abs_diff(position_to_face.0) + self.position.1.abs_diff(position_to_face.1) == 1);
        let delta_x = position_to_face.0 - self.position.0;
        let delta_y = position_to_face.1 - self.position.1;
        if delta_x < 0 {
            Direction::Left
        } else if delta_x > 0 {
            Direction::Right
        } else if delta_y < 0 {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    fn score_move(&self, new_position: (isize, isize)) -> usize {
        self.score_turn(self.get_new_direction(new_position)) + COST_TO_STEP
    }

    fn score_turn(&self, new_direction: Direction) -> usize {
        self.get_turn_len(new_direction)*COST_TO_TURN
    }
}

fn solve_labyrinth(reindeer: &Reindeer, labyrinth: &Labyrinth) -> Vec<Reindeer> {
    let mut visited_to_score: HashMap<VisitedHistory, usize> = HashMap::new();
    let mut reindeer_at_the_end: Vec<Reindeer> = Vec::new();
    let mut to_visit: VecDeque<Reindeer> = VecDeque::new();
    to_visit.push_back(reindeer.clone());

    while let Some(reindeer) = to_visit.pop_front() {
        if reindeer.position == labyrinth.end {
            reindeer_at_the_end.push(reindeer);
            continue;
        }
        if reindeer_at_the_end.iter().any(|r| r.score_moves() < reindeer.score_moves()) {
            continue;
        }
    
        for new_position in labyrinth.get_positions_around(&reindeer.clone()) {
            if reindeer.history.contains(&new_position) {
                continue;
            }
            let mut new_reindeer = reindeer.clone();
            new_reindeer.turn_to_face(new_position);
            new_reindeer.step_forward();
            let visited_history = VisitedHistory { position: new_reindeer.position, direction: new_reindeer.direction };
            if let Some(&score) = visited_to_score.get(&visited_history) {
                if score < new_reindeer.score_moves() {
                    continue;
                }
            }
            visited_to_score.insert(visited_history, new_reindeer.score_moves());
            to_visit.push_back(new_reindeer);
        }
    }

    reindeer_at_the_end
}


pub fn run() -> usize {
    let filename = "inputs/day16.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let labyrinth = Labyrinth::from(&content);
    let reindeer = Reindeer::new(labyrinth.start);
    let reindeer_at_the_end: Vec<Reindeer> = solve_labyrinth(&reindeer, &labyrinth);
    let min_socre = reindeer_at_the_end.iter().map(Reindeer::score_moves).min().unwrap();
    let tiles_visited: HashSet<(isize, isize)> = HashSet::from_iter(reindeer_at_the_end.iter()
        .filter(|r| r.score_moves() == min_socre)
        .flat_map(|r| r.history.clone()));
    tiles_visited.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_to_face_changes_direction() {
        let mut reindeer = Reindeer::new((1,1));
        assert_eq!(Direction::Right, reindeer.turn_to_face((2,1)).direction);
        assert_eq!(Direction::Left, reindeer.turn_to_face((0,1)).direction);
        assert_eq!(Direction::Up, reindeer.turn_to_face((1,0)).direction);
        assert_eq!(Direction::Down, reindeer.turn_to_face((1,2)).direction);
    }

    #[test]
    fn test_step_forward() {
        let mut reindeer = Reindeer::new((1,1));
        assert_eq!((2,1), reindeer.step_forward().position);
    }
}
