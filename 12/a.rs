use std::io::stdin;

type Pos = (i32, i32, i32);
type System = Vec<(Pos, Pos)>;

fn gravity(sys: &mut System) {
    for i in 0..sys.len() {
        for j in 0..sys.len() {
            sys[i].1.0 += (sys[j].0.0 - sys[i].0.0).signum();
            sys[i].1.1 += (sys[j].0.1 - sys[i].0.1).signum();
            sys[i].1.2 += (sys[j].0.2 - sys[i].0.2).signum();
        }
    }
}

fn velocity(sys: &mut System) {
    for (x, v) in sys {
        x.0 += v.0;
        x.1 += v.1;
        x.2 += v.2;
    }
}

fn step(sys: &mut System) {
    gravity(sys);
    velocity(sys);
}

fn energy(sys: &System) -> i32 {
    sys
        .iter()
        .map(|(x, v)| (x.0.abs() + x.1.abs() + x.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs()))
        .reduce(|a, b| a + b).unwrap()
}

fn main() {
    let mut sys: System = stdin()
        .lines()
        .map(|l| l.unwrap()
            .split(',')
            .map(|s| s
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '-')
                .collect::<String>()
                .parse().unwrap())
            .collect::<Vec<_>>())
        .map(|v| ((v[0], v[1], v[2]), (0, 0, 0)))
        .collect();

    for _ in 0..1000 {
        step(&mut sys);
    }

    println!("{}", energy(&sys));
}
