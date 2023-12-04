fn main() {
    let input = include_str!("./input1.txt");
    let output = part_two(input);
    dbg!(output);

    // 5105378 too low
}

// pub fn part_two(input: &str) -> Option<u32> {
//     let (value_map, gear_map) = map_grid(input, 140);
//
//     let indices = gear_map
//         .into_iter()
//         .filter(|(_, c)| c == &'*')
//         .map(|(idx, _)| idx)
//         .collect::<Vec<_>>();
//
//     let mut gear_adjancies: HashMap<usize, Vec<i32>> = HashMap::new();
//
//     value_map.into_iter().for_each(|(coords, v)| {
//         for i in &indices {
//             if coords.contains(*i) {
//                 gear_adjancies
//                     .entry(*i)
//                     .and_modify(|e| e.push(v))
//                     .or_insert(vec![v]);
//             }
//         }
//     });
//
//     Some(
//         gear_adjancies
//             .into_values()
//             .map(|vs| {
//                 if vs.len() == 2 {
//                     vs.into_iter().map(|v| v as u32).product::<u32>()
//                 } else {
//                     0
//                 }
//             })
//             .sum(),
//     )
// }
//
//    use std::{collections::HashMap, ops::Range};
//
// /// Pads the input with `.` above, below, and to the sides
// /// to avoid dealing with edge cases
// fn pre_process(input: &str, width: i32) -> String {
//     let mut s = String::new();
//     for _ in 0..width + 1 {
//         s.push('.');
//     }
//
//     for line in input.lines() {
//         s.push('.');
//         s.push_str(line);
//         s.push('.');
//     }
//
//     for _ in 0..width + 1 {
//         s.push('.');
//     }
//
//     s
// }
//
// #[derive(Debug, Eq, PartialEq, Hash)]
// struct Coords {
//     prev_row: Range<usize>,
//     curr_row: Range<usize>,
//     next_row: Range<usize>,
// }
//
// impl Coords {
//     fn contains(&self, idx: usize) -> bool {
//         self.prev_row.contains(&idx) || self.curr_row.contains(&idx) || self.next_row.contains(&idx)
//     }
// }
//
// fn map_grid(input: &str, width: usize) -> (HashMap<Coords, i32>, HashMap<usize, char>) {
//     let input = pre_process(input, width as i32);
//
//     let mut value_map = HashMap::new();
//     let mut gear_map = HashMap::new();
//
//     let mut value = String::new();
//     for (idx, c) in input.chars().enumerate() {
//         if c.is_ascii_digit() {
//             value.push(c);
//             continue;
//         }
//
//         if c != '.' {
//             gear_map.insert(idx + 1, c);
//         }
//
//         if value.is_empty() {
//             continue;
//         }
//
//         // value is present, we've stepped "past" it and need to record its
//         // coordinates
//
//         let current_row_start = idx - value.len();
//         let current_row_end = idx + 2; // non-inclusive
//
//         let prev_row = (current_row_start - (width + 2))..(current_row_end - width - 2);
//         let next_row = (current_row_start + (width + 2))..(current_row_end + (width + 2));
//
//         value_map.insert(
//             Coords {
//                 prev_row,
//                 curr_row: current_row_start..current_row_end,
//                 next_row,
//             },
//             value.parse::<i32>().unwrap(),
//         );
//         value = String::new();
//     }
//
//     (value_map, gear_map)
// }

#[derive(Debug, Clone, Copy)]
enum Value {
    Empty,
    Star,
    Number(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    coord: Coord,
    value: Value,
}

impl Point {
    fn get_value(&self) -> Option<u32> {
        match self.value {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }
}

fn process(input: &str) -> String {
    let matrix: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().map(move |(x, c)| Point {
                coord: Coord { y, x },
                value: match c {
                    c if c.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("Should not failt"))
                    }
                    '*' => Value::Star,
                    _ => Value::Empty,
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

    let b = matrix
        .iter()
        .filter(|x| matches!(x.value, Value::Star))
        .map(|star| {
            let star_box = get_box(star);

            let star_items: Vec<&Point> = grouped_numbers.iter().filter(|num| {
                let (p1, p2) = get_box_num(num);

                star_box.iter().any(|x| {
                    p1.x <= x.x && x.x <= p2.x && p1.y <= x.y && x.y <= p2.y
                })

            }).collect();

                dbg!(&star_items);
            if star_items.len() == 2 {
                let a: Vec<u32> = star_items
                    .iter()
                    .map(|x| match x.value {
                        Value::Number(n) => n,
                        _ => unreachable!("Should not reach"),
                    })
                    .collect();

                a[0] * a[1]
            } else {
                0
            }
        })
        .sum::<u32>();
    dbg!(b);

    dbg!(b);
    todo!()
}

fn get_box(point: &Point) -> Vec<Coord> {
    let box_shape = vec![
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (1, 1),
    ];

    let a = box_shape
        .iter()
        .map(|(x, y)| Coord {
            x: (point.coord.x as i32 + x) as usize,
            y: (point.coord.y as i32 + y) as usize,
        })
        .collect();

    a
}

fn get_box_num(point: &Point) -> (Coord, Coord) {
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
