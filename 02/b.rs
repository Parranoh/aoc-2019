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

fn test(ram: &Vec<usize>, noun: usize, verb: usize) -> Option<usize> {
    let mut ram = ram.clone();
    ram[1] = noun;
    ram[2] = verb;
    exec(&mut ram);
    if ram[0] == 19690720 {
        Some(100 * noun + verb)
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
    for noun in 0..100 {
        for verb in 0..100 {
            if let Some(res) = test(&ram, noun, verb) {
                println!("{}", res);
                return;
            }
        }
    }
}
