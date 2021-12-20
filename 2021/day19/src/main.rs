use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use itertools::Itertools;

fn main() {
    let in_str = include_str!("../input");
    let (p1, p2) = solve(in_str);
    println!("PART1\n=====\n");
    println!("{}", p1);
    println!("\nPART2\n=====\n");
    println!("{}", p2);
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point(i64, i64, i64);

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Point {
    // given a id, this function calculates a rotation of the point.
    fn rotate(&self, id: usize) -> Self {
        match id {
            0 => Point(self.0, self.1, self.2),

            1 => Point(self.0, -self.2, self.1),
            2 => Point(self.0, -self.1, -self.2),
            3 => Point(self.0, self.2, -self.1),
            4 => Point(-self.0, -self.1, self.2),
            5 => Point(-self.0, -self.2, -self.1),
            6 => Point(-self.0, self.1, -self.2),
            7 => Point(-self.0, self.2, self.1),

            8 => Point(self.1, self.0, -self.2),
            9 => Point(self.1, -self.0, self.2),
            10 => Point(self.1, self.2, self.0),
            11 => Point(self.1, -self.2, -self.0),
            12 => Point(-self.1, self.0, self.2),
            13 => Point(-self.1, -self.0, -self.2),
            14 => Point(-self.1, -self.2, self.0),
            15 => Point(-self.1, self.2, -self.0),

            16 => Point(self.2, self.0, self.1),
            17 => Point(self.2, -self.0, -self.1),
            18 => Point(self.2, -self.1, self.0),
            19 => Point(self.2, self.1, -self.0),
            20 => Point(-self.2, self.0, -self.1),
            21 => Point(-self.2, -self.0, self.1),
            22 => Point(-self.2, self.1, self.0),
            23 => Point(-self.2, -self.1, -self.0),
            _ => unreachable!(),
        }
    }

    fn inverse(&self) -> Self {
        Point(-self.0, -self.1, -self.2)
    }

    fn manhattan_distance(&self, other: Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) as usize
    }
}

fn parse_inputs(input: &str) -> Vec<HashSet<Point>> {
    let mut v: Vec<HashSet<Point>> = Vec::new();
    let split = input.split("\n\n");

    for s in split {
        let mut grid: HashSet<Point> = HashSet::new();
        let mut lines = s.trim().lines();
        lines.next();
        for line in lines {
            let pos: (i64, i64, i64) = line
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            grid.insert(Point(pos.0, pos.1, pos.2));
        }
        v.push(grid);
    }

    v
}

// Solves part 1 and 2 by brute-force.
fn solve(input: &str) -> (usize, usize) {
    let mut scanners = parse_inputs(input);
    let mut ocean: HashSet<Point> = HashSet::new();
    let mut scanner_positions: Vec<Point> = Vec::new();

    // we map all points in relation to scanner 0 orientation.
    ocean.extend(scanners.remove(0));
    scanner_positions.push(Point(0, 0, 0));

    // we loop over the scanners until we have mapped all of them.
    while scanners.len() > 0 {
        // loop in reverse order so we can remove scanners without trouble.
        'outer: for i in (0..scanners.len()).rev() {
            // try all 24 rotations.
            for rot in 0..24 {
                // track the offset of different known_points and the rotated points from the scanner.
                let mut offests: HashMap<Point, usize> = HashMap::new();
                for known_point in &ocean {
                    for p in &scanners[i] {
                        let offset = p.rotate(rot) - known_point.clone(); // calculate offset with vector subtraction.
                        *offests.entry(offset).or_insert(0) += 1; // count the offset.
                    }
                }

                for (offset, count) in offests {
                    // if an offset has occurred more then 12 times, we know that a mapped scanner is adjacent.
                    if count >= 12 {
                        // the position of the scanner is the inverse of the offset.
                        let scanner = offset.inverse();
                        scanner_positions.push(scanner.clone());

                        // Rotate and translate all the points in the scanner.
                        for p in &scanners[i] {
                            let mapped = p.rotate(rot) + scanner.clone();
                            ocean.insert(mapped);
                        }

                        // we have mapped this scanner now, therefore we can discard it and continue.
                        scanners.remove(i);
                        continue 'outer;
                    }
                }
            }
        }
    }
    let p1 = ocean.len();
    let mut p2 = 0;
    for i in 0..scanner_positions.len() {
        for j in 0..scanner_positions.len() {
            p2 = p2.max(scanner_positions[i].manhattan_distance(scanner_positions[j].clone()));
        }
    }

    (p1, p2)
}

#[test]
fn test_all() {
    let test = include_str!("../sample");
    let (p1, p2) = solve(test);
    assert_eq!(p1, 79);
    assert_eq!(p2, 3621);
}
