use std::io::stdin;

struct Computer {
    ram: Vec<i32>,
    input: Vec<i32>,
    pc: usize,
    ic: usize,
    output: Vec<i32>,
}

impl Computer {
    fn new(ram: Vec<i32>, input: Vec<i32>) -> Computer {
        let res = Computer {
            ram: ram,
            input: input,
            pc: 0,
            ic: 0,
            output: Vec::new(),
        };
        res
    }

    fn exec(&mut self) {
        while !self.cycle() {}
    }

    fn cycle(&mut self) -> bool {
        let (dpc, halt) = match self.op() {
            1 => {
                *self.param(3) = *self.param(1) + *self.param(2);
                (4, false)
            },
            2 => {
                *self.param(3) = *self.param(1) * *self.param(2);
                (4, false)
            },
            3 => {
                *self.param(1) = self.input[self.ic];
                self.ic += 1;
                (2, false)
            },
            4 => {
                let tmp = *self.param(1);
                self.output.push(tmp);
                (2, false)
            },
            99 => (0, true),
            _ => unreachable!(),
        };
        self.pc += dpc;
        halt
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

fn main() {
    let ram: Vec<_> = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();
    let mut test = Computer::new(ram, vec![1]);
    test.exec();
    println!("{}", test.output.last().unwrap());
}
