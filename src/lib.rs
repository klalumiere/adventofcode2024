use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Map {
    values: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Region {
    points: HashSet<(usize, usize)>,
}

impl Map {
    fn from(content: &str) -> Self {
        let values: Vec<Vec<char>> = content.lines()
            .map(|line| line.chars().collect())
            .collect();
        Map { values }
    }

    fn get_adjacent(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = point;
        let mut adjacent: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            adjacent.push((x - 1, y));
        }
        if x +1 < self.values.len() {
            adjacent.push((x + 1, y));
        }
        if y > 0 {
            adjacent.push((x, y - 1));
        }
        if y + 1 < self.values[0].len() {
            adjacent.push((x, y + 1));
        }
        adjacent
    }

    fn get_adjacent_of_same_type(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = point;
        self.get_adjacent(point).into_iter().filter(|(i, j)| self.values[*i][*j] == self.values[x][y]).collect()
    }

    fn get_regions(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = Vec::new();
        for i in 0..self.values.len() {
            for j in 0..self.values[0].len() {
                let point = (i, j);
                if regions_contain(&regions, point) {
                    continue;
                }
                let mut region = Region::new(point);
                let mut points_to_explore = self.get_adjacent_of_same_type(point);
                let mut points_already_explored = vec![point];
                while let Some(current) = points_to_explore.pop() {
                    if points_already_explored.contains(&current) || regions_contain(&regions, current) {
                        continue;
                    }
                    if !region.contains(current) {
                        region.points.insert(current);
                        let adjacent = self.get_adjacent_of_same_type(current);
                        points_to_explore.extend(adjacent);
                    }
                    points_already_explored.push(current);
                }
                regions.push(region);
            }
        }
        regions
    }
}

impl Region {
    fn new(point: (usize, usize)) -> Self {
        let mut points = HashSet::new();
        points.insert(point);
        Region {
            points,
        }
    }

    fn contains(&self, point: (usize, usize)) -> bool {
        self.points.contains(&point)
    }

    fn get_perimeter(&self, map: &Map) -> usize {
        let mut sorted_points: Vec<(usize, usize)> = self.points.iter().copied().collect();
        let mut perimeter = 0usize;
        sorted_points.sort();
        for point in sorted_points {
            perimeter += 4 - map.get_adjacent_of_same_type(point).len();
        }
        perimeter
    }

    fn get_area(&self) -> usize {
        self.points.len()
    }

    fn get_price(&self, map: &Map) -> usize {
        self.get_area() * self.get_perimeter(map)
    }
}

fn regions_contain(regions: &[Region], point: (usize, usize)) -> bool {
    regions.iter().any(|region| region.contains(point))
}

pub fn run() -> usize {
    let filename = "inputs/day12.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let map = Map::from(&content);
    let regions = map.get_regions();
    regions.iter().map(|region| region.get_price(&map)).sum()
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_count_digits() {
//         assert_eq!(1, 1);
//     }
// }
