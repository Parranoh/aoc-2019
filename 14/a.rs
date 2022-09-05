use std::collections::HashMap;
use std::io::stdin;

type Ing = (usize, String);
type Rec = (Vec<Ing>, Ing);

fn parse(l: String) -> Rec {
    let mut it = l.split(" => ");
    let inp = it.next().unwrap();
    let out = it.next().unwrap();
    let prec = |s: &str| {
        let mut it = s.split(' ');
        let n = it.next().unwrap().parse().unwrap();
        let i = it.next().unwrap();
        (n, String::from(i))
    };
    (inp.split(", ")
        .map(prec)
        .collect(),
    prec(out))
}

fn cost(input: &HashMap<String, Rec>) -> usize {
    let fuel = String::from("FUEL");
    let ore = String::from("ORE");
    let mut sources = vec![&fuel];

    let mut indeg: HashMap<&String, usize> = HashMap::new();
    for v in input.values() {
        for i in v.0.iter() {
            *indeg.entry(&i.1).or_default() += 1;
        }
    }

    let mut required: HashMap<_, _> = HashMap::new();
    required.insert(&fuel, 1);
    while let Some(v) = sources.pop() {
        if let Some(rec) = input.get(v) {
            let req = *required.entry(&v).or_default();
            let nbatches = (req - 1) / rec.1.0 + 1;
            for (n, i) in rec.0.iter() {
                *required.entry(&i).or_default() += nbatches * n;
                let d = indeg.entry(&i).or_default();
                *d -= 1;
                if *d == 0 {
                    sources.push(&i);
                }
            }
        }
    }

    required[&ore]
}

fn main() {
    let input: HashMap<_, _> = stdin().lines()
        .map(|l| parse(l.unwrap()))
        .map(|r| (r.1.1.clone(), r))
        .collect();

    println!("{}", cost(&input));
}
