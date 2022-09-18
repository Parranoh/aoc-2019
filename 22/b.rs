use std::io::stdin;

type Int = i128;
const NITERATIONS: Int = 101741582076661;
const NCARDS:      Int = 119315717514047;

enum Technique {
    Rev,
    CutTop(Int),
    CutBtm(Int),
    Deal(Int),
}
use Technique::*;

struct Affine {
    coef: Int,
    offs: Int,
}

impl Affine {
    fn compose(&self, other: &Self) -> Self {
        // (λx. ax + b) ∘ (λx. cx + d) = (λx. acx + ad + b)
        Self {
            coef: self.coef * other.coef % NCARDS,
            offs: (self.coef * other.offs + self.offs) % NCARDS,
        }
    }

    fn apply(&self, x: &Int) -> Int {
        (self.coef * *x + self.offs) % NCARDS
    }

    fn identity() -> Self {
        Self {
            coef: 1,
            offs: 0,
        }
    }
}

fn parse(s: &str) -> Technique {
    if s.starts_with("deal i") {
        Rev
    } else if s.starts_with("cut -") {
        let n = s[5..].parse().unwrap();
        CutBtm(n)
    } else if s.starts_with("cut") {
        let n = s[4..].parse().unwrap();
        CutTop(n)
    } else {
        let n = s[20..].parse().unwrap();
        Deal(n)
    }
}

impl Technique {
    fn invert(&self) -> Affine {
        match self {
            Rev => {
                Affine {
                    coef: NCARDS - 1,
                    offs: NCARDS - 1,
                }
            },
            CutTop(n) => {
                Affine {
                    coef: 1,
                    offs: *n,
                }
            },
            CutBtm(n) => {
                Affine {
                    coef: 1,
                    offs: NCARDS - *n,
                }
            },
            Deal(n) => {
                // input · n ≡ output (mod NCARDS)
                // <=> input ≡ output · n¯¹ (mod NCARDS)
                let inv = {
                    let mut t = 0;
                    let mut newt = 1;
                    let mut r = NCARDS;
                    let mut newr = *n;
                    while newr != 0 {
                        let q = r / newr;
                        (t, newt) = (newt, t - q * newt);
                        (r, newr) = (newr, r - q * newr);
                    }
                    if t < 0 {
                        t += NCARDS;
                    }
                    t
                };
                Affine {
                    coef: inv,
                    offs: 0,
                }
            },
        }
    }
}

fn main() {
    let input: Vec<Technique> = stdin().lines()
        .map(|l| parse(&l.unwrap()))
        .collect();

    let iteration = {
        let mut iteration = Affine::identity();
        for t in input {
            iteration = iteration.compose(&t.invert());
        }
        iteration
    };

    let perm = {
        let mut x = iteration;
        let mut n = NITERATIONS;
        let mut y = Affine::identity();
        while n > 1 {
            if n % 2 == 1 {
                y = x.compose(&y);
            }
            x = x.compose(&x);
            n /= 2;
        }
        x.compose(&y)
    };

    let res = perm.apply(&2020);
    println!("{res}");
}
