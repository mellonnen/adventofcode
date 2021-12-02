fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("\n{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n)
fn part1(in_str: &str) -> i64 {
    // Split string on whitespace, will result in a vector: ["up","5","down","1",...]
    let input: Vec<&str> = in_str.trim().split_whitespace().collect();
    let mut horizontal = 0;
    let mut vertical = 0;

    // loop in steps of 2, handling each command every iteration.
    for i in (1..input.len()).step_by(2) {
        let command = input[i - 1];
        let amount: i64 = input[i].parse().unwrap();
        match command {
            "down" => vertical += amount,
            "up" => vertical -= amount,
            "forward" => horizontal += amount,
            _ => break,
        }
    }
    vertical * horizontal
}

// O(n)
fn part2(in_str: &str) -> i64 {
    let input: Vec<&str> = in_str.trim().split_whitespace().collect();

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for i in (1..input.len()).step_by(2) {
        let command = input[i - 1];
        let amount: i64 = input[i].parse().unwrap();
        match command {
            "down" => aim += amount,
            "up" => aim -= amount,
            "forward" => {
                horizontal += amount;
                vertical += aim * amount;
            }
            _ => break,
        }
    }

    vertical * horizontal
}
