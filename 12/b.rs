use std::collections::HashMap;
use std::io::stdin;

type System = Vec<(i32, i32)>;

fn gravity(sys: &mut System) {
    for i in 0..sys.len() {
        for j in 0..sys.len() {
            sys[i].1 += (sys[j].0 - sys[i].0).signum();
        }
    }
}

fn velocity(sys: &mut System) {
    for (x, v) in sys {
        *x += *v;
    }
}

fn step(sys: &mut System) {
    gravity(sys);
    velocity(sys);
}

fn period(mut sys: System) -> (usize, usize) {
    let mut history: HashMap<Vec<(i32, i32)>, usize> = HashMap::new();
    let mut i = 0;
    loop {
        history.insert(sys.clone(), i);
        step(&mut sys);
        i += 1;
        if let Some(first) = history.get(&sys) {
            break (*first, i - first);
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    let input: Vec<Vec<i32>> = stdin()
        .lines()
        .map(|l| l.unwrap()
            .split(',')
            .map(|s| s
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '-')
                .collect::<String>()
                .parse().unwrap())
            .collect())
        .collect();
    let dims = {
        let mut dims: Vec<System> = Vec::new();
        for d in 0..input[0].len() {
            dims.push(input.iter().map(|v| (v[d], 0)).collect());
        }
        dims
    };

    let periods: Vec<_> = dims.into_iter().map(period).collect();
    let start = *periods.iter().map(|(s, _)| s).reduce(|a, b| a.max(b)).unwrap();
    let period = periods.into_iter().map(|(_, p)| p).reduce(lcm).unwrap();
    let res = start + period;
    println!("{res}");
}
