use std::collections::HashSet;

fn main() {
    let in_str = include_str!("../input");
    let (p1, p2) = solve(in_str);
    println!("PART1\n=====\n");
    println!("{}", p1);
    println!("\nPART2\n=====\n");
    println!("{}", p2);
}

// One generation update of the cellular automaton (game of life).
// As alg[0] = # and alg[512] = . the image will alternate between having an initiate number of on/off
// pixels. Therefore, we alternate tracking **on** pixels and **of**  pixels using a set.
// If on=true, all the pixels in the set are **on** (everything else is **off**),
// and if on=false, all the pixels in the set are **off** (everything else is **on**).
fn update(grid: &HashSet<(i64, i64)>, alg: [bool; 512], on: bool) -> HashSet<(i64, i64)> {
    let mut g: HashSet<(i64, i64)> = HashSet::new();
    // Calculate the bounding box of the pixels we are tracking.
    let ymax = *grid.iter().map(|(y, _)| y).max().unwrap();
    let ymin = *grid.iter().map(|(y, _)| y).min().unwrap();
    let xmax = *grid.iter().map(|(_, x)| x).max().unwrap();
    let xmin = *grid.iter().map(|(_, x)| x).min().unwrap();

    // Loop over the bounding box with one-pixel slack.
    for y in ymin - 1..=ymax + 1 {
        for x in xmin - 1..=xmax + 1 {
            let mut bin = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    bin *= 2;
                    if grid.contains(&(y + dy, x + dx)) == on {
                        // we want '#' to contribute a 1 in the binary number.
                        bin += 1;
                    }
                }
            }

            // track the off pixels in the next iteration
            // if we are currently tracking on pixels, and vice versa.
            if alg[bin] != on {
                g.insert((y, x));
            }
        }
    }

    g
}

// Solves both parts by running a cellular automaton simulation for varying length.
fn solve(input: &str) -> (usize, usize) {
    let mut alg: [bool; 512] = [false; 512];
    let mut grid: HashSet<(i64, i64)> = HashSet::new();

    let (alg_str, grid_str) = input.split_once("\n\n").unwrap();

    // Create the algorithm lookup.
    for (i, c) in alg_str.char_indices() {
        alg[i] = c == '#';
    }
    // Create grid.
    for (y, line) in grid_str.trim().lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '#' {
                grid.insert((y as i64, x as i64));
            }
        }
    }

    // run simulation.
    let mut p1 = 0;
    for i in 0..50 {
        if i == 2 {
            p1 = grid.len();
        }
        grid = update(&grid, alg, i % 2 == 0);
    }
    (p1, grid.len())
}
