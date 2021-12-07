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
    let crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    // loop over the range to minimize the total fuel
    let mut fuel = i64::MAX;
    let start = *crabs.iter().min().unwrap();
    let stop = *crabs.iter().max().unwrap();
    for i in start..stop {
        let f = crabs.iter().map(|c| (c - i).abs()).sum();
        fuel = fuel.min(f);
    }
    fuel
}
// O(n * r)
fn part2(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut fuel = i64::MAX;
    let start = *crabs.iter().min().unwrap();
    let stop = *crabs.iter().max().unwrap();
    for i in start..stop {
        let mut f = 0;
        for c in &crabs {
            let d = (c - i).abs();
            f += d * (d + 1) / 2
        }
        fuel = fuel.min(f);
    }
    fuel
}
