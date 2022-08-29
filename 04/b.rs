use std::io::stdin;

fn main() {
    let range: Vec<_> = stdin()
        .lines()
        .next().unwrap().unwrap()
        .split('-')
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    let mut res = 0;
    'outer: for i in 111_111..1_000_000 {
        if i < range[0] { continue; }
        if i > range[1] { continue; }
        let s = i.to_string();
        let s = s.as_bytes();
        let mut double = false;
        let mut group_len = 1;
        for j in 0..5 {
            if s[j] > s[j + 1] { continue 'outer; }

            if !double {
                if s[j] != s[j + 1] {
                    if group_len == 2 {
                        double = true;
                    }
                    group_len = 0;
                }
                group_len += 1;
            }
        }
        double |= group_len == 2;
        if !double { continue; }
        res += 1;
    }
    println!("{}", res);
}
