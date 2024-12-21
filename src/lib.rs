use std::{collections::{HashMap, HashSet}, fs};

use pathfinding::prelude::dijkstra;

const END: char = 'E';
const INITIAL_POSITION: (i32, i32) = (-1, -1);
const NOTHING: char = '.';
const START: char = 'S';
const WALL: char = '#';

#[derive(Clone, Debug)]
struct Labyrinth {
    terrain: Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
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
                        start = (x as i32, y as i32);
                        row.push(NOTHING);
                    }
                    END => {
                        end = (x as i32, y as i32);
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

    fn is_wall(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;
        self.terrain[y as usize][x as usize] == WALL
    }

    fn get_positions_around(&self, position: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + use<'_> {
        let (x, y) = position;
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].into_iter()
            .filter(|(x,y)| *x >= 0 && *y >= 0)
            .filter(|(x,y)| *x < self.terrain[0].len() as i32 && *y < self.terrain.len() as i32)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Position {
    value: (i32, i32),
    step_in_cheat: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct PositionState {
    value: (i32, i32),
    is_cheating: bool,
}

impl Position {
    fn new (value: (i32, i32)) -> Self {
        Position {
            value,
            step_in_cheat: 0,
        }
    }

    fn is_cheating(&self, max_cheat_move: i32) -> bool {
        self.step_in_cheat > 0 && self.step_in_cheat < max_cheat_move
    }

    fn get_position_state(&self, max_cheat_move: i32) -> PositionState {
        PositionState {
            value: self.value,
            is_cheating: self.is_cheating(max_cheat_move),
        }
    }

    fn move_while_cheating(&self, new_position: (i32, i32)) -> Self {
        Position {
            value: new_position,
            step_in_cheat: self.step_in_cheat + 1,
        }
    }

    fn move_legally(&self, new_position: (i32, i32)) -> Self {
        Position {
            value: new_position,
            step_in_cheat: self.step_in_cheat,
        }
    }

    fn move_to(&self, new_position: (i32, i32), labyrinth: &Labyrinth, max_cheat_move: i32) -> Option<Self> {
        if labyrinth.is_wall(new_position) {
            self.move_to_wall(new_position, labyrinth, max_cheat_move)
        } else {
            self.move_to_free_space(new_position, labyrinth, max_cheat_move)
        }
    }

    fn move_to_wall(&self, wall_position: (i32, i32), _: &Labyrinth, max_cheat_move: i32) -> Option<Self> {
        if self.step_in_cheat + 1 < max_cheat_move {
            Some(self.move_while_cheating(wall_position))
        } else {
            None
        }
    }

    fn move_to_free_space(&self, new_position: (i32, i32), labyrinth: &Labyrinth, max_cheat_move: i32) -> Option<Self> {
        if self.is_cheating(max_cheat_move) {
            Some(self.move_while_cheating(new_position))
        } else if labyrinth.is_wall(self.value) {
            None
        } else { 
            Some(self.move_legally(new_position))
        }
    }

    fn successors(&self, labyrinth: &Labyrinth, max_cheat_move: i32) -> Vec<(Position, usize)> {
        labyrinth.get_positions_around(self.value)
            .filter_map(|new_position|
                self.move_to(new_position, labyrinth, max_cheat_move).map(|new_position| (new_position, 1))
            )
            .collect()
    }

    fn successors_stack<'a>(&'a self, labyrinth: &'a Labyrinth, max_cheat_move: i32) -> impl Iterator<Item = Position> + 'a {
        labyrinth.get_positions_around(self.value)
            .filter_map(move |new_position|
                self.move_to(new_position, labyrinth, max_cheat_move))
    }
}

fn solve_dijkstra(labyrinth: &Labyrinth, max_cheat_move: i32) -> Option<(Vec<Position>, usize)> {
    let start = Position::new(labyrinth.start);
    let end = Position::new(labyrinth.end);
    dijkstra(&start, |p| p.successors(labyrinth, max_cheat_move), |p| p.value == end.value)
}

fn count_cheats_saving_at_least(labyrinth: & Labyrinth, min_time_saved: i32, max_cheat_move: i32) -> usize {
    let (_, original_count) = solve_dijkstra(labyrinth, 0).expect("a solution");
    println!("original count {}", original_count);
    let max_cost = (original_count as i32) - min_time_saved;
    let visited: HashSet<(i32, i32)> = HashSet::new();
    let mut cache: HashMap<CacheKey, usize> = HashMap::new();
    count_cheats(Position::new(labyrinth.start), labyrinth, max_cheat_move, 0, max_cost, & visited, & mut cache)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct CacheKey {
    position: PositionState,
    actual_cost: i32,
}

fn count_cheats(start: Position, labyrinth: &Labyrinth, max_cheat_move: i32, actual_cost: i32, max_cost: i32,
    visited: &HashSet<(i32, i32)>, cache: & mut HashMap<CacheKey, usize>) -> usize
{
    if start.value == labyrinth.end {
        return 1;
    }
    if actual_cost >= max_cost {
        return 0;
    }
    if let Some(count) = cache.get(&CacheKey { position: start.get_position_state(max_cheat_move), actual_cost }) {
        return *count;
    }

    let mut total_count: usize = 0;
    for successor in start.successors_stack(labyrinth, max_cheat_move) {
        if visited.contains(&successor.value) {
            continue;
        }
        let mut cloned_visited = visited.clone();
        cloned_visited.insert(start.value);
        let successor_count = count_cheats(successor, labyrinth, max_cheat_move, actual_cost + 1, max_cost, &cloned_visited, cache);
        total_count += successor_count;
    }

    cache.insert(CacheKey { position: start.get_position_state(max_cheat_move), actual_cost}, total_count);
    total_count
}

pub fn run() -> usize {
    const MAX_CHEAT_MOVE: i32 = 2;
    const MINIMUM_TIME_SAVED: i32 = 40;

    let filename = "inputs/day20.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let labyrinth = Labyrinth::from(&content);

    count_cheats_saving_at_least(&labyrinth, MINIMUM_TIME_SAVED, MAX_CHEAT_MOVE)
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_turn_to_face_changes_direction() {
//         assert_eq!(1,1);
//     }
// }
