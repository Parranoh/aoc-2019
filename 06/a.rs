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

fn toposort(edges: &Vec<(String, String)>) -> usize {
    let mut graph: HashMap<&String, Vec<&String>> = HashMap::new();
    let mut indeg: HashMap<&String, usize> = HashMap::new();
    for (v, w) in edges {
        graph.entry(v).or_default().push(w);
        *indeg.entry(w).or_default() += 1;
    }
    let graph = graph;

    let mut pindeg: HashMap<&String, usize> = HashMap::new();
    let mut res = 0;
    let mut sources: Vec<(&String, &Vec<&String>)> = graph
        .iter()
        .filter(|(k, _)| indeg.get(*k).is_none())
        .map(|(k, v)| (*k, v))
        .collect();
    let empty = Vec::new();
    while let Some(v) = sources.pop() {
        let n = *pindeg.entry(v.0).or_default();
        res += n;
        for w in v.1.iter() {
            *pindeg.entry(w).or_default() += n + 1;
            *indeg.get_mut(w).unwrap() -= 1;
            if indeg[w] == 0 {
                sources.push((*w, graph.get(w).unwrap_or(&empty)));
            }
        }
    }
    res
}

fn main() {
    let edges: Vec<_> = stdin()
        .lines()
        .map(|l| parse(&l.unwrap()))
        .collect();
    println!("{}", toposort(&edges));
}
