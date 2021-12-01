use easy_io::InputReader;

fn main() {
    part1();
    part2();
}

// O(n)
fn part1() {
    let mut input = InputReader::from_file("input");
    println!("PART 1\n=====\n");

    // input.has_more() does not work, therefore have to specify n.
    let n = 2000;
    let mut curr: usize = input.next();
    let mut increase = 0;
    for _ in 1..n {
        let next = input.next();
        if curr < next {
            increase += 1;
        }
        curr = next;
    }

    println!("{}\n", increase)
}

// O(n)
fn part2() {
    let mut input = InputReader::from_file("input");
    println!("PART 2\n=====\n");

    let n = 2000;
    let mut a_1: usize = input.next();
    let mut a_2: usize = input.next();
    let mut a_3: usize = input.next();
    let mut increase = 0;
    for _ in 3..n {
        let a_4: usize = input.next();
        if (a_1 + a_2 + a_3) < (a_2 + a_3 + a_4) {
            increase += 1;
        }
        a_1 = a_2;
        a_2 = a_3;
        a_3 = a_4;
    }
    println!("{}", increase)
}
