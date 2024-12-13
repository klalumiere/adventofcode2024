use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, PartialEq, std::hash::Hash)]
enum SideType {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, PartialEq, std::hash::Hash)]
struct Side {
    side_type: SideType,
    point: (usize, usize),
}


impl Side {
    fn new(side_type: SideType, point: (usize, usize)) -> Self {
        Side { side_type, point }
    }
}

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

    fn get_sides(&self, point: (usize, usize)) -> Vec<Side> {
        let (i, j) = point;
        let adjacents = self.get_adjacent_of_same_type(point);
        let mut sides: Vec<Side> = Vec::new();
        if i == 0 || !adjacents.contains(&(i - 1, j)) {
            sides.push(Side::new(SideType::Top, point));
        }
        if i + 1 == self.values.len() ||  !adjacents.contains(&(i + 1, j)) {
            sides.push(Side::new(SideType::Bottom, point));
        }
        if j == 0 || !adjacents.contains(&(i, j - 1)) {
            sides.push(Side::new(SideType::Left, point));
        }
        if j + 1 == self.values[0].len() || !adjacents.contains(&(i, j + 1)) {
            sides.push(Side::new(SideType::Right, point));
        }
        sides
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
    
    #[allow(dead_code)]
    fn get_perimeter(&self, map: &Map) -> usize {
        let mut perimeter = 0usize;
        for point in &self.points {
            perimeter += 4 - map.get_adjacent_of_same_type(*point).len();
        }
        perimeter
    }

    fn get_all_sides(&self, map: &Map) -> HashSet<Side> {
        let mut sides: HashSet<Side> = HashSet::new();
        for point in &self.points {
            sides.extend(map.get_sides(*point));
        }
        sides
    }

    fn get_sides_count(&self, map: &Map) -> usize {
        let all_sides: Vec<Side> = self.get_all_sides(map).iter().copied().collect();
        let mut tops: Vec<Side> = all_sides.iter().copied().filter(|side| side.side_type == SideType::Top).collect();
        let mut bottoms: Vec<Side> = all_sides.iter().copied().filter(|side| side.side_type == SideType::Bottom).collect();
        let mut lefts: Vec<Side> = all_sides.iter().copied().filter(|side| side.side_type == SideType::Left).collect();
        let mut rights: Vec<Side> = all_sides.iter().copied().filter(|side| side.side_type == SideType::Right).collect();
        tops.sort_by(|a,b| {
            if a.point.0 == b.point.0 {
                a.point.1.cmp(&b.point.1)
            } else {
                a.point.0.cmp(&b.point.0)
            }
        });
        bottoms.sort_by(|a,b| {
            if a.point.0 == b.point.0 {
                a.point.1.cmp(&b.point.1)
            } else {
                a.point.0.cmp(&b.point.0)
            }
        });
        lefts.sort_by(|a,b| {
            if a.point.1 == b.point.1 {
                a.point.0.cmp(&b.point.0)
            } else {
                a.point.1.cmp(&b.point.1)
            }
        });
        rights.sort_by(|a,b| {
            if a.point.1 == b.point.1 {
                a.point.0.cmp(&b.point.0)
            } else {
                a.point.1.cmp(&b.point.1)
            }
        });
        
        let mut reduced: Vec<Side> = Vec::new();
        
        let mut last_side: Option<Side> = None;
        for side in tops {
            if let Some(last) = last_side {
                if last.point.0 == side.point.0 && last.point.1.abs_diff(side.point.1) == 1 {
                    last_side = Some(side);
                    continue;
                }
            }
            last_side = Some(side);
            reduced.push(side);
        }

        let mut last_side: Option<Side> = None;
        for side in bottoms {
            if let Some(last) = last_side {
                if last.point.0 == side.point.0 && last.point.1.abs_diff(side.point.1) == 1 {
                    last_side = Some(side);
                    continue;
                }
            }
            last_side = Some(side);
            reduced.push(side);
        }

        let mut last_side: Option<Side> = None;
        for side in lefts {
            if let Some(last) = last_side {
                if last.point.1 == side.point.1 && last.point.0.abs_diff(side.point.0) == 1 {
                    last_side = Some(side);
                    continue;
                }
            }
            last_side = Some(side);
            reduced.push(side);
        }

        let mut last_side: Option<Side> = None;
        for side in rights {
            if let Some(last) = last_side {
                if last.point.1 == side.point.1 && last.point.0.abs_diff(side.point.0) == 1 {
                    last_side = Some(side);
                    continue;
                }
            }
            last_side = Some(side);
            reduced.push(side);
        }

        reduced.len()
    }

    fn get_area(&self) -> usize {
        self.points.len()
    }

    #[allow(dead_code)]
    fn get_price(&self, map: &Map) -> usize {
        self.get_area() * self.get_perimeter(map)
    }

    fn get_price_by_side(&self, map: &Map) -> usize {
        self.get_area() * self.get_sides_count(map)
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
    regions.iter().map(|region| region.get_price_by_side(&map)).sum()
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_count_digits() {
//         assert_eq!(1, 1);
//     }
// }
