use std::io::stdin;

fn main() {
    println!("{}", stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .map(|x| x / 3 - 2)
        .reduce(|x, y| x + y)
        .unwrap());
}
