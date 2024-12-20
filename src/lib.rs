use std::{collections::{HashSet, VecDeque}, fs};

use pathfinding::prelude::dijkstra;

const END: char = 'E';
const NOTHING: char = '.';
const START: char = 'S';
const WALL: char = '#';

#[derive(Clone, Debug)]
struct Labyrinth {
    terrain: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),

    prohibited_cheats: HashSet<Cheat>,
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
    
        Labyrinth { terrain, start, end, prohibited_cheats: HashSet::new() }
    }

    fn get_legal_positions_around(&self, position: (isize, isize)) -> Vec<(isize, isize)> {
        self.get_positions_around(position).iter()
            .copied()
            .filter(|&(x,y)| self.terrain[y as usize][x as usize] != WALL)
            .collect()
    }

    fn get_positions_around(&self, position: (isize, isize)) -> Vec<(isize, isize)> {
        let (x, y) = position;
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].iter()
            .copied()
            .filter(|&(x,y)| x >= 0 && y >= 0)
            .filter(|&(x,y)| x < self.terrain[0].len() as isize && y < self.terrain.len() as isize)
            .collect()
    }

    fn is_shortcut(&self, position: (isize, isize)) -> bool {
        let (x, y) = position;
        self.terrain[y as usize][x as usize] == WALL
            && self.get_legal_positions_around(position).len() > 1
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct ReindeerState {
    position: (isize, isize),
}

#[derive(Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Reindeer {
    state: ReindeerState,
    history: Vec<ReindeerState>,
}

impl ReindeerState {
    fn new(position: (isize, isize)) -> Self {
        ReindeerState { position }
    }
}

impl Reindeer {
    fn new(position: (isize, isize)) -> Self {
        let state = ReindeerState::new(position);
        let history: Vec<ReindeerState> = vec![state];
        Reindeer { state, history }
    }

    fn clone_with_state(&self, state: ReindeerState) -> Self {
        let mut clone = self.clone();
        clone.state = state;
        clone.history.push(state);
        clone
    }
}

