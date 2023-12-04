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
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (l, r) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

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

        Card { winning, have }
    }

    fn count_winning(&self) -> u32 {
        self.winning.iter().filter(|x| self.have.iter().any(|f| &f == x)).count() as u32
    }
}

fn process(input: &str) -> String {
    let cards = parse(input);

    cards.iter().map(|c| {
        if c.count_winning() > 1 {
            2u32.pow(c.count_winning().sub(1) as u32)
        }else{
            c.count_winning()
        }
    }).sum::<u32>().to_string()
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| Card::parse(line))
        .collect::<Vec<Card>>()
}
