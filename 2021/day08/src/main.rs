use std::collections::HashMap;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n) as there are always 4 values to decode.
fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.trim().lines() {
        let values = line
            .split(" | ")
            .into_iter()
            .last()
            .unwrap()
            .split(" ")
            .into_iter()
            // filter by the lengths that give 1, 4, 7, and 8 and count.
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();

        count += values;
    }
    count
    //WORKS:
}

/*
    IDEA:
    Mapping of segments numbers.
     000
    1   2
    1   2
     333
    4   5
    4   5
     666

    In digits 0-9 this is how many times a segment is seen.
    0 appears 8 times
    1 appears 6 times *
    2 appears 8 times
    3 appears 7 times
    4 appears 4 times *
    5 appears 9 times *
    6 appears 7 times
    We can us this occurrence information to deduce which segments map to 1, 4, 5.

    The rest of the segments we will have to use properties of specific digits to determine the mapping.

    Digits 1 and 7 share segment 2, but 7 also have segment 0.
    Therefore, we ken check if a if one of the 8-count segments (0 and 2)
    are in both digits 1 and 7 (which have unique lengths so we can filter them out).
    - If True -> that segment is segment 2 and the other segment 0.
    - If False -> that segment is segment 0 and the other segment 2.

    The 4 digit contains segment 3, so we can check if one of the two 7-count segments (3 and 6)
    are in the digit 4 (which have a unique length so it can be filtered out).
    - If True -> that segment is segment 3 and the other is segment 6.
    - If False -> that segment is segment 6 and the other is segment 3.
*/
// O(n)
fn part2(input: &str) -> usize {
    let mut count = 0;
    // Mapping from segment numbering (as strings) to the numbers displayed.
    let segments_to_number: HashMap<String, char> = HashMap::from([
        ("012456".to_owned(), '0'),
        ("25".to_owned(), '1'),
        ("02346".to_owned(), '2'),
        ("02356".to_owned(), '3'),
        ("1235".to_owned(), '4'),
        ("01356".to_owned(), '5'),
        ("013456".to_owned(), '6'),
        ("025".to_owned(), '7'),
        ("0123456".to_owned(), '8'),
        ("012356".to_owned(), '9'),
    ]);
    for line in input.trim().lines() {
        // Counting the segment frequencies. Will map something like 'a' -> 4.
        let mut segment_counts: HashMap<char, usize> = HashMap::new();
        // Actual mapping to our segment numbering system. Will map something like 'a' -> '4'.
        let mut segment_map: HashMap<char, char> = HashMap::new();

        // parse the line.
        let mut split = line.split(" | ").into_iter();
        let patterns: Vec<&str> = split.next().unwrap().split(" ").into_iter().collect();
        let values: Vec<&str> = split.next().unwrap().split(" ").into_iter().collect();

        // count segment occurrences and filter out patterns for digits 1, 4 and 7.
        let mut one = "";
        let mut four = "";
        let mut seven = "";
        for pattern in patterns {
            if pattern.len() == 2 {
                one = pattern;
            }
            if pattern.len() == 3 {
                seven = pattern;
            }
            if pattern.len() == 4 {
                four = pattern;
            }
            for segment in pattern.chars() {
                if let Some(x) = segment_counts.get_mut(&segment) {
                    *x += 1;
                } else {
                    segment_counts.insert(segment, 1);
                }
            }
        }
        // map segments '1', '4', '5' based on occurrences
        // and save segments with counts 7 and 8.
        let mut seg8: Vec<char> = Vec::new();
        let mut seg7: Vec<char> = Vec::new();
        for (k, v) in segment_counts {
            if v == 6 {
                segment_map.insert(k, '1');
            }

            if v == 4 {
                segment_map.insert(k, '4');
            }

            if v == 9 {
                segment_map.insert(k, '5');
            }

            if v == 7 {
                seg7.push(k);
            }
            if v == 8 {
                seg8.push(k);
            }
        }
        assert_eq!(segment_map.len(), 3);
        assert_eq!(seg7.len(), 2);
        assert_eq!(seg8.len(), 2);

        // map the rest of the segments.
        if one.contains(seg8[0]) && seven.contains(seg8[0]) {
            segment_map.insert(seg8[0], '2');
            segment_map.insert(seg8[1], '0');
        } else {
            segment_map.insert(seg8[1], '2');
            segment_map.insert(seg8[0], '0');
        }

        if four.contains(seg7[0]) {
            segment_map.insert(seg7[0], '3');
            segment_map.insert(seg7[1], '6');
        } else {
            segment_map.insert(seg7[1], '3');
            segment_map.insert(seg7[0], '6');
        }
        assert_eq!(segment_map.len(), 7);
        // construct the String representation of this lines number.
        let mut num_str: String = String::new();
        for v in values.into_iter() {
            // construct the String key for lookup in segments_to_number.
            let mut x: Vec<char> = v
                .chars()
                .into_iter()
                .map(|c| *segment_map.get(&c).unwrap())
                .collect();
            x.sort();
            num_str.push(*segments_to_number.get(&String::from_iter(x)).unwrap());
        }
        count += num_str.parse::<usize>().unwrap();
    }
    count
}
