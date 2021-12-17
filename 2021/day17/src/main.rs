use std::ops::Range;

fn main() {
    let in_str = include_str!("../input");
    // let in_str = "target area: x=20..30, y=-10..-5";
    let (p1, p2) = solve(in_str);
    println!("PART1\n=====\n");
    println!("{}", p1);
    println!("\nPART2\n=====\n");
    println!("{}", p2);
}
fn parse_inputs(input: &str) -> ((i64, i64), (i64, i64)) {
    let target_range: Vec<(i64, i64)> = input
        .trim()
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ")
        .map(|s| {
            let (i, j) = s.split_once("=").unwrap().1.split_once("..").unwrap();
            (i.parse().unwrap(), j.parse().unwrap())
        })
        .collect();
    (*target_range.get(0).unwrap(), *target_range.get(1).unwrap())
}

// The idea is to try to bound the ranges as tight as possible.
// vx is always non negative, and should be bounded by the upper target x range.
// vy should be be lower bounded by the lower target y range
// And the we try all possible initial velocities in the ranges.
fn solve(input: &str) -> (i64, i64) {
    let ((x1, x2), (y1, y2)) = parse_inputs(input);

    let mut p1 = 0;
    let mut p2 = 0;
    for vx in 0..x2 {
        // Hard to do the upper bound for the velocity, so we set it to a large number.
        for vy in y1..1000 {
            let mut hit_target = false;
            let mut max_y = 0;
            let mut x = 0;
            let mut y = 0;
            let mut dx = vx;
            let mut dy = vy;

            // run simulation for 500 time steps.
            for _ in 0..500 {
                x += dx;
                y += dy;
                max_y = max_y.max(y);

                // drag.
                if dx > 0 {
                    dx -= 1;
                } else if dx < 0 {
                    dx += 1;
                }
                // gravity
                dy -= 1;

                // hit the target.
                if (x1 <= x && x <= x2) && (y1 <= y && y <= y2) {
                    hit_target = true;
                    break;
                }
            }
            if hit_target {
                p1 = p1.max(max_y);
                p2 += 1;
            }
        }
    }
    (p1, p2)
}
