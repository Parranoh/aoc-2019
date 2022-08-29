use std::io::stdin;

fn fuel(x: u32) -> u32 {
    if x < 6 {
        0
    } else {
        x / 3 - 2
    }
}

fn total_fuel(x: u32) -> u32 {
    let mut res = 0;
    let mut new = fuel(x);
    while new > 0 {
        res += new;
        new = fuel(new);
    }
    res
}

fn main() {
    println!("{}", stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .map(total_fuel)
        .reduce(|x, y| x + y)
        .unwrap());
}
