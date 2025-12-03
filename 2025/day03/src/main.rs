use std::{fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn calc(input: &[char], amount: usize) -> usize {
    let mut remaining = amount;
    let mut rest = input;
    let mut result = String::new();

    while remaining > 0 {
        let first = rest[0..rest.len() - remaining + 1].iter().max().unwrap();
        result.push(*first);

        let pos = rest.iter().position(|x| x == first).unwrap();

        rest = &rest[pos + 1..];
        remaining -= 1;
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

    let mut part1 = 0;
    let mut part2 = 0;

    for line in content {
        part1 += calc(&line, 2);
        part2 += calc(&line, 12);
    }

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
