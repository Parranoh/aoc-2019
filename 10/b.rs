use std::io::stdin;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq)]
struct Angle {
    x: i32,
    y: i32,
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        let greater_pi = |a: &Angle| a.y < 0 || a.y == 0 && a.x > 0;
        match greater_pi(self).cmp(&greater_pi(other)) {
            Ordering::Equal => {
                (self.x * other.y).cmp(&(other.x * self.y))
            },
            o => o,
        }
    }
}
impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn simplify(a: i32, b: i32) -> Angle {
    let (mut x, mut y) = (a.abs(), b.abs());
    while y > 0 {
        (x, y) = (y, x % y);
    }
    Angle { x: a / x, y: b / x }
}

fn main() {
    let map: Vec<Vec<bool>> = stdin()
        .lines()
        .map(|l| l.unwrap()
            .into_bytes()
            .into_iter()
            .map(|c| c == b'#')
            .collect())
        .collect();
    let nrows = map.len() as i32;
    let ncols = map[0].len() as i32;


    let mut best = None;
    for x in 0_i32..nrows {
        for y in 0_i32..ncols {
            if map[x as usize][y as usize] {
                let mut visible = map.clone();
                visible[x as usize][y as usize] = false;
                for i in 0_i32..nrows {
                    for j in 0_i32..ncols {
                        if (i, j) != (x, y) && visible[i as usize][j as usize] {
                            let Angle { x: dx, y: dy } = simplify(i - x, j - y);
                            let mut k = 1_i32;
                            while (0_i32..nrows).contains(&(i + k * dx)) && (0_i32..ncols).contains(&(j + k * dy)) {
                                visible[(i + k * dx) as usize][(j + k * dy) as usize] = false;
                                k += 1;
                            }
                        }
                    }
                }
                let visible = visible;

                let nvisible = visible
                    .into_iter()
                    .map(|r| r
                        .into_iter()
                        .filter(|b| *b)
                        .count())
                    .reduce(|a, b| a + b).unwrap();
                best = best.max(Some((nvisible, x, y)));
            }
        }
    }

    let (_, x, y) = best.unwrap();
    let mut by_angle: BTreeMap<Angle, Vec<(i32, i32)>> = BTreeMap::new();
    for i in 0_i32..nrows {
        for j in 0_i32..ncols {
            if (x, y) != (i, j) && map[i as usize][j as usize] {
                by_angle.entry(simplify(i - x, j - y)).or_default().push((i, j));
            }
        }
    }

    for v in by_angle.values_mut() {
        v.sort_by_key(|(i, j)| (i - x).abs() + (j - y).abs());
    }

    let mut it = by_angle.values_mut();
    let (mut x, mut y) = (-1, -1);
    for _ in 0..200 {
        loop {
            match it.next() {
                None    => { it = by_angle.values_mut(); },
                Some(v) => match v.pop() {
                    None    => {},
                    Some(a) => {
                        (x, y) = a;
                        break;
                    },
                },
            }
        }
    };
    println!("{}", 100 * y + x)
}
