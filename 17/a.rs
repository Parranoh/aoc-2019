use std::collections::HashMap;
use std::collections::HashSet;
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

fn map(computer: &mut Computer) -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();
    let mut pos = (0, 0);
    while let (false, o) = computer.cycle(0) {
        match o {
            None     => {},
            Some(10) => {
                pos = (pos.0 + 1, 0);
            },
            Some(46) => {
                pos.1 += 1;
            },
            Some(_)  => {
                res.insert(pos);
                pos.1 += 1;
            },
        }
    }
    res
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

    let mut computer = Computer::new(ram);
    let m = map(&mut computer);
    let mut res = 0;
    for (x, y) in &m {
        if m.contains(&(x - 1, *y)) && m.contains(&(x + 1, *y)) && m.contains(&(*x, y - 1)) && m.contains(&(*x, y + 1)) {
            res += x * y;
        }
    }
    println!("{res}");
}
