use std::io::stdin;

fn exec(ram: &mut Vec<usize>) {
    let mut pc = 0;
    loop {
        match ram[pc] {
            1 => {
                let x = ram[pc + 1];
                let y = ram[pc + 2];
                let z = ram[pc + 3];
                ram[z] = ram[x] + ram[y];
                pc += 4;
            },
            2 => {
                let x = ram[pc + 1];
                let y = ram[pc + 2];
                let z = ram[pc + 3];
                ram[z] = ram[x] * ram[y];
                pc += 4;
            },
            99 => break,
            _ => unreachable!()
        }
    }
}

fn main() {
    let mut ram: Vec<_> = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();
    ram[1] = 12;
    ram[2] = 2;
    exec(&mut ram);
    println!("{}", ram[0]);
}
