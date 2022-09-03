use std::io::stdin;

fn main() {
    let layers = {
        let mut res = Vec::new();
        let mut row = 0;
        let mut col = 0;
        stdin()
            .lines()
            .next().unwrap().unwrap()
            .as_bytes()
            .iter()
            .for_each(|c| {
                if col == 0 {
                    if row == 0 {
                        res.push(Vec::new());
                    }
                    res.last_mut().unwrap().push(Vec::new());
                }
                res.last_mut().unwrap().last_mut().unwrap().push(*c);
                col = (col + 1) % 25;
                row = (row + 1) % 6;
            });
        res
    };

    for row in 0..6 {
        for col in 0..25 {
            let mut pixel = None;
            for layer in layers.iter().rev() {
                match layer[row][col] {
                    b'0' => { pixel = Some('#'); },
                    b'1' => { pixel = Some('.'); },
                    _    => {},
                }
            }
            print!("{}", pixel.unwrap());
        }
        println!("");
    }
}
