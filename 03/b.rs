use std::io::stdin;
use std::collections::HashMap;

type Point = (i32, i32);

#[derive(Clone)]
#[derive(Copy)]
enum Dir {
    U, D, R, L
}
struct Step {
    dir: Dir,
    len: i32,
}

fn step(s: &str) -> Step {
    let len = (&s[1..]).parse().unwrap();
    let dir = match s.chars().next().unwrap() {
        'U' => Dir::U,
        'D' => Dir::D,
        'R' => Dir::R,
        'L' => Dir::L,
        _   => unreachable!(),
    };
    Step { dir: dir, len: len }
}

fn add(a: &Point, b: &Step) -> Point {
    match b.dir {
        Dir::U => (a.0, a.1 + b.len),
        Dir::D => (a.0, a.1 - b.len),
        Dir::R => (a.0 + b.len, a.1),
        Dir::L => (a.0 - b.len, a.1),
    }
}

fn main() {
    let wires: Vec<Vec<_>> = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(step)
                .collect()
        })
        .collect();

    let mut on = HashMap::new();

    let mut pos = (0, 0);
    let mut dist = 0;
    for s in &wires[0] {
        for i in 0..s.len {
            on.insert(add(&pos, &Step { dir: s.dir, len: i }), dist);
            dist += 1;
        }
        pos = add(&pos, s);
    }

    on.remove(&(0, 0));
    let mut min_dist: Option<i32> = None;

    let mut pos = (0, 0);
    let mut dist = 0;
    for s in &wires[1] {
        for i in 0..s.len {
            let p = &add(&pos, &Step { dir: s.dir, len: i });
            if let Some(d) = on.get(&p) {
                let d = d + dist;
                if let Some(m) = min_dist {
                    min_dist = Some(m.min(d));
                } else {
                    min_dist = Some(d);
                }
            }
            dist += 1;
        }
        pos = add(&pos, s);
    }

    println!("{}", min_dist.unwrap());
}
