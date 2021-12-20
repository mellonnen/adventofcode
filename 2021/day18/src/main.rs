use std::{fmt::Display, ops::Add, str::FromStr, string::ParseError};

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}
// Binary tree-like datastructure, can either be a number of a pair of snailnumbers.
#[derive(Clone)]
enum SnailFishNumber {
    Literal(u32),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>),
}

// This enum is used to pass upp the recursion when we have exploded a pair.
enum Explosion {
    Consumed,
    Lhs(u32),
    Rhs(u32),
}

impl FromStr for SnailFishNumber {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Literal.
        if let Ok(val) = s.parse::<u32>() {
            return Ok(SnailFishNumber::Literal(val));
        }
        // Pair.
        let x = s.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        let mut matched = 0;
        let mut l = "";
        let mut start = 0;
        // find the left number.
        for (i, c) in x.char_indices() {
            match c {
                '[' => matched += 1,
                ']' => matched -= 1,
                ',' => {
                    if matched == 0 {
                        l = &x[start..i];
                        start = i + 1;
                        break;
                    }
                }
                _ => (),
            }
        }
        // get right number.
        let r = &x[start..];
        Ok(SnailFishNumber::Pair(
            Box::new(SnailFishNumber::from_str(l).unwrap()),
            Box::new(SnailFishNumber::from_str(r).unwrap()),
        ))
    }
}

impl Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishNumber::Literal(n) => write!(f, "{}", n),
            SnailFishNumber::Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl Add for SnailFishNumber {
    type Output = Self;
    // add and reduce numbers.
    fn add(self, rhs: Self) -> Self::Output {
        let mut num = Self::Pair(Box::new(self), Box::new(rhs));
        loop {
            let mut exploded = false;
            num.explode(&mut exploded, 0);
            if exploded {
                continue;
            }

            let mut split = false;
            num.split(&mut split);
            if split {
                continue;
            }

            // no explosion and split, reduction done!
            break;
        }
        num
    }
}

impl SnailFishNumber {
    fn unwrap_literal(&self) -> u32 {
        if let Self::Literal(n) = self {
            return *n;
        }
        panic!("not literal");
    }

    fn unwrap_pair(&self) -> (u32, u32) {
        if let Self::Pair(l, r) = self {
            if matches!(**l, Self::Literal(_)) && matches!(**r, Self::Literal(_)) {
                return (l.unwrap_literal(), r.unwrap_literal());
            }
        }
        panic!("cant unwrap pair")
    }

    // explosion function.
    pub fn explode(
        &mut self,
        exploded: &mut bool,
        depth: usize,
    ) -> (Option<Explosion>, Option<Explosion>) {
        // we have already exploded.
        if *exploded {
            return (None, None);
        }
        // if we are a literal.
        if matches!(self, Self::Literal(_)) {
            return (None, None);
        }
        if depth == 4 {
            let (l, r) = self.unwrap_pair();
            *self = Self::Literal(0);
            *exploded = true;
            return (Some(Explosion::Lhs(l)), Some(Explosion::Rhs(r)));
        }

        if depth < 4 {
            if let Self::Pair(l, r) = self {
                if let (Some(l_explosion), Some(r_explosion)) = l.explode(exploded, depth + 1) {
                    // propagate the explosion.
                    let r_explosion = r.propagate(r_explosion);
                    return (Some(l_explosion), Some(r_explosion));
                } else if let (Some(l_explosion), Some(r_explosion)) =
                    r.explode(exploded, depth + 1)
                {
                    let l_explosion = l.propagate(l_explosion);
                    return (Some(l_explosion), Some(r_explosion));
                }
            }
        }
        (None, None)
    }

    // This function propagates the explosion to the left most and right most leaf.
    fn propagate(&mut self, explosion: Explosion) -> Explosion {
        match explosion {
            Explosion::Consumed => explosion,
            Explosion::Lhs(x) => match self {
                Self::Literal(n) => {
                    *n += x;
                    Explosion::Consumed
                }
                Self::Pair(_, r) => r.propagate(explosion),
            },

            Explosion::Rhs(x) => match self {
                Self::Literal(n) => {
                    *n += x;
                    Explosion::Consumed
                }
                Self::Pair(l, _) => l.propagate(explosion),
            },
        }
    }

    fn split(&mut self, split: &mut bool) {
        if *split {
            return;
        }
        match self {
            Self::Literal(n) => {
                if *n >= 10 {
                    *self = Self::Pair(
                        Box::new(Self::Literal(*n / 2)),
                        Box::new(Self::Literal((*n + 1) / 2)),
                    );
                    *split = true;
                }
            }

            Self::Pair(l, r) => {
                l.split(split);
                r.split(split);
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Literal(n) => *n,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut numbers: Vec<SnailFishNumber> = input
        .trim()
        .lines()
        .map(|l| SnailFishNumber::from_str(l).unwrap())
        .collect();
    let inital = numbers.remove(0);
    let num = numbers.into_iter().fold(inital, |acc, curr| acc + curr);

    num.magnitude()
}

fn part2(input: &str) -> u32 {
    let numbers: Vec<SnailFishNumber> = input
        .trim()
        .lines()
        .map(|l| SnailFishNumber::from_str(l).unwrap())
        .collect();

    let mut ans = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let n1 = numbers[i].clone();
            let n2 = numbers[j].clone();

            ans = ans.max((n1 + n2).magnitude());
        }
    }
    ans
}

#[test]
fn explosion() {
    let tests: Vec<(&str, &str)> = vec![
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];

    for (test, expect) in tests {
        let mut num = SnailFishNumber::from_str(test).unwrap();
        num.explode(&mut false, 0);
        assert_eq!(format!("{}", num), expect);
    }
}
