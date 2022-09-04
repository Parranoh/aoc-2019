use std::io::stdin;
use std::collections::HashMap;

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
    let mut hull = HashMap::new();
    let mut pos = (0, 0);
    let mut dir = 0;
    let mut paint = true;
    while let (false, output) = computer.cycle(*hull.get(&pos).unwrap_or(&false) as i64) {
        if let Some(o) = output {
            if paint {
                hull.insert(pos, o == 1);
            } else {
                dir = (dir + 2 * o - 1 + 4) % 4;
                match dir {
                    0 => { pos.1 += 1; },
                    1 => { pos.0 += 1; },
                    2 => { pos.1 -= 1; },
                    3 => { pos.0 -= 1; },
                    _ => unreachable!(),
                }
            }
            paint ^= true;
        }
    }

    println!("{}", hull.len());
}
