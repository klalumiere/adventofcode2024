use std::{collections::{HashSet, VecDeque}, fs};

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

    fn get_legal_positions_around(&self, position: (isize, isize)) -> Vec<(isize, isize)> {
        let (x, y) = position;
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].iter()
            .copied()
            .filter(|&(x,y)| x >= 0 && y >= 0)
            .filter(|&(x,y)| x < self.terrain[0].len() as isize && y < self.terrain.len() as isize)
            .filter(|&(x,y)| self.terrain[y as usize][x as usize] != WALL)
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
    step_taken: usize
    // history: Vec<ReindeerState>,
}

impl ReindeerState {
    fn new(position: (isize, isize)) -> Self {
        ReindeerState { position }
    }
}

impl Reindeer {
    fn new(position: (isize, isize)) -> Self {
        let state = ReindeerState::new(position);
        // let history: Vec<ReindeerState> = vec![state];
        Reindeer { state, step_taken: 0 }
    }

    fn clone_with_state(&self, state: ReindeerState) -> Self {
        let mut clone = self.clone();
        clone.state = state;
        clone.step_taken += 1;
        // clone.history.push(state);
        clone
    }
}

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

fn get_labyrinth_solution_cost(reindeer: &Reindeer, labyrinth: &Labyrinth) -> usize {
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

    reindeer_at_the_end.iter().map(|x| x.step_taken ).min().expect("a path")
}


pub fn run() -> usize {
    const MINIMUM_TIME_SAVED: usize = 100;

    let filename = "inputs/day20.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let labyrinth = Labyrinth::from(&content);
    let reindeer = Reindeer::new(labyrinth.start);
    let len_min_path = get_labyrinth_solution_cost(&reindeer, &labyrinth);
    let mut path_count = 0;
    for (y, row) in labyrinth.terrain.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let position = (x as isize, y as isize);
            if labyrinth.is_shortcut(position) {
                println!("shortcut at position: {:?}", position);
                let mut labyrinth_with_shortcut = labyrinth.clone();
                labyrinth_with_shortcut.terrain[y][x] = NOTHING;
                let len_with_shortcut = get_labyrinth_solution_cost(&reindeer, &labyrinth_with_shortcut);
                if len_min_path - len_with_shortcut >= MINIMUM_TIME_SAVED {
                    path_count += 1;
                }
            }
        }
    }
    path_count
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_to_face_changes_direction() {
        assert_eq!(1,1);
    }
}
