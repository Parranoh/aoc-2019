use std::io::stdin;

enum Technique {
    Rev,
    CutTop(usize),
    CutBtm(usize),
    Deal(usize),
}
use Technique::*;

type Deck = Vec<u16>;

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
    fn apply(&self, deck: &mut Deck) {
        match self {
            Rev => {
                deck.reverse();
            },
            CutTop(n) => {
                deck.rotate_left(*n);
            },
            CutBtm(n) => {
                deck.rotate_right(*n);
            },
            Deal(n) => {
                let mut new_deck = Vec::new();
                new_deck.resize(deck.len(), 0);
                let mut dest = 0;
                for src in 0..deck.len() {
                    new_deck[dest] = deck[src];
                    dest = (dest + n) % new_deck.len();
                }
                *deck = new_deck;
            },
        }
    }
}

fn main() {
    let input: Vec<Technique> = stdin().lines()
        .map(|l| parse(&l.unwrap()))
        .collect();
    let mut deck: Deck = (0..10007).collect();
    for t in input {
        t.apply(&mut deck);
    }
    let res = (0..deck.len()).find(|&i| deck[i] == 2019).unwrap();
    println!("{res}");
}
