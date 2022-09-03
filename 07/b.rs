use std::io::stdin;

struct Computer {
    ram: Vec<i32>,
    pc: usize,
    ic: usize,
}

enum PC {
    Inc(usize),
    Set(usize),
}

impl Computer {
    fn new(ram: Vec<i32>) -> Computer {
        let res = Computer {
            ram: ram,
            pc: 0,
            ic: 0,
        };
        res
    }

    fn cycle(&mut self, input: &mut Vec<i32>, output: &mut Vec<i32>) -> bool {
        let (dpc, success) = match self.op() {
            1 => {
                *self.param(3) = *self.param(1) + *self.param(2);
                (PC::Inc(4), true)
            },
            2 => {
                *self.param(3) = *self.param(1) * *self.param(2);
                (PC::Inc(4), true)
            },
            3 => {
                if self.ic < input.len() {
                    *self.param(1) = input[self.ic];
                    self.ic += 1;
                    (PC::Inc(2), true)
                } else {
                    (PC::Inc(0), false)
                }
            },
            4 => {
                let tmp = *self.param(1);
                output.push(tmp);
                (PC::Inc(2), true)
            },
            5 => {
                (if *self.param(1) != 0 {
                    PC::Set(*self.param(2) as usize)
                } else {
                    PC::Inc(3)
                }, true)
            },
            6 => {
                (if *self.param(1) == 0 {
                    PC::Set(*self.param(2) as usize)
                } else {
                    PC::Inc(3)
                }, true)
            },
            7 => {
                *self.param(3) = if *self.param(1) < *self.param(2) { 1 } else { 0 };
                (PC::Inc(4), true)
            },
            8 => {
                *self.param(3) = if *self.param(1) == *self.param(2) { 1 } else { 0 };
                (PC::Inc(4), true)
            },
            99 => (PC::Inc(0), false),
            _ => unreachable!(),
        };
        match dpc {
            PC::Inc(d) => self.pc += d,
            PC::Set(a) => self.pc = a,
        };
        success
    }

    fn op(&self) -> i32 {
        self.opcode() % 100
    }

    fn param(&mut self, i: usize) -> &mut i32 {
        match self.parammode(i) {
            0 => {
                let addr = self.ram[self.pc + i];
                assert!(addr >= 0);
                &mut self.ram[addr as usize]
            },
            1 => &mut self.ram[self.pc + i],
            _ => unreachable!(),
        }
    }

    fn parammode(&self, i: usize) -> i32 {
        let v = 10_i32.pow(i as u32 + 1);
        (self.opcode() / v) % 10
    }

    fn opcode(&self) -> i32 {
        self.ram[self.pc]
    }
}

fn run(ram: &Vec<i32>, settings: &Vec<i32>) -> i32 {
    let mut computers = Vec::new();
    let mut inputs = Vec::new();
    for i in 0..5 {
        computers.push(Computer::new(ram.clone()));
        inputs.push(vec![settings[i]]);
    }

    inputs[0].push(0);
    let mut some_ran = true;
    while some_ran {
        some_ran = false;
        for active in 0..5 {
            let next = (active + 1) % 5;
            let (input, output) = {
                if next == 0 {
                    let (last, init) = inputs.split_last_mut().unwrap();
                    (last, init.get_mut(0).unwrap())
                } else {
                    let (head, tail) = inputs[active..].split_first_mut().unwrap();
                    (head, tail.get_mut(0).unwrap())
                }
            }; // thank you, Rust
            some_ran |= computers[active].cycle(input, output);
        }
    }

    *inputs[0].last().unwrap()
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

    let mut max = None;
    for i in 5..=9 {
        for j in 5..=9 {
            if j == i { continue; }
            for k in 5..=9 {
                if k == i || k == j { continue; }
                for l in 5..=9 {
                    if l == i || l == j || l == k { continue; }
                    for m in 5..=9 {
                        if m == i || m == j || m == k || m == l { continue; }
                        let output = run(&ram, &vec![i, j, k, l, m]);
                        max = max.max(Some(output));
                    }
                }
            }
        }
    }
    println!("{}", max.unwrap());
}
