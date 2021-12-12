use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

fn main() {
    let in_str = include_str!("../input");
    println!("PART1\n=====\n");
    println!("{}", part1(in_str));
    println!("\nPART2\n=====\n");
    println!("{}", part2(in_str));
}
// implementation of a graph (or cave system), that is probably more complex then necessary.
// An adjacency list would probably be the better choice, but I wanted to try to implement a
// more complex data structure just for practice.

//NOTE: Due to ownership, we pass indexes instead of references (like we would in go or C).

// Cave struct (vertex).
struct Cave {
    small: bool,
    label: String,
    tunnel_idxs: Vec<usize>,
}

// Tunnel struct (edge).
struct Tunnel {
    c1_idx: usize,
    c2_idx: usize,
}

impl Tunnel {
    // given an endpoint of a tunnel, this function calculates the other endpoint.
    fn get_next_cave(&self, c: usize) -> usize {
        if c == self.c1_idx {
            return self.c2_idx;
        } else {
            return self.c1_idx;
        }
    }
}

// System struct (graph).
// NOTE:
// This struct owns the caves and tunnels, while the caves and tunnel can only reference
// each other through indexes in the vectors where they are stored.

struct CaveSystem {
    start_idx: usize,
    end_idx: usize,
    caves: Vec<Cave>,
    tunnels: Vec<Tunnel>,

    part2: bool, // indicates which part we are solving.
}

impl CaveSystem {
    // test if a cave should be retried for part 2.
    fn can_retry(&self, idx: usize) -> bool {
        self.part2 && idx != self.start_idx && idx != self.end_idx
    }
    // calculates the all the paths from start to end, using a BFS.
    pub fn all_paths(&self) -> usize {
        let mut ans = 0;
        let mut q: VecDeque<(usize, HashSet<usize>, bool)> = VecDeque::new();
        q.push_back((self.start_idx, HashSet::from([self.start_idx]), false));

        while !q.is_empty() {
            let (u, visited, twice) = q.pop_front().unwrap();

            // A new path is found.
            if u == self.end_idx {
                ans += 1;
                continue;
            }

            for t_idx in &self.caves[u].tunnel_idxs {
                let next = self.tunnels[*t_idx].get_next_cave(u);

                let mut new_visited = visited.clone();

                // if we have not seen this cave we explore it.
                // NOTE: This will always be true for big caves.
                if !visited.contains(&next) {
                    // If the new cave, we mark it as visited.
                    if self.caves[next].small {
                        new_visited.insert(next);
                    }
                    q.push_back((next, new_visited, twice));

                // If the next cave is a candidate for a retry, we retry it.
                } else if self.can_retry(next) && !twice {
                    q.push_back((next, new_visited, true));
                }
            }
        }

        ans
    }
}

// Parse a string into a cave system.
impl From<&str> for CaveSystem {
    fn from(input: &str) -> Self {
        let mut caves: Vec<Cave> = Vec::new();
        let mut tunnels: Vec<Tunnel> = Vec::new();
        let mut start_idx = 0;
        let mut end_idx = 0;

        let mut seen_cave: HashMap<&str, usize> = HashMap::new();
        let mut current_cave_idx = 0;

        // Calculate the indexes for the caves.
        for line in input.trim().lines() {
            let idxs: Vec<(usize, &str)> = line
                .split("-")
                .into_iter()
                .map(|c| {
                    let mut idx = current_cave_idx;
                    if let Some(i) = seen_cave.get(c) {
                        // If we have seen this cave, we use that index.
                        idx = *i;
                    } else {
                        // give the cave a new index, and increment the current index.
                        current_cave_idx += 1;
                    }
                    (idx, c)
                })
                .collect();

            // Create a new tunnel, and its index.
            tunnels.push(Tunnel {
                c1_idx: idxs[0].0,
                c2_idx: idxs[1].0,
            });
            let tunnel_idx = tunnels.len() - 1;

            // Create caves from labels and indexes.
            for (idx, c) in idxs {
                if seen_cave.contains_key(c) {
                    // If we have seen this cave, we push the new tunnel id to it.
                    caves.get_mut(idx).unwrap().tunnel_idxs.push(tunnel_idx);
                } else {
                    let cave = Cave {
                        // check if all chars in label are lowercase
                        small: c.chars().all(char::is_lowercase),
                        label: String::from(c),
                        tunnel_idxs: vec![tunnel_idx],
                    };
                    // check if we have hit start or end cave, and save their indexes.
                    if c == "start" {
                        start_idx = idx;
                    }
                    if c == "end" {
                        end_idx = idx;
                    }

                    // mark the cave as seen, and push it.
                    seen_cave.insert(c, idx);
                    caves.push(cave);
                }
            }
        }

        CaveSystem {
            start_idx,
            end_idx,
            caves,
            tunnels,
            part2: false,
        }
    }
}

// Formatting for debug purposes.
impl Display for CaveSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "System {{\n").unwrap();
        for t in &self.tunnels {
            write!(
                f,
                "\t{} - {}\n",
                self.caves[t.c1_idx].label, self.caves[t.c2_idx].label
            )
            .unwrap();
        }
        write!(f, "}}")
    }
}

// O(exp(m)) in the worst case (where **m** is #caves),
// But as the inputs are friendly it will likely be fast enough.
fn part1(input: &str) -> usize {
    let system = CaveSystem::from(input);
    system.all_paths()
}

// O(exp(m)).
fn part2(input: &str) -> usize {
    let mut system = CaveSystem::from(input);
    system.part2 = true;
    system.all_paths()
}
