use std::collections::{HashMap, HashSet};

use colour::cyan;

const SIMULATIONS: usize = 100;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// helper function to print the grid.
fn dump_grid(grid: &HashMap<(i64, i64), i64>) {
    let n = (grid.len() as f64).sqrt() as i64;
    for y in 0..n {
        for x in 0..n {
            let energy = grid.get(&(y, x)).unwrap();
            if *energy == 0 {
                cyan!(" {} ", energy);
            } else {
                print!(" {} ", energy);
            }
        }
        print!("\n");
    }
    print!("\n");
}

/*
    It is a Game of life(like) simulation, where in each iteration the following steps
    happens in order.

   1. first increase energy level by one for each octopus
   2. if an octopus has energy > 9 it **flashes** -> increase all adjacent  octopuses by one.
       This may cause a chain reaction of flashes, but a octopus can only flash once per iteration.
   3. All octopuses that **flash** have their energy reset to zero.
*/
fn part1(input: &str) -> usize {
    // Parse inputs into the grid.
    let mut grid: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((y as i64, x as i64), c.to_string().parse().unwrap());
        }
    }
    println!("Before simulation.\n");
    dump_grid(&grid);

    let mut score = 0;
    let mut i = 1; // just for selective printing.

    // run simulation.
    for _ in 0..SIMULATIONS {
        // set to keep track of which octopuses that have flashed.
        let mut flashed: HashSet<(i64, i64)> = HashSet::new();

        // stack of flashed octopuses to process.
        let mut stack: Vec<(i64, i64)> = Vec::new();

        // Step 1.
        for (pos, energy) in grid.iter_mut() {
            *energy += 1;
            if *energy > 9 {
                flashed.insert(*pos);
                stack.push(*pos);
            }
        }
        // Step 2.
        // pop next octopus from stack and simulate the flash, until stack is empty.
        while !stack.is_empty() {
            let (y, x) = stack.pop().unwrap();
            for dy in -1..2 {
                for dx in -1..2 {
                    // has already been flashed.
                    if flashed.contains(&(y + dy, x + dx)) {
                        continue;
                    }
                    if let Some(energy) = grid.get_mut(&(y + dy, x + dx)) {
                        *energy += 1;
                        if *energy > 9 {
                            flashed.insert((y + dy, x + dx));
                            stack.push((y + dy, x + dx));
                        }
                    }
                }
            }
        }

        // Step 3.
        for pos in &flashed {
            let energy = grid.get_mut(&pos).unwrap();
            *energy = 0;
        }

        // print grid.
        if i <= 10 || i % 10 == 0 {
            println!("iteration {}\n", i);
            dump_grid(&grid);
        }
        score += flashed.len();
        i += 1;
    }
    score
}

// Same simulation as before, but we run it until the octopuses are synchronized.
fn part2(input: &str) -> usize {
    let mut grid: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((y as i64, x as i64), c.to_string().parse().unwrap());
        }
    }
    println!("Before simulation.\n");
    dump_grid(&grid);

    let mut i = 1; // current iteration.
    loop {
        let mut flashed: HashSet<(i64, i64)> = HashSet::new();
        let mut stack: Vec<(i64, i64)> = Vec::new();

        // Step 1.
        for (pos, energy) in grid.iter_mut() {
            *energy += 1;
            if *energy > 9 {
                flashed.insert(*pos);
                stack.push(*pos);
            }
        }

        // Step 2.
        while !stack.is_empty() {
            let (y, x) = stack.pop().unwrap();
            for dy in -1..2 {
                for dx in -1..2 {
                    if flashed.contains(&(y + dy, x + dx)) {
                        continue;
                    }
                    if let Some(energy) = grid.get_mut(&(y + dy, x + dx)) {
                        *energy += 1;
                        if *energy > 9 {
                            flashed.insert((y + dy, x + dx));
                            stack.push((y + dy, x + dx));
                        }
                    }
                }
            }
        }

        // Step 3.
        for pos in &flashed {
            let energy = grid.get_mut(&pos).unwrap();
            *energy = 0;
        }

        // Print grid.
        if i <= 10 || i % 10 == 0 {
            println!("iteration {}\n", i);
            dump_grid(&grid);
        }

        // check for synchronization.
        if flashed.len() == grid.len() {
            println!("iteration {}(all flash!)\n", i);
            dump_grid(&grid);
            break;
        }
        i += 1;
    }
    i
}
