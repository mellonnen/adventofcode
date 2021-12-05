use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("\n{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n).
fn part1(input: &str) -> usize {
    let mut segments: Vec<((usize, usize), (usize, usize))> = Vec::new();
    let mut plane: HashMap<(usize, usize), usize> = HashMap::new();

    // parse input.
    for line in input.trim().lines() {
        let mut iter = line.split(" -> ").map(|s| {
            let mut split = s.split(",");
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        });
        segments.push((iter.next().unwrap(), iter.next().unwrap()));
    }

    // loop over segments and update plane.
    for ((x1, y1), (x2, y2)) in segments {
        if x1 == x2 {
            let from = min(y1, y2);
            let to = max(y1, y2);

            for y in from..=to {
                if let Some(val) = plane.get_mut(&(x1, y)) {
                    *val += 1;
                } else {
                    plane.insert((x1, y), 1);
                }
            }
        } else if y1 == y2 {
            let from = min(x1, x2);
            let to = max(x1, x2);

            for x in from..=to {
                if let Some(val) = plane.get_mut(&(x, y1)) {
                    *val += 1;
                } else {
                    plane.insert((x, y1), 1);
                }
            }
        }
    }

    plane.values().into_iter().filter(|v| **v > 1).count()
}

fn part2(input: &str) -> usize {
    let mut segments: Vec<((i64, i64), (i64, i64))> = Vec::new();
    let mut plane: HashMap<(i64, i64), i64> = HashMap::new();

    // parse inputs.
    for line in input.trim().lines() {
        let mut iter = line.split(" -> ").map(|s| {
            let mut split = s.split(",");
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        });
        segments.push((iter.next().unwrap(), iter.next().unwrap()));
    }

    // update plane.
    for ((x1, y1), (x2, y2)) in segments {
        // calculate x and y directions.
        let x = (x2 - x1).signum();
        let y = (y2 - y1).signum();
        let mut current = (x1, y1);
        // loop until +x and +y so we do not miss the last iteration.
        while current != (x2 + x, y2 + y) {
            if let Some(val) = plane.get_mut(&current) {
                *val += 1;
            } else {
                plane.insert(current, 1);
            }
            current.0 += x;
            current.1 += y;
        }
    }

    plane.values().into_iter().filter(|v| **v > 1).count()
}
