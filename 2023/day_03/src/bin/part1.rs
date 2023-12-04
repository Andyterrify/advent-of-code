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
    Number(u32),
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
    let matrix: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().map(move |(x, c)| Point {
                coord: Coord { y, x },
                value: match c {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("Should not failt"))
                    }
                    _ => Value::Symbol,
                },
            })
        })
        .collect();

    let mut grouped_numbers: Vec<Point> = vec![];

    matrix
        .iter()
        .filter(|x| match x.value {
            Value::Number(_) => true,
            _ => false,
        })
        .for_each(|x| match x.value {
            Value::Number(_) => {
                let last: &mut Point = match grouped_numbers.last_mut() {
                    Some(a) => a,
                    None => {
                        grouped_numbers.push(*x);
                        grouped_numbers.last_mut().expect("This has to be an item")
                    }
                };

                if (x.coord.x == (last.coord.x + 1)) && (x.coord.y == last.coord.y) {
                    let last_x = match last.value {
                        Value::Number(n) => n,
                        _ => unreachable!("This should never reach"),
                    };
                    let next_x = match x.value {
                        Value::Number(n) => n,
                        _ => unreachable!("This should never reach"),
                    };

                    last.value = Value::Number(last_x * 10 + next_x);
                    //
                    //
                    //
                    last.coord = x.coord;
                    //
                    //
                    //
                } else {
                    grouped_numbers.push(*x)
                }
            }
            _ => (),
        });

    grouped_numbers.iter().for_each(|x| {
        let _a = get_box(x);
    });

    grouped_numbers
        .iter()
        .map(|point| {
            let (p1, p2) = get_box(point);

            let has_symbol = matrix
                .iter()
                .filter(|x| {
                    if p1.x <= x.coord.x
                        && x.coord.x <= p2.x
                        && p1.y <= x.coord.y
                        && x.coord.y <= p2.y
                    {
                        match x.value {
                            Value::Symbol => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
                .count()
                > 0;

            if has_symbol {
                match point.value {
                    Value::Number(n) => n,
                    _ => unreachable!("This should not reach"),
                }
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

fn get_box(point: &Point) -> (Coord, Coord) {
    let p1: Coord = Coord {
        x: point.coord.x.saturating_sub(match point.value {
            Value::Number(n) => n.to_string().len(),
            _ => unreachable!("hello"),
        }),
        y: point.coord.y.saturating_sub(1),
    };

    let p2: Coord = Coord {
        x: 140.min(point.coord.x + 1),
        y: 140.min(point.coord.y + 1),
    };

    (p1, p2)
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
