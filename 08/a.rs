use std::io::stdin;

fn count(x: u8, layers: &Vec<Vec<u8>>) -> usize {
    layers
        .iter()
        .map(|row| row
            .iter()
            .filter(|c| **c == x)
            .count())
        .reduce(|a, b| a + b).unwrap()
}

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

    let layer = layers
        .iter()
        .min_by_key(|l| count(b'0', l)).unwrap();

    let res = count(b'1', layer) * count(b'2', layer);
    println!("{res}");
}
