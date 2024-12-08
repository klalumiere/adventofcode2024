use std::{collections::{HashMap, HashSet}, fs};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Board {
    antennas: HashMap<char, Vec<Point>>,
    len_x: isize,
    len_y: isize,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn subtract(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    fn is_bounded(&self, board: &Board) -> bool {
        self.x >= 0 && self.x < board.len_x && self.y >= 0 && self.y < board.len_y
    }
}

impl Board {
    fn from(char_matrix: &[Vec<char>]) -> Board {
        let mut antennas = HashMap::new();
        for (i, row) in char_matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                antennas.entry(*c).or_insert(Vec::new())
                    .push(Point { x: j as isize, y: i as isize });
            }
        }
        Board {
            antennas,
            len_x: char_matrix[0].len() as isize,
            len_y: char_matrix.len() as isize,
        }
    }
}

fn find_antinodes(board: &Board) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    for points in board.antennas.values() {
        for p0 in points {
            for p in points {
                if p0 == p {
                    continue;
                }
                let antinode_position = p0.add(&p0.subtract(&p));
                if antinode_position.is_bounded(board) {
                    antinodes.insert(antinode_position);
                }
            }
        }
    }
    antinodes
}

fn find_antinodes_part_2(board: &Board) -> HashSet<Point> {
    let mut antinodes: HashSet<Point>  = HashSet::new();
    for points in board.antennas.values() {
        for p0 in points {
            for p in points {
                if p0 == p {
                    continue;
                }
                antinodes.insert(*p);
                let delta = p0.subtract(p);
                let mut antinode_position = p0.add(&delta);
                loop {
                    if antinode_position.is_bounded(board) {
                        antinodes.insert(antinode_position);
                        antinode_position = antinode_position.add(&delta);
                    } else {
                        break;
                    }
                }
                
            }
        }
    }
    antinodes
}

#[allow(dead_code)]
fn day8_part1() -> usize {
    let filename = "inputs/day8.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let char_matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let board = Board::from(&char_matrix);
    find_antinodes(&board).len()
}

#[allow(dead_code)]
fn day8_part2() -> usize {
    let filename = "inputs/day8.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let char_matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let board = Board::from(&char_matrix);
    find_antinodes_part_2(&board).len()
}

fn main() {
    let result = day8_part2();
    println!("result={result}");
}
