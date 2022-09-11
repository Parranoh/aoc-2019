use std::io::stdin;
use std::thread;

type Vector = Vec<u8>;

fn fft(input: Vector, offset: usize) -> Vector {
    let num_threads = 8;
    if offset + 1 < input.len() {
        eprintln!("Using slow algorithm.");
        let offset = offset + 1;
        let mut output = Vec::new();
        output.reserve(input.len());

        let work_per_thread = input.len() / num_threads + 1;
        thread::scope(|s| {
            let mut threads: Vec<thread::ScopedJoinHandle<Vec<u8>>> = Vec::new();
            for t in 0..num_threads {
                let input = &input; // prevent capture
                threads.push(s.spawn(move || {
                    let mut res: Vec<u8> = Vec::new();
                    res.reserve(work_per_thread);
                    for i in 0.. {
                        let i = i * num_threads + t;
                        if i >= input.len() {
                            break;
                        }
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
                        res.push(x as u8);
                    }
                    res
                }));
            }
            let results: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
            for i in 0..input.len() {
                output.push(results[i % num_threads][i / num_threads]);
            }
        });

        output
    } else {
        let mut signal = input;
        let mut sum = 0;
        for x in signal.iter_mut().rev() {
            sum = (*x + sum) % 10;
            *x = sum;
        }
        signal
    }
}

fn main() {
    let input: Vector = stdin().lines()
        .next().unwrap().unwrap()
        .as_bytes().into_iter()
        .map(|c| *c - b'0')
        .collect();

    let offset: usize = input.iter().take(7).map(|c| c.to_string()).collect::<String>().parse().unwrap();
    let mut signal = Vec::new();
    let n = input.len();
    for i in offset..10000 * n {
        signal.push(input[i % n]);
    }
    // let offset = 10;
    // let mut signal = vec![1,1,1,1,1,1,1,1];

    for _ in 0..100 {
        signal = fft(signal, offset);
    }

    println!("{}", signal.iter().take(8).map(|c| c.to_string()).collect::<String>());
}
