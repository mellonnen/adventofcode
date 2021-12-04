use std::collections::{HashMap, HashSet};

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("\n{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// let c = # calls
// let n = # boards

// Time complexity: O(c * n)
fn part1(input: &str) -> usize {
    let mut lines = input.trim().lines();
    // parse the calls.
    let calls: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // A board is represented by a HashMap number -> position on board.
    let mut boards: Vec<HashMap<usize, (usize, usize)>> = Vec::new();
    // Which numbers that are marked.
    let mut boards_markers: Vec<HashSet<(usize, usize)>> = Vec::new();

    // loop over the lines and populate boards.
    // This will loop "1 board at a time"
    while let Some(_) = lines.next() {
        let mut board = HashMap::new();
        for y in 0..5 {
            for (x, val) in lines.next().unwrap().split_whitespace().enumerate() {
                let num = val.parse().unwrap();
                board.insert(num, (y, x));
            }
        }
        boards.push(board);
        boards_markers.push(HashSet::new());
    }

    // Start the Bingo Game!
    for i in 0..calls.len() {
        // Update all the boards with the latest call.
        for j in 0..boards.len() {
            if let Some(pos) = boards[j].get(&calls[i]) {
                boards_markers[j].insert(*pos);
            }
        }

        // we cant have bingo unless 5 numbers has been called.
        if i < 4 {
            continue;
        }

        // Check if we have bingo on any of the boards.
        for (j, markers) in boards_markers.iter().enumerate() {
            let mut bingo = false;
            // Check columns.
            for y in 0..5 {
                let mut col_bingo = true;
                for x in 0..5 {
                    if !markers.contains(&(y, x)) {
                        col_bingo = false;
                        break;
                    }
                }
                if col_bingo {
                    bingo = true;
                }
            }

            // If we did not get column bingo, we check row bingo.
            if !bingo {
                for x in 0..5 {
                    let mut row_bingo = true;
                    for y in 0..5 {
                        if !markers.contains(&(y, x)) {
                            row_bingo = false;
                            break;
                        }
                    }
                    if row_bingo {
                        bingo = true;
                    }
                }
            }

            // Bingo! now we need to calculate the score.
            if bingo {
                let mut score = 0;
                for (num, pos) in boards[j].iter() {
                    if !markers.contains(&pos) {
                        score += num;
                    }
                }
                return score * calls[i];
            }
        }
    }
    0
}

// O(c * n)
fn part2(input: &str) -> usize {
    // Identical to part1, except we remove boards that get bingo
    // until the last board get bingo and the we return its score.

    let mut lines = input.trim().lines();
    let calls: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<HashMap<usize, (usize, usize)>> = Vec::new();
    let mut boards_markers: Vec<HashSet<(usize, usize)>> = Vec::new();

    while let Some(_) = lines.next() {
        let mut board = HashMap::new();
        for y in 0..5 {
            for (x, val) in lines.next().unwrap().split_whitespace().enumerate() {
                let num = val.parse().unwrap();
                board.insert(num, (y, x));
            }
        }
        boards.push(board);
        boards_markers.push(HashSet::new());
    }

    for i in 0..calls.len() {
        // keep track of the boards that get bingo.
        let mut boards_to_remove: Vec<usize> = Vec::new();
        for j in 0..boards.len() {
            if let Some(pos) = boards[j].get(&calls[i]) {
                boards_markers[j].insert(*pos);
            }
        }

        if i < 4 {
            continue;
        }

        for (j, markers) in boards_markers.iter().enumerate() {
            let mut bingo = false;
            for y in 0..5 {
                let mut col_bingo = true;
                for x in 0..5 {
                    if !markers.contains(&(y, x)) {
                        col_bingo = false;
                        break;
                    }
                }
                if col_bingo {
                    bingo = true;
                }
            }
            if !bingo {
                for x in 0..5 {
                    let mut row_bingo = true;
                    for y in 0..5 {
                        if !markers.contains(&(y, x)) {
                            row_bingo = false;
                            break;
                        }
                    }
                    if row_bingo {
                        bingo = true;
                    }
                }
            }

            if bingo {
                // case the last board gets bingo, we return the score
                if boards.len() == 1 {
                    let mut score = 0;
                    for (num, pos) in boards[j].iter() {
                        if !markers.contains(&pos) {
                            score += num;
                        }
                    }

                    return score * calls[i];
                } else {
                    boards_to_remove.push(j);
                }
            }
        }
        // reverse the list so we do not have to update indices.
        boards_to_remove.reverse();
        for board in boards_to_remove {
            boards.remove(board);
            boards_markers.remove(board);
        }
    }
    0
}
