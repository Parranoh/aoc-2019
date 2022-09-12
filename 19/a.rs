use std::io::stdin;
use std::collections::HashMap;

struct Computer {
    ram: HashMap<usize, i64>,
    input: Vec<i64>,
    pc: usize,
    ic: usize,
    output: Vec<i64>,
    relative_base: i64,
}

enum PC {
    Inc(usize),
    Set(usize),
}

impl Computer {
    fn new(ram: &Vec<i64>, input: Vec<i64>) -> Computer {
        Computer {
            ram: ram
                .iter()
                .map(|x| *x)
                .enumerate()
                .collect(),
            input: input,
            pc: 0,
            ic: 0,
            output: Vec::new(),
            relative_base: 0,
        }
    }

    fn exec(&mut self) {
        while !self.cycle() {}
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
                *self.param(1) = self.input[self.ic];
                self.ic += 1;
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

    let mut res = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut computer = Computer::new(&ram, vec![x, y]);
            computer.exec();
            res += computer.output[0];
            print!("{}", if computer.output[0] == 0 { " " } else { "#" });
        }
        println!("");
    }
    println!("{}", res);
}
