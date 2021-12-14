use std::collections::HashMap;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// regex match wont give overlapping matches.
fn get_overlaping_match_idxs(input: &String, m: &str) -> Vec<usize> {
    let chars: Vec<char> = input.chars().into_iter().collect();
    let mut res = Vec::new();
    for i in 1..chars.len() {
        if format!("{}{}", chars[i - 1], chars[i]).as_str() == m {
            res.push(i - 1);
        }
    }
    res
}

// Fell for the same trick again! So this is a naive implementation works for
// part1. Due to string concatenation the time complexity will explode.
fn part1(input: &str) -> usize {
    // read input.
    let mut lines = input.trim().lines();
    let mut template = String::from(lines.next().unwrap());
    let mut rules: Vec<(&str, &str)> = Vec::new();
    lines.next(); // consume empty line.
    for line in lines {
        rules.push(line.split_once(" -> ").unwrap());
    }

    for _ in 0..10 {
        //get all matches of a rule and save a tuple of index and &str.
        let mut insertions: Vec<(usize, &str)> = Vec::new();
        for (matcher, inserter) in &rules {
            for idx in get_overlaping_match_idxs(&template, matcher) {
                insertions.push((idx + 1, inserter));
            }
        }

        // reverse sort by indexes, so we do not have top update the indexes as we insert.
        insertions.sort_by(|(a, _), (b, _)| b.cmp(a));
        // insert!
        for (idx, inserter) in insertions {
            template.insert_str(idx, inserter);
        }
    }
    // count char occurrences.
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}

// More sophisticated solution compared with part1.
// We observe that there are a finite number of unique pairs, so we track pairs instead of growing the string.
// The time complexity is O(n*m) where **n** is the #unique pairs, and **m** is the number of steps.
fn part2(input: &str) -> usize {
    let mut lines = input.trim().lines();

    let template = String::from(lines.next().unwrap());
    // Track what pairs are yielded from applying a rule, and which char is added to the string.
    // So if we are applying the rule AA -> B we get a entry: {"AA":("AB", "BA", 'B')}
    let mut rules: HashMap<String, (String, String, char)> = HashMap::new();
    // Track how many occurrences of a pair we have in the string.
    let mut pairs: HashMap<String, usize> = HashMap::new();
    // Track counts of chars in the string.
    let mut counts: HashMap<char, usize> = HashMap::new();
    lines.next();
    for line in lines {
        let (matcher, insert) = line
            .split_once(" -> ")
            .map(|(m, i)| (m.to_string(), i.to_string()))
            .unwrap();
        let chars: Vec<char> = matcher.chars().collect();

        rules.insert(
            matcher.clone(),
            (
                format!("{}{}", chars[0], insert),
                format!("{}{}", insert, chars[1]),
                insert.chars().next().unwrap(),
            ),
        );
    }

    // Count pairs and chars of template.
    for i in 1..template.len() {
        let k = &template[i - 1..=i];
        *pairs.entry(k.to_string()).or_insert(0) += 1;
    }
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for _ in 0..40 {
        // Save the updates to the string.
        let mut updates: Vec<(String, usize)> = Vec::new();
        for (pair, pair_count) in pairs.iter_mut() {
            // Apply the rule.
            let (p1, p2, c) = rules.get(pair).unwrap();
            // push the updates.
            updates.push((p1.clone(), *pair_count));
            updates.push((p2.clone(), *pair_count));
            // update the char count.
            *counts.entry(*c).or_insert(0) += *pair_count;
            // As applying the rule consumes all occurrences of the pair, we set the counter to 0.
            *pair_count = 0;
        }

        for (pair, count) in updates {
            *pairs.entry(pair).or_insert(0) += count;
        }
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}
