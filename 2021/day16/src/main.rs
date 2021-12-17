use std::collections::HashMap;

fn main() {
    let in_str = include_str!("../input");

    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}

// very ugly string transformation from hex to bin string.
fn hex_to_bin(hex: &str) -> String {
    let hex_lookup: HashMap<char, &str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    let mut bin: String = String::new();
    for c in hex.chars() {
        bin.push_str(hex_lookup.get(&c).unwrap());
    }
    bin
}

// chop of a i64 from the first i chars and decode it to decimal.
fn chop_i64(s: &mut String, i: usize) -> i64 {
    let chars = s.chars();
    let chop: String = chars.take(i).collect();
    s.drain(..i);
    i64::from_str_radix(chop.as_str(), 2).unwrap()
}

// chop of a usize from the first i chars and decode it to decimal.
fn chop_usize(s: &mut String, i: usize) -> usize {
    let chars = s.chars();
    let chop: String = chars.take(i).collect();
    s.drain(..i);
    usize::from_str_radix(chop.as_str(), 2).unwrap()
}

// chop of a substring of i chars from the string.
fn chop_string(s: &mut String, i: usize) -> String {
    let chars = s.chars();
    let chop: String = chars.take(i).collect();
    s.drain(..i);
    chop
}

// recursive function that sums up all version numbers of the packets.
// works by continuously chop of the input string.
fn count_versions(packet: &mut String) -> i64 {
    let mut version_count = 0;
    version_count += chop_i64(packet, 3);
    // Handle literal values.
    if chop_usize(packet, 3) == 4 {
        // chop each value until we hit a leading zero.
        loop {
            let done = chop_i64(packet, 1) == 0;
            chop_i64(packet, 4);
            if done {
                break;
            }
        }
        // return early.
        return version_count;
    }

    // check the length ID and handle accordingly.
    if chop_usize(packet, 1) == 0 {
        let length = chop_usize(packet, 15);
        // chop of the sub packets.
        let mut sub_packets = chop_string(packet, length);
        while !sub_packets.is_empty() {
            version_count += count_versions(&mut sub_packets);
        }
    } else {
        // there are an exact number of packets.
        let num_packets = chop_usize(packet, 11);
        for _ in 0..num_packets {
            version_count += count_versions(packet);
        }
    }

    version_count
}

// converts a slice of chars to usize.
fn chars_to_usize(c: &[char]) -> usize {
    let c: String = c.iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

// recursive function that solves part2.
// The "chopping" version of this had some hard to spot bug, so this version maintains an index instead.
// A good thing about this is that it does not pass around references,
fn evaluate(packet: &[char], mut i: usize) -> (usize, usize) {
    let packet_type = chars_to_usize(&packet[i + 3..i + 6]);
    i += 6;

    if packet_type == 4 {
        let mut v = 0;

        // decode literal.
        loop {
            let stop = &packet[i];
            v = v * 16 + chars_to_usize(&packet[i + 1..i + 5]);
            i += 5;
            if *stop == '0' {
                break;
            }
        }
        return (i, v);
    }

    let length_type = &packet[i];
    i += 1;

    let mut values: Vec<usize> = Vec::new();
    // handle different length types.
    if *length_type == '0' {
        let length = chars_to_usize(&packet[i..i + 15]);
        i += 15;
        let stop = i + length;

        while i < stop {
            let (new_i, val) = evaluate(packet, i);
            i = new_i; // update i.
            values.push(val);
        }
    } else {
        let sub_packets = chars_to_usize(&packet[i..i + 11]);
        i += 11;

        for _ in 0..sub_packets {
            let (new_i, val) = evaluate(packet, i);
            i = new_i;
            values.push(val);
        }
    }
    // evaluate the packet.
    let mut v = values.iter();
    let res = match packet_type {
        0 => v.sum(),
        1 => v.product(),
        2 => *v.min().unwrap(),
        3 => *v.max().unwrap(),
        5 => (v.next().unwrap() > v.next().unwrap()) as usize,
        6 => (v.next().unwrap() < v.next().unwrap()) as usize,
        7 => (v.next().unwrap() == v.next().unwrap()) as usize,
        _ => unreachable!(),
    };

    (i, res)
}

fn part1(input: &str) -> i64 {
    count_versions(&mut hex_to_bin(input.trim()))
}

fn part2(input: &str) -> usize {
    let packet: Vec<char> = hex_to_bin(input.trim()).chars().collect();
    let (_, ans) = evaluate(&packet, 0);
    ans
}
