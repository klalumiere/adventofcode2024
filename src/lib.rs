use std::{collections::HashSet, fs};

use pathfinding::prelude::dijkstra;

const END: char = 'E';
const INITIAL_POSITION: (isize, isize) = (-1, -1);
const NOTHING: char = '.';
const START: char = 'S';
const WALL: char = '#';

#[derive(Clone, Debug)]
struct Labyrinth {
    terrain: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),

    cheats_used: HashSet<Cheat>,
}

impl Labyrinth {
    fn from(content: &str) -> Labyrinth {
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
    
        Labyrinth { terrain, start, end, cheats_used: HashSet::new() }
    }

    fn is_wall(&self, position: (isize, isize)) -> bool {
        let (x, y) = position;
        self.terrain[y as usize][x as usize] == WALL
    }

    fn get_positions_around(&self, position: (isize, isize)) -> impl Iterator<Item = (isize, isize)> + use<'_> {
        let (x, y) = position;
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].into_iter()
            .filter(|(x,y)| *x >= 0 && *y >= 0)
            .filter(|(x,y)| *x < self.terrain[0].len() as isize && *y < self.terrain.len() as isize)
    }

    fn add_cheat(& mut self, cheat: Cheat) {
        self.cheats_used.insert(cheat);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Position {
    value: (isize, isize),
    is_cheating: bool,
    enable_cheat_position: (isize, isize),
    last_exited_wall_position: (isize, isize),
    step_in_cheat: isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Cheat {
    enable_cheat_position: (isize, isize),
    last_exited_wall_position: (isize, isize),
}

impl Position {
    fn new (value: (isize, isize)) -> Self {
        Position {
            value,
            is_cheating: false,
            enable_cheat_position: INITIAL_POSITION,
            last_exited_wall_position: INITIAL_POSITION,
            step_in_cheat: 0,
        }
    }

    fn is_cheating(&self) -> bool {
        self.is_cheating
    }

    fn start_cheating(&self, wall_position: (isize, isize)) -> Self {
        assert!(!self.is_cheating() && self.step_in_cheat == 0);
        Position {
            value: wall_position,
            is_cheating: true,
            enable_cheat_position: self.value,
            last_exited_wall_position: INITIAL_POSITION,
            step_in_cheat: 1,
        }
    }

    fn stop_cheating(&self, new_position: (isize, isize), max_cheat_move: isize) -> Self {
        assert!(self.is_cheating() && self.step_in_cheat == max_cheat_move);
        Position {
            value: new_position,
            is_cheating: false,
            enable_cheat_position: self.enable_cheat_position,
            last_exited_wall_position: self.last_exited_wall_position,
            step_in_cheat: self.step_in_cheat,
        }
    }

    fn move_while_cheating(&self, new_position: (isize, isize), max_cheat_move: isize) -> Self {
        assert!(self.is_cheating() && self.step_in_cheat < max_cheat_move);
        Position {
            value: new_position,
            is_cheating: self.is_cheating,
            enable_cheat_position: self.enable_cheat_position,
            last_exited_wall_position: self.last_exited_wall_position,
            step_in_cheat: self.step_in_cheat + 1,
        }
    }

    fn move_out_of_wall_while_cheating(&self, new_position: (isize, isize), max_cheat_move: isize) -> Self {
        assert!(self.is_cheating() && self.step_in_cheat < max_cheat_move);
        Position {
            value: new_position,
            is_cheating: self.is_cheating,
            enable_cheat_position: self.enable_cheat_position,
            last_exited_wall_position: new_position,
            step_in_cheat: self.step_in_cheat + 1,
        }
    }

    fn move_legally(&self, new_position: (isize, isize), max_cheat_move: isize) -> Self {
        assert!(self.step_in_cheat <= max_cheat_move);
        assert!(!self.is_cheating());
        Position {
            value: new_position,
            is_cheating: self.is_cheating,
            enable_cheat_position: self.enable_cheat_position,
            last_exited_wall_position: self.last_exited_wall_position,
            step_in_cheat: self.step_in_cheat,
        }
    }

    fn move_to(&self, new_position: (isize, isize), labyrinth: &Labyrinth, max_cheat_move: isize) -> Option<Self> {
        if labyrinth.is_wall(new_position) {
            self.move_to_wall(new_position, labyrinth, max_cheat_move)
        } else {
            self.move_to_free_space(new_position, labyrinth, max_cheat_move)
        }
    }

    fn move_to_wall(&self, wall_position: (isize, isize), _: &Labyrinth, max_cheat_move: isize) -> Option<Self> {
        if !self.is_cheating() && self.step_in_cheat == 0 {
            Some(self.start_cheating(wall_position))
        } else if self.step_in_cheat < max_cheat_move {
            Some(self.move_while_cheating(wall_position, max_cheat_move))
        } else {
            None
        }
    }

    fn move_to_free_space(&self, new_position: (isize, isize), labyrinth: &Labyrinth, max_cheat_move: isize) -> Option<Self> {
        if self.is_cheating()  {
            let potential_position = if labyrinth.is_wall(self.value) {
                if self.step_in_cheat < max_cheat_move {
                    Some(self.move_out_of_wall_while_cheating(new_position, max_cheat_move))
                } else {
                    None
                }
            } else if self.step_in_cheat < max_cheat_move {
                Some(self.move_while_cheating(new_position, max_cheat_move))
            } else  {
                Some(self.stop_cheating(new_position, max_cheat_move))
            };
            potential_position.filter(|x|
                !((x.step_in_cheat == max_cheat_move || labyrinth.end == x.value)
                    && labyrinth.cheats_used.contains(&x.get_cheat())))
        } else {
            Some(self.move_legally(new_position, max_cheat_move))
        }
    }

    fn successors(&self, labyrinth: &Labyrinth, max_cheat_move: isize) -> Vec<(Position, usize)> {
        labyrinth.get_positions_around(self.value)
            .filter_map(|new_position|
                self.move_to(new_position, labyrinth, max_cheat_move).map(|new_position| (new_position, 1))
            )
            .collect()
    }

    fn get_cheat(&self) -> Cheat {
        assert!(self.enable_cheat_position != INITIAL_POSITION);
        assert!(self.last_exited_wall_position != INITIAL_POSITION);
        Cheat {
            enable_cheat_position: self.enable_cheat_position,
            last_exited_wall_position: self.last_exited_wall_position,
        }
    }
}

fn solve_dijkstra(labyrinth: &Labyrinth, max_cheat_move: isize) -> Option<(Vec<Position>, usize)> {
    let start = Position::new(labyrinth.start);
    let end = Position::new(labyrinth.end);
    dijkstra(&start, |p| p.successors(labyrinth, max_cheat_move), |p| p.value == end.value)
}

fn calculate_time_saved(original_count: usize, new_count: usize) -> isize {
    (original_count as isize) - (new_count as isize)
}

fn count_cheats_saving_at_least(labyrinth: & mut Labyrinth, min_time_saved: isize, max_cheat_move: isize) -> usize {
    let mut count: usize = 0;
    let (_, original_count) = solve_dijkstra(labyrinth, 0).expect("a solution");
    println!("original count {}", original_count);
    while let Some((last_path, new_count)) = solve_dijkstra(labyrinth, max_cheat_move) {
        let saved = calculate_time_saved(original_count, new_count);
        if saved < min_time_saved {
            break;
        }
        println!("count {}", count);
        println!("saved {}", saved);
        labyrinth.add_cheat(last_path.last().expect("a position").get_cheat());
        count += 1;
    }
    count
}

pub fn run() -> usize {
    const MAX_CHEAT_MOVE: isize = 2;
    const MINIMUM_TIME_SAVED: isize = 100;

    let filename = "inputs/day20.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let mut labyrinth = Labyrinth::from(&content);

    count_cheats_saving_at_least(&mut labyrinth, MINIMUM_TIME_SAVED, MAX_CHEAT_MOVE)
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_turn_to_face_changes_direction() {
//         assert_eq!(1,1);
//     }
// }


// fn solve_labyrinth(reindeer: &Reindeer, labyrinth: &Labyrinth) -> Vec<Reindeer> {
//     let mut reindeer_at_the_end: Vec<Reindeer> = Vec::new();
//     let mut visited: HashSet<ReindeerState> = HashSet::new();
//     let mut to_visit: VecDeque<Reindeer> = VecDeque::new();
//     to_visit.push_back(reindeer.clone());

//     while let Some(reindeer) = to_visit.pop_front() {
//         if reindeer.state.position == labyrinth.end {
//             reindeer_at_the_end.push(reindeer);
//             continue;
//         }
    
//         for new_position in labyrinth.get_legal_positions_around(reindeer.state.position) {
//             let new_state = ReindeerState::new(new_position);
//             if visited.contains(&new_state) {
//                 continue;
//             } else {
//                 visited.insert(new_state);
//             }
//             let new_reindeer = reindeer.clone_with_state(new_state);
//             to_visit.push_back(new_reindeer);
//         }
//     }

//     reindeer_at_the_end
// }
