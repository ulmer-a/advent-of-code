pub fn main(input: String) -> (u64, u64) {
    let rgb_loaded: [u64; 3] = [12, 13, 14];

    // get rid of the `Game n: ` part, then parse into tuple of the minimal set
    let min_set_of_cubes = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line| min_set_from_line(line));

    // For task 1, filter for games that are actually possible -> minimal set of cubes must be
    // smaller than or equal to the amount loaded. Then sum up game IDs (game_id = index + 1).
    let task1 = min_set_of_cubes
        .clone()
        .enumerate()
        .filter(|(_, rgb)| {
            std::iter::zip(rgb, rgb_loaded)
                .fold(true, |prev, (n, n_loaded)| prev && *n <= n_loaded)
        })
        .map(|(game_id, _)| game_id as u64 + 1)
        .sum();

    // For task 2, just take minimal set of cubes and apply power function according to description.
    let task2 = min_set_of_cubes.map(|[r, g, b]| r * g * b).sum();

    (task1, task2)
}

/// Returns the minimum set of red, green, blue cubes that would be necessary to play a game.
fn min_set_from_line(line: &str) -> [u64; 3] {
    line.split("; ").fold([0; 3], |min_set, round| {
        round
            .split(", ")
            .map(|n_color_combo| n_color_combo.split_once(" ").unwrap())
            .map(|(n_str, color_str)| {
                (
                    /* n: */ n_str.parse::<u64>().unwrap(),
                    /* color_index: */
                    match color_str {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => unreachable!(),
                    },
                )
            })
            .fold(min_set, |mut min_set, (n, color)| {
                if n > min_set[color] {
                    min_set[color] = n;
                }
                min_set
            })
    })
}
