use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let mut state: Vec<Vec<bool>> = stdin()
        .lines()
        .map(|l| l
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect())
        .collect();

    let mut history = HashSet::new();
    while !history.contains(&state) {
        history.insert(state.clone());
        let mut new_state = Vec::new();
        new_state.reserve(state.len());
        for y in 0..state.len() {
            let mut new_row = Vec::new();
            new_row.reserve(state[y].len());
            for x in 0..state[y].len() {
                let num_adjacent = {
                    let mut n: u8 = 0;
                    if y > 0 && state[y - 1][x] {
                        n += 1;
                    }
                    if x > 0 && state[y][x - 1] {
                        n += 1;
                    }
                    if y + 1 < state.len() && state[y + 1][x] {
                        n += 1;
                    }
                    if x + 1 < state[y].len() && state[y][x + 1] {
                        n += 1;
                    }
                    n
                };
                new_row.push(match (state[y][x], num_adjacent) {
                    (_, 1) => true,
                    (false, 2) => true,
                    _ => false,
                });
            }
            new_state.push(new_row);
        }
        state = new_state;
    }

    let mut res: u32 = 0;
    let mut pnts = 1;
    for row in state {
        for cell in row {
            if cell {
                res += pnts;
            }
            pnts *= 2;
        }
    }

    println!("{res}");
}
