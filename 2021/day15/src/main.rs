use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// State used in Dijkstra.
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (i64, i64),
    cost: i64,
}

// Ord and PartialOrd implemented so we can use the binary heap effectively.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip order of comparisons.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// Dijkstra's shortest paths algorithm implementation.
// As |V| = n^2 (where **n** is the side of the square) and |E| = O(4*n^2),
// We will get the time complexity O((|V| + |E|) * log(|V|) = O(n^2 * log(n^2)).
fn dijsktra(
    map: HashMap<(i64, i64), i64>,
    mut dist: HashMap<(i64, i64), i64>,
    goal: (i64, i64),
) -> i64 {
    let mut heap = BinaryHeap::new();

    // relax and push start position.
    *dist.get_mut(&(0, 0)).unwrap() = 0;
    heap.push(State {
        cost: 0, // do not count first position.
        position: (0, 0),
    });

    while let Some(State {
        position: (y, x),
        cost,
    }) = heap.pop()
    {
        // We found a path!
        if (y, x) == goal {
            return cost;
        }

        // Check if we have already found a better path.
        if cost > *dist.get(&(y, x)).unwrap() {
            continue;
        }

        // Loop over all the possible moves, to see if we can find a way,
        // That lowers the cost.
        for (dy, dx) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_pos = (y + dy, x + dx);
            // edge check.
            if let Some(c) = map.get(&next_pos) {
                let next = State {
                    position: next_pos,
                    cost: cost + c,
                };
                // Check if this way lowers the cost
                if next.cost < *dist.get(&next_pos).unwrap() {
                    // Relax and push position.
                    *dist.get_mut(&next_pos).unwrap() = next.cost;
                    heap.push(next);
                }
            }
        }
    }
    0
}

// O(n^2)
fn part1(input: &str) -> i64 {
    // parse inputs.
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut dist: HashMap<(i64, i64), i64> = HashMap::new();
    let mut goal = (0, 0); // track the size of the grid.

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.char_indices() {
            map.insert((y as i64, x as i64), c.to_string().parse().unwrap());
            dist.insert((y as i64, x as i64), i64::MAX);
            goal.1 = x as i64;
        }
        goal.0 = y as i64;
    }
    // calculate the lowest cost path.
    dijsktra(map, dist, goal)
}

fn part2(input: &str) -> i64 {
    // parse the inputs.
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut dist: HashMap<(i64, i64), i64> = HashMap::new();
    let mut shape = (0, 0);

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.char_indices() {
            map.insert((y as i64, x as i64), c.to_string().parse().unwrap());
            dist.insert((y as i64, x as i64), i64::MAX);
            shape.1 = x as i64;
        }
        shape.0 = y as i64;
    }
    // update shape size.
    shape = (shape.0 + 1, shape.0 + 1);

    // Extend the map.
    for y in 0..shape.0 * 5 {
        for x in 0..shape.1 * 5 {
            // Skip the "original" square.
            if map.contains_key(&(y, x)) {
                continue;
            }

            let mut cost = if let Some(c) = map.get(&(y, x - shape.1)) {
                // horizontal.
                c + 1
            } else {
                // vertical.
                map.get(&(y - shape.0, x)).unwrap() + 1
            };

            // wrap cost.
            if cost > 9 {
                cost = 1;
            }
            map.insert((y, x), cost);
            dist.insert((y, x), i64::MAX);
        }
    }

    let goal = (shape.0 * 5 - 1, shape.1 * 5 - 1);
    dijsktra(map, dist, goal)
}
