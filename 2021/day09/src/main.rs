use std::collections::{HashMap, HashSet};

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n^2) where n is the length of the side of the map.
fn part1(input: &str) -> i64 {
    // read input into our map.
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((y as i64, x as i64), c.to_string().parse().unwrap());
        }
    }
    // loop over each position and check if the current position is
    // lower then all its neighbors.
    let mut lowpoints: Vec<i64> = Vec::new();

    for ((y, x), v) in &map {
        let mut lowest = true;
        for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(vv) = map.get(&(y + dy, x + dx)) {
                if v >= vv {
                    lowest = false;
                    break;
                }
            }
        }

        if lowest {
            lowpoints.push(*v);
        }
    }

    // calculate score
    lowpoints.into_iter().map(|x| x + 1).sum()
}

// O(n^2), as the worst case is one large basin containing all elements.
fn part2(input: &str) -> usize {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((y as i64, x as i64), c.to_string().parse().unwrap());
        }
    }

    // recursive flood fill function that will recurse
    // until we hit the end of the basins.
    fn flood_fill(
        (y, x): (i64, i64),
        basin: &mut HashSet<(i64, i64)>,
        map: &HashMap<(i64, i64), i64>,
    ) {
        // base cases. Edge of basin, and already seen element.
        if *map.get(&(y, x)).unwrap() == 9 || basin.contains(&(y, x)) {
            return ();
        }

        basin.insert((y, x));
        // for all neighbors, call flood_fill again.
        for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if map.contains_key(&(y + dy, x + dx)) {
                flood_fill((y + dy, x + dx), basin, map);
            }
        }
    }

    // set to keep track of elements that already have been seen.
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    // top 3 largest basins.
    let mut basins: [usize; 3] = [0; 3];

    // loop over all positions on map, and call flood_fill on them
    // if they are not 9s or have already been seen.
    for (pos, v) in &map {
        if seen.contains(&pos) || *v == 9 {
            continue;
        }
        // create a empty basin and call recursion with it.
        let mut basin: HashSet<(i64, i64)> = HashSet::new();
        flood_fill(*pos, &mut basin, &map);

        // add the elements of the basin to seen.
        for p in &basin {
            seen.insert(*p);
        }

        // if there is a smaller basin saved, swap it out.
        let min_index = basins
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, _)| i)
            .unwrap();

        if basins[min_index] < basin.len() {
            basins[min_index] = basin.len();
        }
    }

    basins.iter().product()
}
