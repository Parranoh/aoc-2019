use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;

struct Computer {
    ram: HashMap<usize, i64>,
    pc: usize,
    relative_base: i64,
}

enum PC {
    Inc(usize),
    Set(usize),
}

impl Computer {
    fn new(ram: Vec<i64>) -> Self {
        Computer {
            ram: ram
                .into_iter()
                .enumerate()
                .collect(),
            pc: 0,
            relative_base: 0,
        }
    }

    fn cycle(&mut self, input: i64) -> (bool, Option<i64>) {
        let mut output = None;
        let (dpc, halt) = match self.op() {
            1 => {
                *self.param(3) = *self.param(1) + *self.param(2);
                (PC::Inc(4), false)
            },
            2 => {
                *self.param(3) = *self.param(1) * *self.param(2);
                (PC::Inc(4), false)
            },
            3 => {
                *self.param(1) = input;
                (PC::Inc(2), false)
            },
            4 => {
                output = Some(*self.param(1));
                (PC::Inc(2), false)
            },
            5 => {
                (if *self.param(1) != 0 {
                    PC::Set(*self.param(2) as usize)
                } else {
                    PC::Inc(3)
                }, false)
            },
            6 => {
                (if *self.param(1) == 0 {
                    PC::Set(*self.param(2) as usize)
                } else {
                    PC::Inc(3)
                }, false)
            },
            7 => {
                *self.param(3) = if *self.param(1) < *self.param(2) { 1 } else { 0 };
                (PC::Inc(4), false)
            },
            8 => {
                *self.param(3) = if *self.param(1) == *self.param(2) { 1 } else { 0 };
                (PC::Inc(4), false)
            },
            9 => {
                self.relative_base += *self.param(1);
                (PC::Inc(2), false)
            },
            99 => (PC::Inc(0), true),
            _ => unreachable!(),
        };
        match dpc {
            PC::Inc(d) => self.pc += d,
            PC::Set(a) => self.pc = a,
        };
        (halt, output)
    }

    fn op(&mut self) -> i64 {
        self.opcode() % 100
    }

    fn param(&mut self, i: usize) -> &mut i64 {
        match self.parammode(i) {
            0 => {
                let addr = *self.ram.entry(self.pc + i).or_default();
                assert!(addr >= 0);
                self.ram.entry(addr as usize).or_default()
            },
            1 => self.ram.entry(self.pc + i).or_default(),
            2 => {
                let addr = *self.ram.entry(self.pc + i).or_default() + self.relative_base;
                assert!(addr >= 0);
                self.ram.entry(addr as usize).or_default()
            },
            _ => unreachable!(),
        }
    }

    fn parammode(&mut self, i: usize) -> i64 {
        let v = 10_i64.pow(i as u32 + 1);
        (self.opcode() / v) % 10
    }

    fn opcode(&mut self) -> i64 {
        *self.ram.entry(self.pc).or_default()
    }
}

struct Droid {
    program: Computer,
    map: HashMap<(i32, i32), i64>,
    pos: (i32, i32),
    oxygen_pos: Option<(i32, i32)>,
}

impl Droid {
    fn new(program: Computer) -> Self {
        Self {
            program: program,
            map: HashMap::from([((0, 0), 1)]),
            pos: (0, 0),
            oxygen_pos: None,
        }
    }

    fn north(pos: (i32, i32)) -> (i32, i32) {
        (pos.0, pos.1 + 1)
    }
    fn south(pos: (i32, i32)) -> (i32, i32) {
        (pos.0, pos.1 - 1)
    }
    fn west(pos: (i32, i32)) -> (i32, i32) {
        (pos.0 - 1, pos.1)
    }
    fn east(pos: (i32, i32)) -> (i32, i32) {
        (pos.0 + 1, pos.1)
    }

    fn movement(&mut self, dir: i64) -> i64 {
        loop {
            match self.program.cycle(dir) {
                (true, _)    => unreachable!(),
                (_, None)    => {},
                (_, Some(s)) => {
                    let pos = match dir {
                        1 => Self::north(self.pos),
                        2 => Self::south(self.pos),
                        3 => Self::west(self.pos),
                        4 => Self::east(self.pos),
                        _ => unreachable!(),
                    };
                    self.map.insert(pos, s);
                    match s {
                        0 => {},
                        1 => {
                            self.pos = pos;
                        },
                        2 => {
                            self.pos = pos;
                            self.oxygen_pos = Some(pos);
                        },
                        _ => unreachable!(),
                    }
                    return s;
                },
            }
        }
    }

    fn explore(&mut self) {
        let mut stack = vec![(0, 0)];
        while let Some(curr) = stack.last() {
            assert_eq!(*curr, self.pos);
            if !self.map.contains_key(&Self::north(*curr)) {
                if self.movement(1) != 0 {
                    stack.push(self.pos);
                    continue;
                }
            }
            if !self.map.contains_key(&Self::south(*curr)) {
                if self.movement(2) != 0 {
                    stack.push(self.pos);
                    continue;
                }
            }
            if !self.map.contains_key(&Self::west(*curr)) {
                if self.movement(3) != 0 {
                    stack.push(self.pos);
                    continue;
                }
            }
            if !self.map.contains_key(&Self::east(*curr)) {
                if self.movement(4) != 0 {
                    stack.push(self.pos);
                    continue;
                }
            }

            stack.pop();
            if let Some(prev) = stack.last() {
                if *prev == Self::north(self.pos) {
                    self.movement(1);
                } else if *prev == Self::south(self.pos) {
                    self.movement(2);
                } else if *prev == Self::west(self.pos) {
                    self.movement(3);
                } else if *prev == Self::east(self.pos) {
                    self.movement(4);
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn bfs(&mut self) -> usize {
        self.explore();

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut time = 0;
        let start = self.oxygen_pos.unwrap();
        queue.push_back((0, start));
        visited.insert(start);
        while let Some((d, pos)) = queue.pop_front() {
            assert!(d < self.map.len());
            match self.map[&pos] {
                0 => {},
                _ => {
                    time = d;
                    for new_pos in [Self::north(pos), Self::south(pos), Self::west(pos), Self::east(pos)] {
                        if !visited.contains(&new_pos) {
                            visited.insert(new_pos);
                            queue.push_back((d + 1, new_pos));
                        }
                    }
                },
            }
        }
        time
    }
}

fn main() {
    let ram: Vec<_> = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let computer = Computer::new(ram);
    let mut droid = Droid::new(computer);
    println!("{}", droid.bfs());
}
