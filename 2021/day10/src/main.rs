use std::collections::HashMap;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n)
fn part1(input: &str) -> i64 {
    // Lookup tables for scores and pairs.
    let scores: HashMap<char, i64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut score = 0;
    for line in input.trim().lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            // check if we are dealing with a closing char.
            if let Some(s) = scores.get(&c) {
                // correct char.
                if stack.last().unwrap() == pairs.get(&c).unwrap() {
                    stack.pop();
                    continue;
                }
                // incorrect char.
                score += *s;
                break;
            }
            stack.push(c);
        }
    }
    score
}
// O(n)
fn part2(input: &str) -> i64 {
    let score_map: HashMap<char, i64> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let mut scores: Vec<i64> = Vec::new();

    for line in input.trim().lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut incorrect = false; // flag to eliminate incorrect lines.

        for c in line.chars() {
            if let Some(p) = pairs.get(&c) {
                // incorrect line.
                if stack.last().unwrap() != p {
                    incorrect = true;
                    break;
                }
                // correct closing char.
                stack.pop();
                continue;
            }
            stack.push(c);
        }
        if incorrect {
            continue;
        }
        // calculate score.
        let mut score = 0; // local score.
        while !stack.is_empty() {
            score *= 5;
            score += score_map.get(&stack.pop().unwrap()).unwrap();
        }
        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2]
}
