fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("\n{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// Time complexity will explode as `num_fish` will continue to grow in each iteration.
fn part1(input: &str) -> usize {
    // parse inputs.
    let mut fishes: Vec<usize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    // run simulation.
    for _ in 0..80 {
        let num_fish = fishes.len();

        // for each of the fishes we have seen we update their internal timer
        // and if a timer hits 0, we reset it to 6 and spawn a new fish with timer 8.
        for i in 0..num_fish {
            let fish = fishes.get_mut(i).unwrap();

            if *fish == 0 {
                *fish = 6;
                fishes.push(8);
                continue;
            }
            *fish -= 1;
        }
    }

    fishes.len()
}

// O(n) where n = simulation iterations (256).
// Instead of tracking individual fish, we track the types of fishes (two fish with the same timer value are equivalent).
// In this way we can keep the things we need to keep track of to a constant factor (9).
fn part2(input: &str) -> usize {
    // Set initial conditions.
    // Each position tracks how many fish have that timer value (are of the same type).
    let mut fish_types: [usize; 9] = [0; 9];
    for x in input.trim().split(",").map(|s| s.parse::<usize>().unwrap()) {
        fish_types[x] += 1;
    }

    //run simulation.
    for _ in 0..256 {
        // track the previous positions value, and loop
        // in reverse order shifting all the values to the left.
        let mut next = fish_types[8];
        for i in (0..8).rev() {
            // tmp is used to swap values.
            let tmp = fish_types[i];
            fish_types[i] = next;
            next = tmp;
            // in this case we want to reset timers and spawn new fish.
            if i == 0 {
                fish_types[6] += next;
                fish_types[8] = next;
            }
        }
    }
    fish_types.iter().sum()
}
