use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;

type Pos = (usize, usize);

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lines()
        .map(|l| l.unwrap()
            .as_bytes()
            .to_vec())
        .collect();
    let map: Vec<Vec<bool>> = input.iter()
        .map(|r| r.iter()
            .map(|c| *c == b'.')
            .collect())
        .collect();
    let mut portals_by_name = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] {
                for name in [
                    [input[y - 2][x], input[y - 1][x]],
                    [input[y + 1][x], input[y + 2][x]],
                    [input[y][x - 2], input[y][x - 1]],
                    [input[y][x + 1], input[y][x + 2]],
                ] {
                    if name[0].is_ascii_uppercase() && name[1].is_ascii_uppercase() {
                        portals_by_name.entry(name).or_insert(Vec::new()).push((x, y));
                    }
                }
            }
        }
    }
    let mut portals: HashMap<Pos, (Pos, bool)> = HashMap::new();
    let mut start = None;
    let mut target = None;
    for (name, coords) in portals_by_name {
        if name == [b'A', b'A'] {
            assert_eq!(coords.len(), 1);
            start = Some(coords[0]);
        } else if name == [b'Z', b'Z'] {
            assert_eq!(coords.len(), 1);
            target = Some(coords[0]);
        } else {
            assert_eq!(coords.len(), 2);
            let first_is_outer = [2, map.len() - 3].contains(&coords[0].1) || [2, map[0].len() - 3].contains(&coords[0].0);
            portals.insert(coords[0], (coords[1], first_is_outer));
            portals.insert(coords[1], (coords[0], !first_is_outer));
        }
    }
    let start  = start.unwrap();
    let target = target.unwrap();
    let start  = (start.0,  start.1,  0);
    let target = (target.0, target.1, 0);

    let none_visited: Vec<Vec<bool>> = input.iter()
        .map(|r| r.iter()
            .map(|_| false)
            .collect())
        .collect();
    let mut visited = vec![none_visited.clone()];
    let mut q = VecDeque::from([(0, start)]);
    while let Some((c, (x, y, d))) = q.pop_front() {
        if (x, y, d) == target {
            println!("{c}");
            break;
        }
        if let Some(((x, y), outer)) = portals.get(&(x, y)) {
            let (x, y, outer) = (*x, *y, *outer);
            if d > 0 || !outer {
                let d = if outer { d - 1 } else { d + 1 };
                if d >= visited.len() {
                    visited.push(none_visited.clone());
                }
                if map[y][x] && !visited[d][y][x] {
                    visited[d][y][x] = true;
                    q.push_back((c + 1, (x, y, d)));
                }
            }
        }
        for (x, y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if map[y][x] && !visited[d][y][x] {
                visited[d][y][x] = true;
                q.push_back((c + 1, (x, y, d)));
            }
        }
    }
}
