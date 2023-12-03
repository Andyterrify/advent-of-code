use std::{
    collections::{hash_map::ValuesMut, BTreeMap},
    env::vars,
    vec,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);

    // 2286 is too low
    // 2248 is too low
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Empty,
    Symbol,
    Number(usize),
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    coord: Coord,
    value: Value,
}

fn process(input: &str) -> String {
    let mut matrix = input.lines().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().map(move |(x, c)| Point {
            coord: Coord { y, x },
            value: match c {
                '.' => Value::Empty,
                c if c.is_ascii_digit() => Value::Number(c.to_digit(10).expect("Should not failt").try_into().unwrap()),
                _ => Value::Symbol,
            },
        })
    });

    let mut grouped_numbers: Vec<Point> = vec![];

    matrix
        .filter(|x| match x.value {
            Value::Number(_) => true,
            _ => false,
        })
        .for_each(|x| match x.value {
            Value::Number(_) => {
                let last: &mut Point = match grouped_numbers.last_mut() {
                    Some(a) => a,
                    None => {
                        grouped_numbers.push(x);
                        grouped_numbers.last_mut().expect("This has to be an item")
                    }
                };

                if (x.coord.x == (last.coord.x + 1)) && (x.coord.y == last.coord.y) {
                    dbg!(&last);
                    let last_x = match last.value {
                        Value::Number(n) => n,
                        _ => unreachable!("This should never reach"),
                    };
                    let next_x = match x.value {
                        Value::Number(n) => n,
                        _ => unreachable!("This should never reach"),
                    };

                    last.value = Value::Number(last_x * 10 + next_x);
                    last.coord = x.coord;
                }else {
                    grouped_numbers.push(x)
                }
            }
            _ => (),
        });

    dbg!(grouped_numbers);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!("4361", result);
    }
}
