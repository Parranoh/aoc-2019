use std::io::stdin;

fn simplify(a: i32, b: i32) -> (i32, i32) {
    let (mut x, mut y) = (a.abs(), b.abs());
    while y > 0 {
        (x, y) = (y, x % y);
    }
    (a / x, b / x)
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
                            let (dx, dy) = simplify(i - x, j - y);
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
                best = best.max(Some(nvisible));
            }
        }
    }

    println!("{}", best.unwrap());
}
