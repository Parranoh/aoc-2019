use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::io::stdin;

const NKEYS: usize = 26;

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    poss: [(usize, usize); 4],
    collected: [bool; NKEYS],
}
struct Entry {
    cost: usize,
    state: State,
}
impl Entry {
    fn new(cost: usize, poss: [(usize, usize); 4], collected: [bool; NKEYS]) -> Self {
        Self {
            cost: cost,
            state: State {
                poss: poss,
                collected: collected,
            }
        }
    }
}
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for Entry {}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // reverse for min-heap
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lines()
        .map(|l| l.unwrap()
            .as_bytes()
            .iter()
            .map(|c| *c)
            .collect())
        .collect();
    let (map, doors, keys, poss) = {
        let mut map   = Vec::new(); map.reserve(input.len());
        let mut doors = [(0, 0); NKEYS];
        let mut keys  = [(0, 0); NKEYS];
        let mut pos   = None;
        for y in 0..input.len() {
            map.push(Vec::new()); map[y].reserve(input[y].len());
            for x in 0..input[y].len() {
                map[y].push(input[y][x] != b'#');
                match input[y][x] {
                    b'#' | b'.' => {},
                    b'@' => {
                        pos = Some((x, y));
                    },
                    b'A'..=b'Z' => {
                        doors[(input[y][x] - b'A') as usize] = (x, y);
                    },
                    b'a'..=b'z' => {
                        keys[(input[y][x] - b'a') as usize] = (x, y);
                    },
                    _ => unreachable!(),
                }
            }
        }
        let (x, y) = pos.unwrap();
        for (x, y) in [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            map[y][x] = false;
        }
        (map, doors, keys, [(x + 1, y + 1), (x + 1, y - 1), (x - 1, y + 1), (x - 1, y - 1)])
    };

    let start = Entry::new(0, poss, [false; NKEYS]);
    let is_target = |s: &State| s.collected == [true; NKEYS];
    let mut pq = BinaryHeap::new();
    let mut dist = HashMap::from([(start.state.clone(), 0)]);
    pq.push(start);
    while let Some(s) = pq.pop() {
        if is_target(&s.state) {
            println!("{}", s.cost);
            return;
        }
        if dist[&s.state] < s.cost {
            continue;
        }

        let real_map = {
            let mut m = map.clone();
            for i in 0..doors.len() {
                let (x, y) = doors[i];
                m[y][x] = s.state.collected[i];
            }
            m
        };

        let mut visited: Vec<Vec<bool>> = map.iter().map(|r| r.iter().map(|_| false).collect()).collect();
        for i in 0..s.state.poss.len() {
            let mut q = VecDeque::from([(s.cost, s.state.poss[i].0, s.state.poss[i].1)]);
            visited[s.state.poss[i].1][s.state.poss[i].0] = true;
            while let Some((cost, x, y)) = q.pop_front() {
                let mut enq = || {
                    for (x, y) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
                        if real_map[y][x] && !visited[y][x] {
                            visited[y][x] = true;
                            q.push_back((cost + 1, x, y));
                        }
                    }
                };
                match keys.iter().position(|&p| p == (x, y)) {
                    None => {
                        enq();
                    },
                    Some(k) => {
                        if s.state.collected[k] {
                            enq();
                        } else {
                            // println!("found key {k} in {cost} steps");
                            let mut state = s.state.clone();
                            state.collected[k] = true;
                            state.poss[i] = (x, y);
                            let entry = Entry { cost: cost, state: state };
                            match dist.get_mut(&entry.state) {
                                None => {
                                    dist.insert(entry.state.clone(), cost);
                                    pq.push(entry);
                                },
                                Some(d) => {
                                    if cost < *d {
                                        *d = cost;
                                        pq.push(entry);
                                    }
                                },
                            }
                        }
                    },
                }
            }
        }
    }
}
