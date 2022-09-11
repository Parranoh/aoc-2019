use std::io::stdin;

type Vector = Vec<u8>;

fn fft(input: &Vector, offset: usize) -> Vector {
    let offset = offset + 1;
    let mut output = Vec::new();
    output.reserve(input.len());

    for i in 0..input.len() {
        let mut x: i32 = 0;
        let period = 2 * (i + offset);
        'outer: for j in 0.. {
            let base = i + 2 * j * period;
            for k in 0..i + offset {
                if base + k < input.len() {
                    x += input[base + k] as i32;
                } else {
                    break 'outer;
                }
            }
            let base = base + period;
            for k in 0..i + offset {
                if base + k < input.len() {
                    x -= input[base + k] as i32;
                } else {
                    break 'outer;
                }
            }
        }
        x = x.abs() % 10;
        output.push(x as u8);
    }

    output
}

fn main() {
    let mut signal: Vector = stdin().lines()
        .next().unwrap().unwrap()
        .as_bytes().into_iter()
        .map(|c| *c - b'0')
        .collect();
    for _ in 0..100 {
        signal = fft(&signal, 0);
    }
    println!("{}", signal.iter().take(8).map(|c| c.to_string()).collect::<String>());
}
