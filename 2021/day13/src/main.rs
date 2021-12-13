use std::collections::HashSet;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// Print the paper.
fn dump_paper(grid: &HashSet<(usize, usize)>) -> String {
    let max_x = *grid.iter().map(|(_, x)| x).max().unwrap();
    let max_y = *grid.iter().map(|(y, _)| y).max().unwrap();
    let mut s = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if grid.contains(&(y, x)) {
                s.push_str(" # ");
            } else {
                s.push_str(" . ");
            }
        }
        s.push_str("\n");
    }
    s.push_str("\n");
    s
}

fn part1(input: &str) -> usize {
    let mut paper: HashSet<(usize, usize)> = HashSet::new();
    let mut lines = input.trim().lines();

    // parse all the dots on the paper.
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let (x, y): (usize, usize) = line
            .split_once(",")
            .map(|(i, k)| (i.parse().unwrap(), k.parse().unwrap()))
            .unwrap();
        paper.insert((y, x));
    }

    // apply the fold once.
    let (dir, fold): (&str, usize) = lines
        .next()
        .unwrap()
        .strip_prefix("fold along ")
        .unwrap()
        .split_once("=")
        .map(|(d, v)| (d, v.parse().unwrap()))
        .unwrap();

    // calculate dimensions.
    let max_x = *paper.iter().map(|(_, x)| x).max().unwrap();
    let max_y = *paper.iter().map(|(y, _)| y).max().unwrap();

    // loop over the dots to be folded and fold them.
    if dir == "y" {
        for y in fold..=max_y {
            for x in 0..=max_x {
                if paper.remove(&(y, x)) {
                    // This is a simplification of the equation,
                    // fold - (y - fold) = fold - y + fold = 2 * fold - y.
                    paper.insert((2 * fold - y, x));
                }
            }
        }
    } else {
        for y in 0..=max_y {
            for x in fold..=max_x {
                if paper.remove(&(y, x)) {
                    paper.insert((y, 2 * fold - x));
                }
            }
        }
    }

    paper.len()
}

fn part2(input: &str) -> String {
    let mut paper: HashSet<(usize, usize)> = HashSet::new();
    let mut lines = input.trim().lines();

    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let (x, y): (usize, usize) = line
            .split_once(",")
            .map(|(i, k)| (i.parse().unwrap(), k.parse().unwrap()))
            .unwrap();

        paper.insert((y, x));
    }

    // apply all folds.
    for line in lines {
        let (dir, fold): (&str, usize) = line
            .strip_prefix("fold along ")
            .unwrap()
            .split_once("=")
            .map(|(d, v)| (d, v.parse().unwrap()))
            .unwrap();

        let max_x = *paper.iter().map(|(_, x)| x).max().unwrap();
        let max_y = *paper.iter().map(|(y, _)| y).max().unwrap();
        if dir == "y" {
            for y in fold..=max_y {
                for x in 0..=max_x {
                    if paper.remove(&(y, x)) {
                        paper.insert((2 * fold - y, x));
                    }
                }
            }
        } else {
            for y in 0..=max_y {
                for x in fold..=max_x {
                    if paper.remove(&(y, x)) {
                        paper.insert((y, 2 * fold - x));
                    }
                }
            }
        }
    }

    // return the string representation of the paper.
    dump_paper(&paper)
}
