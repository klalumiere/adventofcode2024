use std::{collections::HashSet, fs};

#[derive(Clone, Debug)]
struct Board {
    matrix: Vec<Vec<usize>>,
    starting_positions: Vec<(usize, usize)>
}

#[derive(
    Clone,
    Debug,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
)]
struct Path {
    steps: Vec<(usize, usize)>,
    value: usize,
}

impl Board {
    fn from(content: &str) -> Board {
        const RADIX: u32 = 10;
        let matrix: Vec<Vec<usize>> = content.lines().map(|line| {
            line.chars().map(|c| c.to_digit(RADIX).expect("expected digits") as usize).collect()
        }).collect();

        let mut starting_positions: Vec<(usize, usize)> = Vec::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value == 0 {
                    starting_positions.push((i, j));
                }
            }
        }

        Board {
            matrix,
            starting_positions
        }
    }
}

impl Path {
    fn new() -> Path {
        Path { steps: Vec::new(), value: 0 }
    }

    fn append(mut self, step: (usize, usize), value: usize) -> Self {     
        self.steps.push(step);
        self.value = value;
        self
    }

    fn get_next_steps(self: &Path, board: &Board) -> Vec<(usize, usize)> {
        let mut next_steps: Vec<(usize, usize)> = Vec::new();
        if self.value == 9 {
            return next_steps
        }

        let (i, j) = *self.steps.last().unwrap();
        let value = board.matrix[i][j];
        let is_higher = |k: usize, l: usize| board.matrix[k][l] == value + 1;
        if i > 0 && is_higher(i - 1, j) {
            next_steps.push((i - 1, j));
        }
        if j > 0 && is_higher(i, j - 1) {
            next_steps.push((i, j - 1));
        }
        if i < board.matrix.len() - 1 && is_higher(i + 1, j) {
            next_steps.push((i + 1, j));
        }
        if j < board.matrix[i].len() - 1 && is_higher(i, j + 1) {
            next_steps.push((i, j + 1));
        }
        next_steps
    }
}



fn walk(path: &Path, board: &Board, paths: & mut HashSet<Path>) { // Danger: possible Stack Overflow!
    let next_steps = path.get_next_steps(board);
    if next_steps.is_empty() {
        paths.insert(path.clone());
        return;
    }

    for next_step in next_steps {
        let new_path = path.clone().append(next_step, path.value + 1);
        walk(&new_path, board, paths);
    }
}

fn find_path(board: &Board) -> HashSet<Path> {
    let mut paths: HashSet<Path> = HashSet::new();
    for (i, j) in &board.starting_positions {
        let path = Path::new().append((*i, *j), 0);
        walk(&path, board, & mut paths);
    }
    paths
}

#[allow(dead_code)]
fn day10_part1() -> usize {
    let filename = "inputs/day10.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let board = Board::from(&content);
    let paths = find_path(&board);
    paths.iter()
        .filter(|trailhead| {
            assert!(trailhead.steps.len() <= 10);
            trailhead.steps.len() == 10
        })
        .map(|trailhead| {
            let (a, b) = trailhead.steps.first().unwrap();
            let (i, j) = trailhead.steps.last().unwrap();
            assert!(board.matrix[*a][*b] == 0);
            assert!(board.matrix[*i][*j] == 9);
            ((*a, *b), (*i, *j))
        })
        .collect::<HashSet<((usize, usize), (usize, usize))>>()
        .len()
}

fn main() {
    let result = day10_part1();
    println!("result={result}");
}
