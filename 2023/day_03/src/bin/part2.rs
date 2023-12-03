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
        .map(|line| {
            let (_game, results) = line.split_once(": ").unwrap();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            results.split("; ").for_each(|set| {
                set.split(", ").for_each(|cube| {
                    let (digit, color) = cube.split_once(" ").unwrap();
                    let digit = digit.parse::<u32>().unwrap();

                    match color {
                        "red" => red = red.max(digit),
                        "green" => green = green.max(digit),
                        "blue" => blue = blue.max(digit),
                        _ => {}
                    }
                })
            });

            red * blue * green

        })
        .sum::<u32>()
        .to_string()
}
