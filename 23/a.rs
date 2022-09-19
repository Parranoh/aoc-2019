use std::io::stdin;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Computer {
    ram: HashMap<usize, i64>,
    input: VecDeque<i64>,
    pc: usize,
    output: Vec<i64>,
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
            input: VecDeque::new(),
            pc: 0,
            output: Vec::new(),
            relative_base: 0,
        }
    }

    fn cycle(&mut self) -> bool {
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
                *self.param(1) = self.input.pop_front().unwrap_or(-1);
                (PC::Inc(2), false)
            },
            4 => {
                let tmp = *self.param(1);
                self.output.push(tmp);
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
        halt
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

    let mut computers = Vec::new();
    computers.reserve(50);
    for i in 0..50 {
        let mut c = Computer::new(ram.clone());
        c.input.push_back(i);
        computers.push(c);
    }

    let res = 'outer: loop {
        for i in 0..50 {
            let c = &mut computers[i];
            c.cycle();
            if c.output.len() == 3 {
                let y = c.output.pop().unwrap();
                let x = c.output.pop().unwrap();
                let addr = c.output.pop().unwrap();
                if addr == 255 {
                    break 'outer y;
                } else if (0..50).contains(&addr) {
                    computers[addr as usize].input.push_back(x);
                    computers[addr as usize].input.push_back(y);
                }
            }
        }
    };

    println!("{res}");
}
