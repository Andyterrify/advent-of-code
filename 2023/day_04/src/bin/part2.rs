use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    num: u32,
    count: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (f, rest) = line.split_once(": ").unwrap();
        let (l, r) = rest.split_once(" | ").unwrap();

        let winning = l
            .split_whitespace()
            .map(|x| x.trim())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let have = r
            .split_whitespace()
            .map(|x| x.trim())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let num = f
            .strip_prefix("Card ")
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        Card {
            num,
            count: 1,
            winning,
            have,
        }
    }

    fn count_winning(&self) -> u32 {
        self.winning
            .iter()
            .filter(|x| self.have.iter().any(|f| &f == x))
            .count() as u32
    }

    fn increment_by(&mut self, i:u32) {
        self.count += i
    }
}

fn process(input: &str) -> String {
    let mut cards = parse(input);

    (1..=194).for_each(|i| {
        let a = cards.get(&i).unwrap();
        let b = a.count_winning();
        let count = a.count;
        dbg!(b);

        (i+1..=(i + b)).for_each(|j| {
            cards.get_mut(&j).unwrap().increment_by(count);
        });
    });

    let b: u32 = cards.iter().map(|(_, x)| x.count).sum();
    dbg!(b);

    todo!()
}

fn parse(input: &str) -> HashMap<u32, Card> {
    input
        .lines()
        .map(|line| {
            let c = Card::parse(line);
            (c.num.clone(), c)
        })
        .collect()
}
