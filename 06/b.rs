use std::io::stdin;
use std::collections::HashMap;

fn parse(l: &str) -> (String, String) {
    let com = l
        .chars()
        .take_while(|c| *c != ')')
        .collect();
    let sat = l
        .chars()
        .skip_while(|c| *c != ')')
        .skip(1)
        .collect();
    (sat, com)
}

fn get_dist(edges: &Vec<(String, String)>) -> usize {
    let parent: HashMap<&String, &String> = {
        let mut g = HashMap::new();
        for (v, w) in edges {
            g.insert(v, w);
        }
        g
    };

    let from = |name| {
        let mut d = Vec::new();
        d.push(name);
        while *d.last().unwrap() != &String::from("COM") {
            d.push(parent[d.last().unwrap()]);
        }
        d
    };
    let you = String::from("YOU");
    let from_you = from(&you);
    let san = String::from("SAN");
    let from_san = from(&san);

    let mut i = from_you.len() - 1;
    let mut j = from_san.len() - 1;
    while i > 0 && j > 0 && from_you[i] == from_san[j] {
        i -= 1;
        j -= 1;
    }

    i + j
}

fn main() {
    let edges: Vec<_> = stdin()
        .lines()
        .map(|l| parse(&l.unwrap()))
        .collect();
    println!("{}", get_dist(&edges));
}
