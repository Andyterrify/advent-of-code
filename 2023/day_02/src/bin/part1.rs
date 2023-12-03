fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);

    // 2286 is too low
    // 2248 is too low
}

fn process(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (game, results) = line.split_once(": ").unwrap();
            let id = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();

            let game_result = results.split("; ").all(|set| {
                set.split(", ").all(|cube| {
                    let (digit, color) = cube.split_once(" ").unwrap();
                    let digit = digit.parse::<u32>().unwrap();

                    match color {
                        "red" => digit <= 12,
                        "green" => digit <= 13,
                        "blue" => digit <= 14,
                        _ => false,
                    }
                })
            });
            if game_result {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_input() {
    //     let result = process(
    //         "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    //     );
    //     assert_eq!("8", result);
    // }
    #[test]
    fn test_input() {
        let result = process(
            "Game 8: 12 red, 13 green, 14 blue
            Game 8: 12 red",
        );
        assert_eq!("16", result);
    }
}
