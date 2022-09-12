use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;

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
    let mut portals = HashMap::new();
    let mut reverse = HashMap::new();
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
                        if let Some((a, b)) = portals.get_mut(&name) {
                            *a += x;
                            *b += y;
                        } else {
                            portals.insert(name.clone(), (x, y));
                        }
                        reverse.insert((x, y), name);
                    }
                }
            }
        }
    }

    let mut visited: Vec<Vec<bool>> = input.iter()
        .map(|r| r.iter()
            .map(|_| false)
            .collect())
        .collect();
    let start  = portals[&[b'A', b'A']];
    let target = portals[&[b'Z', b'Z']];
    let mut q = VecDeque::from([(0, start)]);
    while let Some((c, (x, y))) = q.pop_front() {
        if (x, y) == target {
            println!("{c}");
            break;
        }
        if let Some(p) = reverse.get(&(x, y)) {
            let (px, py) = portals[p];
            let (x, y) = (px - x, py - y);
            if map[y][x] && !visited[y][x] {
                visited[y][x] = true;
                q.push_back((c + 1, (x, y)));
            }
        }
        for (x, y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if map[y][x] && !visited[y][x] {
                visited[y][x] = true;
                q.push_back((c + 1, (x, y)));
            }
        }
    }
}
