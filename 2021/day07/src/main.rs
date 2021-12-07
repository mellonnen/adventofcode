use std::collections::HashMap;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("\n{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n * r), where n = number of crabs, and r is the length of the range (max(crabs) - min(crabs))
fn part1(input: &str) -> i64 {
    // parse input, and sort
    let mut crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    crabs.sort();

    // loop over the range to minimize the total fuel
    let mut fuel = i64::MAX;
    let start = *crabs.first().unwrap();
    let stop = *crabs.last().unwrap();
    for i in start..stop {
        let f = crabs.iter().map(|c| (c - i).abs()).sum();
        fuel = fuel.min(f);
    }
    fuel
}
// O(n * r^2)
//NOTE: This is the worst case, amortized it is probably better due to the memoization.
fn part2(input: &str) -> i64 {
    let mut crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    crabs.sort();

    let mut fuel = i64::MAX;
    let start = *crabs.first().unwrap();
    let stop = *crabs.last().unwrap();
    // memoize the fuel cost for different distances.
    let mut memo: HashMap<i64, i64> = HashMap::new();

    for i in start..stop {
        let mut f = 0;
        for c in &crabs {
            let d = (c - i).abs();

            // check if we already have calculated this cost.
            if let Some(x) = memo.get(&d) {
                f += x;
            } else {
                // calculate and memoize cost.
                let x: i64 = (1..(d + 1)).sum();
                f += x;
                memo.insert(d, x);
            }
        }
        fuel = fuel.min(f);
    }
    fuel
}
