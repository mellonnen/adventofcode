use std::collections::HashSet;

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("\n{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// O(n).
// The inner loop will always result in line.len() operations, which is 12.
fn part1(input: &str) -> usize {
    // allocate array to count ones.
    let mut ones: [usize; 12] = [0; 12];
    // count ones.
    for line in input.trim().lines() {
        line.chars().into_iter().enumerate().for_each(|(i, c)| {
            if c == '1' {
                ones[i] += 1;
            }
        })
    }

    // calculate the gamma_rate to a decimal number.
    let gamma_rate: usize = ones
        .iter()
        .rev() // we are dealing with binary numbers so we must flip the order.
        .enumerate()
        .filter(|&(_, o)| o > &500) // filter out zeros.
        .map(|(i, _)| 2usize.pow(i as u32)) //2^i.
        .sum();

    // inverse the 12 lsb bits.
    let epsilon_rate = gamma_rate ^ 0xfff;
    epsilon_rate * gamma_rate
}

fn part2(input: &str) -> usize {
    // vector of sets of the numbers that with a set bit at vector index.
    // so set[0] => all numbers with a set bit at position 0. (like 10000)
    let mut set: Vec<_> = (0..12).map(|_| HashSet::<usize>::new()).collect();

    // vector of sets of the numbers that a unset bit at the vector index.
    // so set[0] => all numbers with a unset bit at position 0. (like 00000)
    let mut un_set: Vec<_> = (0..12).map(|_| HashSet::<usize>::new()).collect();

    // identity set (all numbers).
    let mut tot: HashSet<usize> = HashSet::new();

    // loop over lines and populate sets.
    for line in input.trim().lines() {
        line.chars().into_iter().enumerate().for_each(|(i, c)| {
            if c == '1' {
                set[i].insert(usize::from_str_radix(line, 2).unwrap());
            } else {
                un_set[i].insert(usize::from_str_radix(line, 2).unwrap());
            }
            tot.insert(usize::from_str_radix(line, 2).unwrap());
        })
    }

    // copy identity set.
    let mut ogr = tot.clone();
    let mut co2sr = tot.clone();

    // reduce the o2gr and co2sr sets until only one element remain in both.
    for i in 0..12 {
        if ogr.len() == 1 && co2sr.len() == 1 {
            break;
        }
        if ogr.len() > 1 {
            // check if the majority of the numbers bits are set at position i.
            //NOTE: due to rounding shenanigans I convert the to floats.
            if ogr.intersection(&set[i]).count() as f64 >= ogr.len() as f64 / 2f64 {
                ogr.retain(|e| set[i].contains(e));
            } else {
                ogr.retain(|e| un_set[i].contains(e));
            }
        }

        if co2sr.len() > 1 {
            if co2sr.intersection(&set[i]).count() as f64 >= co2sr.len() as f64 / 2f64 {
                co2sr.retain(|e| un_set[i].contains(e));
            } else {
                co2sr.retain(|e| set[i].contains(e));
            }
        }
    }
    assert_eq!(ogr.len(), 1);
    assert_eq!(co2sr.len(), 1);
    let ogr_val: usize = ogr.drain().sum();
    let co2sr_val: usize = co2sr.drain().sum();
    ogr_val * co2sr_val
}
