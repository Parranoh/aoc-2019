use std::collections::HashSet;
use std::collections::HashMap;
use std::io::stdin;

struct Computer {
    ram: HashMap<usize, i64>,
    pc: usize,
    relative_base: i64,
    input: Vec<i64>,
    ic: usize,
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
            input: Vec::new(),
            ic: 0,
        }
    }

    fn cycle(&mut self) -> (bool, Option<i64>) {
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
                *self.param(1) = self.input[self.ic];
                self.ic += 1;
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

#[derive(PartialEq)]
enum Instr {
    L, R, M(u32),
    A, B, C,
}

fn to_string(program: &[Instr]) -> Option<Vec<i64>> {
    let mut res = Vec::new();
    let mut first = true;
    for i in program {
        if !first {
            res.push(b',' as i64);
        } else {
            first = false;
        }

        for c in match i {
            Instr::L    => String::from("L"),
            Instr::R    => String::from("R"),
            Instr::M(n) => n.to_string(),
            Instr::A    => String::from("A"),
            Instr::B    => String::from("B"),
            Instr::C    => String::from("C"),
        }.as_bytes() {
            res.push((*c) as i64);
        }
    }
    if program.len() <= 20 {
        Some(res)
    } else {
        None
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

    let mut map = HashSet::new();
    let mut pos = (0_i32, 0_i32);
    let mut robot_pos = (0, 0);
    let mut robot_dir: u8 = 0;

    {
        let mut empty_row = true;
        loop {
            if let (false, Some(o)) = computer.cycle() {
                match o as u8 {
                    b'.'  => {
                        empty_row = false;
                        pos.1 += 1;
                    },
                    b'\n' => {
                        if empty_row {
                            break;
                        }
                        empty_row = true;
                        pos = (pos.0 + 1, 0);
                    }
                    _     => {
                        empty_row = false;
                        map.insert(pos);
                        if o as u8 != b'#' {
                            robot_pos = pos;
                            robot_dir = match o as u8 {
                                b'>' => 0,
                                b'^' => 1,
                                b'<' => 2,
                                b'v' => 3,
                                _    => unreachable!(),
                            };
                        }
                        pos.1 += 1;
                    }
                }
            }
        }
    }
    let map = map;

    let mut program = Vec::new();
    loop {
        let e = (robot_pos.0, robot_pos.1 + 1);
        let n = (robot_pos.0 - 1, robot_pos.1);
        let w = (robot_pos.0, robot_pos.1 - 1);
        let s = (robot_pos.0 + 1, robot_pos.1);
        let (f, l, r) = match robot_dir {
            0 => (e, n, s),
            1 => (n, w, e),
            2 => (w, s, n),
            3 => (s, e, w),
            _ => unreachable!(),
        };
        if map.contains(&f) {
            if let Some(Instr::M(x)) = program.last_mut() {
                *x += 1;
            } else {
                program.push(Instr::M(1));
            }
            robot_pos = f;
        } else if map.contains(&l) {
            program.push(Instr::L);
            robot_dir = (robot_dir + 1) % 4;
        } else if map.contains(&r) {
            program.push(Instr::R);
            robot_dir = (robot_dir + 3) % 4;
        } else {
            break;
        }
    }
    let program = program;

    'outer: for i in 1..program.len() {
        let mut pc = 0;
        let a = &program[0..i];
        if let Some(print_a) = to_string(a) {
            while program[pc..].starts_with(a) {
                pc += i;
            }
            for j in 1..program.len() - pc {
                let mut pc = pc;
                let b = &program[pc..pc + j];
                if let Some(print_b) = to_string(b) {
                    loop {
                        if program[pc..].starts_with(a) {
                            pc += i;
                        } else if program[pc..].starts_with(b) {
                            pc += j;
                        } else {
                            break;
                        }
                    }
                    for k in 1..program.len() - pc {
                        let c = &program[pc..pc + k];
                        if let Some(print_c) = to_string(c) {
                            let mut m = Vec::new();
                            let mut pc = 0;
                            loop {
                                if program[pc..].starts_with(a) {
                                    pc += i;
                                    m.push(Instr::A);
                                } else if program[pc..].starts_with(b) {
                                    pc += j;
                                    m.push(Instr::B);
                                } else if program[pc..].starts_with(c) {
                                    pc += k;
                                    m.push(Instr::C);
                                } else {
                                    break;
                                }
                            }
                            if pc == program.len() {
                                if let Some(print_main) = to_string(&m) {
                                    computer.input.extend(&print_main);
                                    computer.input.push(b'\n' as i64);
                                    computer.input.extend(&print_a);
                                    computer.input.push(b'\n' as i64);
                                    computer.input.extend(&print_b);
                                    computer.input.push(b'\n' as i64);
                                    computer.input.extend(&print_c);
                                    computer.input.push(b'\n' as i64);
                                    computer.input.push(b'n' as i64);
                                    computer.input.push(b'\n' as i64);
                                    break 'outer;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    let mut res = None;
    while let (false, o) = computer.cycle() {
        if let Some(_) = o {
            res = o;
        }
    }
    println!("{}", res.unwrap());
}