fn solve_labyrinth(reindeer: &Reindeer, labyrinth: &Labyrinth) -> Vec<Reindeer> {
    let mut reindeer_at_the_end: Vec<Reindeer> = Vec::new();
    let mut visited: HashSet<ReindeerState> = HashSet::new();
    let mut to_visit: VecDeque<Reindeer> = VecDeque::new();
    to_visit.push_back(reindeer.clone());

    while let Some(reindeer) = to_visit.pop_front() {
        if reindeer.state.position == labyrinth.end {
            reindeer_at_the_end.push(reindeer);
            continue;
        }
    
        for new_position in labyrinth.get_legal_positions_around(reindeer.state.position) {
            let new_state = ReindeerState::new(new_position);
            if visited.contains(&new_state) {
                continue;
            } else {
                visited.insert(new_state);
            }
            let new_reindeer = reindeer.clone_with_state(new_state);
            to_visit.push_back(new_reindeer);
        }
    }

    reindeer_at_the_end
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Position {
    x: isize,
    y: isize,
    using_cheat: bool,
    cheat_steps: isize,
    entering_cheat_position: (isize, isize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, std::hash::Hash)]
struct Cheat {
    entering_cheat_position: (isize, isize),
    exited_cheat_position: (isize, isize),
}

impl Cheat {
    fn new(entering_cheat_position: (isize, isize), exited_cheat_position: (isize, isize)) -> Self {
        Cheat { entering_cheat_position, exited_cheat_position }
    }
}

impl Position {
    fn new(x: isize, y: isize, using_cheat: bool, cheat_steps: isize, entering_cheat_position: (isize, isize)) -> Self {
        Position { x, y, using_cheat, cheat_steps, entering_cheat_position }
    }

    fn from_tuple(position: (isize, isize)) -> Self {
        Position::new(position.0, position.1, false, 0, (-1, -1))
    }

    fn successors(&self, labyrinth: &Labyrinth, max_cheat_steps: isize) -> Vec<(Position, usize)> {
        let cheat_steps = if self.using_cheat { self.cheat_steps + 1 } else { self.cheat_steps };
        let mut successors: Vec<(Position, usize)>  = labyrinth.get_legal_positions_around((self.x, self.y))
            .iter()
            .filter(|&&(x, y)| !self.using_cheat
                || !labyrinth.prohibited_cheats.contains(&Cheat::new(self.entering_cheat_position, (x,y))))
            .map(|&(x, y)| (Position::new(x, y, false, cheat_steps, (-1, -1)), 1))
            .collect();
        if cheat_steps < max_cheat_steps {
            let entering_cheat_position = if self.using_cheat {
                self.entering_cheat_position
            } else {
                (self.x, self.y)
            };
            let cheat_steps = if ! self.using_cheat { self.cheat_steps + 1 } else { cheat_steps };
            let wall_successors: Vec<(Position, usize)>  = labyrinth.get_positions_around((self.x, self.y))
                .iter()
                .filter(|&&(x,y)| labyrinth.terrain[y as usize][x as usize] == WALL)
                .map(|&(x, y)| {
                    (Position::new(x, y, true, cheat_steps, entering_cheat_position), 1)
                })
                .collect();
            successors.extend(wall_successors);
        }
        successors
      }
}

fn solve_dijkstra(labyrinth: &Labyrinth, max_cheat_steps: isize) -> Option<(Vec<Position>, usize)> {
    let start = Position::from_tuple(labyrinth.start);
    let end = Position::from_tuple(labyrinth.end);
    dijkstra(&start, |p| p.successors(labyrinth, max_cheat_steps),
        |p| p.x == end.x && p.y == end.y)
}

fn get_dijkstra_count(labyrinth: &Labyrinth, max_cheat_steps: isize) -> Option<usize> {
    solve_dijkstra(labyrinth, max_cheat_steps).map(|(_, count)| count)
}

fn ghost_distance(rhs: (isize, isize), lhs: (isize, isize)) -> isize {
    let (x1, y1) = rhs;
    let (x2, y2) = lhs;
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as isize
}

pub fn run() -> usize {
    const MINIMUM_TIME_SAVED: isize = 76;

    let filename = "inputs/day20.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let labyrinth = Labyrinth::from(&content);

    // let reindeer = Reindeer::new(labyrinth.start);
    // let reindeer_at_the_end: Vec<Reindeer> = solve_labyrinth(&reindeer, &labyrinth);
    // reindeer_at_the_end[0].history.len()

    let (_, count) = solve_dijkstra(&labyrinth, 0).expect("a path");
    println!("without cheats, count={:?}", count);

    let mut labyrinth_with_cheats  = labyrinth.clone();
    let mut saved_enough: usize = 0;
    let mut last_run_saved_enough = true;
    const MAX_CHEAT_STEPS: isize = 20;
    while let Some((path, new_count)) = solve_dijkstra(&labyrinth_with_cheats, MAX_CHEAT_STEPS) {
        if !last_run_saved_enough {
            break;
        }
        let saved = (count as isize) - (new_count as isize);
        if saved >= MINIMUM_TIME_SAVED {
            let mut last_position: (isize, isize) = (0,0);
            let mut entered_cheat_position: Option<(isize, isize)> = None;
            let mut exited_cheat_position: Option<(isize, isize)> = None;
            for position in path.iter() {
                let (x, y) = (position.x, position.y);
                if position.using_cheat {
                    if entered_cheat_position.is_none() {
                        entered_cheat_position = Some(last_position);
                    }
                } else if entered_cheat_position.is_some() && exited_cheat_position.is_none() {
                    exited_cheat_position = Some((position.x, position.y));
                }
                last_position = (position.x, position.y);
            }
            println!("saved={:?}", saved);
            println!("path={:?}", path);
            saved_enough += 1;
            labyrinth_with_cheats.prohibited_cheats.insert(Cheat::new
                (entered_cheat_position.expect("entered_cheat_position"),
                exited_cheat_position.expect("exited_cheat_position")));
        } else {
            last_run_saved_enough = false;
        }
    }
    saved_enough
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_turn_to_face_changes_direction() {
//         assert_eq!(1,1);
//     }
// }
