use std::{fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn calc(race_time: i128, target_distance: i128) -> usize {
    (1..race_time)
        .map(|current| (race_time - current) * current)
        .filter(|distance| *distance > target_distance)
        .count()
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?
        .get_lines()
        .numbers_in_column(b' ');

    let part1 = content
        .iter()
        .fold(1, |total, race| total * calc(race[0], race[1]));

    println!("Part 1: {}", part1);

    let content2 = read_to_string(filename)?
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .get_lines()
        .numbers_in_column(b':');

    let part2 = calc(content2[0][0], content2[0][1]);
    println!("Part 2: {}", part2);

    Ok(())
}
