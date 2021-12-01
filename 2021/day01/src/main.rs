fn main() {
    println!("PART 1\n=====\n");
    println!("{}\n", part1());
    println!("PART 2\n=====\n");
    println!("{}\n", part2());
}

// O(n)
fn part1() -> i64 {
    let in_str = include_str!("../input");
    let input: Vec<i64> = in_str
        .trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut increase = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increase += 1;
        }
    }
    increase
}

// O(n)
fn part2() -> i64 {
    let in_str = include_str!("../input");
    let input: Vec<i64> = in_str
        .trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut increase = 0;
    for i in 3..input.len() {
        if input[i] > input[i - 3] {
            increase += 1;
        }
    }
    increase
}
