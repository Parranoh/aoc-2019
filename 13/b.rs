use std::cmp::Ordering;
use std::collections::HashMap;
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
    fn new(ram: Vec<i64>) -> Computer {
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
    computer.ram.insert(0, 2);
    let mut paddlex = None;
    let mut ballx = None;
    let mut score = None;
    let mut output = Vec::new();
    while let (false, o) = computer.cycle(match ballx.cmp(&paddlex) {
        Ordering::Less    => -1,
        Ordering::Equal   =>  0,
        Ordering::Greater =>  1,
    }) {
        if let Some(o) = o {
            output.push(o);
            if output.len() == 3 {
                if output[0] == -1 && output[1] == 0 {
                    score = Some(output[2]);
                } else if output[2] == 3 {
                    paddlex = Some(output[0]);
                } else if output[2] == 4 {
                    ballx = Some(output[0]);
                }
                output.clear();
            }
        }
    }

    println!("{}", score.unwrap());
}
