use std::{fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn calc(input: &[char], amount: usize) -> usize {
    let mut rest = input;
    let mut result = String::new();

    for remaining in (1..=amount).rev() {
        let item = rest[0..rest.len() - remaining + 1].iter().max().unwrap();
        result.push(*item);

        let pos = rest.iter().position(|x| x == item).unwrap();

        rest = &rest[pos + 1..];
    }

    result.parse().unwrap()
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines().lines_as_chars();

    let (part1, part2) = content
        .iter()
        .filter(|line| !line.is_empty())
        .fold((0, 0), |(part1, part2), line| {
            (part1 + calc(line, 2), part2 + calc(line, 12))
        });

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
