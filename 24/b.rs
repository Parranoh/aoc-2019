use std::io::stdin;

fn main() {
    let state: Vec<Vec<bool>> = stdin()
        .lines()
        .map(|l| l
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect())
        .collect();
    let mut state = vec![state];

    const NITERATIONS: u8 = 200;
    const DIM: usize = 5;

    for _ in 0..NITERATIONS {
        let mut new_state = Vec::new();
        new_state.reserve(state.len() + 2);
        for d in 0..state.len() + 2 {
            let mut new_layer = Vec::new();
            new_layer.reserve(DIM);
            for y in 0..DIM {
                let mut new_row = Vec::new();
                new_row.reserve(DIM);
                for x in 0..DIM {
                    let num_adjacent = {
                        let mut n: u8 = 0;
                        if (1..state.len() + 1).contains(&d) {
                            // on same layer
                            if y > 0 && state[d - 1][y - 1][x] {
                                n += 1;
                            }
                            if x > 0 && state[d - 1][y][x - 1] {
                                n += 1;
                            }
                            if y + 1 < DIM && state[d - 1][y + 1][x] {
                                n += 1;
                            }
                            if x + 1 < DIM && state[d - 1][y][x + 1] {
                                n += 1;
                            }
                        }
                        if (2..).contains(&d) {
                            // on outer layer
                            if y == 0 && state[d - 2][1][2] {
                                n += 1;
                            }
                            if x == 0 && state[d - 2][2][1] {
                                n += 1;
                            }
                            if y == 4 && state[d - 2][3][2] {
                                n += 1;
                            }
                            if x == 4 && state[d - 2][2][3] {
                                n += 1;
                            }
                        }
                        if (..state.len()).contains(&d) {
                            // on inner layer
                            for i in 0..5 {
                                if match (y, x) {
                                    (1, 2) => state[d][0][i],
                                    (2, 1) => state[d][i][0],
                                    (3, 2) => state[d][4][i],
                                    (2, 3) => state[d][i][4],
                                    _      => false,
                                } {
                                    n += 1;
                                }
                            }
                        }
                        n
                    };
                    let cell = if d > 0 && d <= state.len() {
                        state[d - 1][y][x]
                    } else {
                        false
                    };
                    let new_cell = match (cell, num_adjacent) {
                        (_,     1) => true,
                        (false, 2) => true,
                        _          => false,
                    };
                    new_row.push(new_cell);
                }
                new_layer.push(new_row);
            }
            new_state.push(new_layer);
        }
        state = new_state;
        for layer in &mut state {
            layer[2][2] = false;
        }
    }

    let mut res: usize = 0;
    for layer in state {
        for row in layer {
            for cell in row {
                if cell {
                    res += 1;
                }
            }
        }
    }

    println!("{res}");
}
