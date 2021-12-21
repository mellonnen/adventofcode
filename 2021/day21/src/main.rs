use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref DP: Mutex<HashMap<(usize, usize, usize, usize), (usize, usize)>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

fn parse_inputs(input: &str) -> (usize, usize) {
    let mut lines = input.trim().lines();
    let p1: usize = lines
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .map(|(_, n)| n.parse().unwrap())
        .unwrap();
    let p2: usize = lines
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .map(|(_, n)| n.parse().unwrap())
        .unwrap();
    (p1, p2)
}

fn part1(input: &str) -> usize {
    let (p1, p2) = parse_inputs(input);
    let mut players: [(usize, usize); 2] = [(0, p1), (0, p2)];

    let mut turn = 0;
    let mut die = 1;
    while players[0].0 < 1000 && players[1].0 < 1000 {
        // select player.
        let player = &mut players[turn % 2];
        // roll three times.
        let mut rolls = 0;
        for _ in 0..3 {
            rolls += die;
            die += 1;
            if die == 101 {
                // wrap die.
                die = 1;
            }
        }
        player.1 += rolls;

        if player.1 % 10 == 0 {
            player.1 = 10; // wrap multiples of 10 to 10.
        } else {
            player.1 %= 10;
        }
        player.0 += player.1; // update score
        turn += 1;
    }
    players[0].0.min(players[1].0) * turn * 3 // three rolls per turn.
}

fn part2(input: &str) -> usize {
    let (p1, p2) = parse_inputs(input);

    // brute-force dirac dice with dynamic programming.
    fn dirac_dice(p1: usize, p2: usize, s1: usize, s2: usize) -> (usize, usize) {
        if s1 >= 21 {
            return (1, 0);
        }
        if s2 >= 21 {
            return (0, 1);
        }

        if DP.lock().unwrap().contains_key(&(p1, p2, s1, s2)) {
            return *DP.lock().unwrap().get(&(p1, p2, s1, s2)).unwrap();
        }

        let mut ans = (0, 0);
        // branch out from each die roll.
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                for d3 in 1..=3 {
                    let mut new_p1 = p1 + d1 + d2 + d3;
                    if new_p1 % 10 == 0 {
                        new_p1 = 10;
                    } else {
                        new_p1 %= 10;
                    }
                    let new_s1 = s1 + new_p1;
                    let wins = dirac_dice(p2, new_p1, s2, new_s1);
                    ans.0 += wins.1;
                    ans.1 += wins.0;
                }
            }
        }
        DP.lock().unwrap().insert((p1, p2, s1, s2), ans);
        ans
    }

    let (p1_wins, p2_wins) = dirac_dice(p1, p2, 0, 0);
    p1_wins.max(p2_wins)
}

#[test]
fn test_part1() {
    let test = include_str!("../sample");
    assert_eq!(part1(test), 739785);
}
#[test]
fn test_part2() {
    let test = include_str!("../sample");
    assert_eq!(part2(test), 444356092776315);
}
